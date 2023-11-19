use lab1::common::input_size;
use lab1::parallel::{get_dispels, Error};
use mpi::datatype::{Partition, PartitionMut};
use mpi::point_to_point::send_receive;
use mpi::topology::{Communicator, SimpleCommunicator};
use mpi::traits::*;
use std::time::Instant;

fn main() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();

    let world_size = world.size() as usize;
    let world_rank = world.rank() as usize;

    let mut matrix_size: usize = 0;
    let mut epsilon = 0.0;

    if world_rank == 0 {
        println!("Parallel Gauss - Seidel algorithm");

        matrix_size = input_size()? as usize;
        epsilon = input_size()? as f64 / 1000.0;
    }

    let root_process = world.process_at_rank(0);

    root_process.broadcast_into(&mut matrix_size);
    root_process.broadcast_into(&mut epsilon);

    let row_num = calculate_row_num(matrix_size, world_size, world_rank);
    let mut p_proc_rows = vec![0.0; row_num * matrix_size];
    let mut p_matrix = vec![0.0; matrix_size * matrix_size];

    // Data initialization on process 0
    if world_rank == 0 {
        dummy_data_initialization(&mut p_matrix, matrix_size);
    }

    // Distribute data among processes
    data_distribution(&mut p_matrix, &mut p_proc_rows, matrix_size, &world);

    let start = Instant::now();

    // Parallel Gauss-Seidel method
    let iterations =
        parallel_result_calculation(&mut p_proc_rows, matrix_size, row_num, epsilon, &world);

    // Gather results
    result_collection(&world, &mut p_matrix, &p_proc_rows, matrix_size);

    let duration = start.elapsed();

    // Output results
    if world_rank == 0 {
        println!("Iterations: {}", iterations);
        println!("Duration: {:?}", duration);
        print_matrix(&p_matrix, matrix_size);
    }

    Ok(())
}

fn calculate_row_num(matrix_size: usize, size: usize, _rank: usize) -> usize {
    // Calculation of number of rows for each process
    (matrix_size - 2) / size + 2
}

fn dummy_data_initialization(p_matrix: &mut [f64], size: usize) {
    // Initialize matrix with dummy data
    for i in 0..size {
        for j in 0..size {
            p_matrix[i * size + j] = if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                100.0
            } else {
                0.0
            };
        }
    }
}

fn data_distribution(
    p_matrix: &mut Vec<f64>,
    p_proc_rows: &mut [f64],
    size: usize,
    world: &SimpleCommunicator,
) {
    let root_process = world.process_at_rank(0);
    if world.rank() == 0 {
        let counts: Vec<i32> = (0..world.size())
            .map(|rank| calculate_row_num(size, world.size() as usize, rank as usize) as i32)
            .collect();
        let dispels = get_dispels(&counts);

        let partition: &Partition<Vec<f64>, Vec<i32>, &[i32]> =
            &Partition::new(p_matrix, counts, &dispels[..]);

        root_process.scatter_varcount_into_root(partition, p_proc_rows);
    } else {
        root_process.scatter_varcount_into(p_proc_rows);
    }
}

fn parallel_result_calculation(
    p_proc_rows: &mut [f64],
    size: usize,
    row_num: usize,
    eps: f64,
    world: &SimpleCommunicator,
) -> i32 {
    let mut iterations = 0;
    let mut delta: f64 = 0.0;

    loop {
        iterations += 1;
        exchange_data(world, p_proc_rows);
        let proc_delta = iteration_calculation(p_proc_rows, size, row_num);

        world.all_reduce_into(
            &proc_delta,
            &mut delta,
            &mpi::collective::SystemOperation::max(),
        );

        if delta <= eps {
            break;
        }
    }

    iterations
}

fn exchange_data(world: &SimpleCommunicator, p_proc_rows: &mut [f64]) {
    let next_proc_num = if world.rank() == world.size() - 1 {
        None
    } else {
        Some(world.rank() + 1)
    };
    let prev_proc_num = if world.rank() == 0 {
        None
    } else {
        Some(world.rank() - 1)
    };

    if let Some(next_process_rank) = next_proc_num {
        let next_process = world.process_at_rank(next_process_rank);

        if let Some(prev_process_rank) = prev_proc_num {
            let prev_process = world.process_at_rank(prev_process_rank);
            unsafe {
                let new_ptr: i32;
                // Assuming send_receive returns a raw pointer to i32
                (new_ptr, _) =
                    send_receive(&(p_proc_rows.as_ptr() as i32), &next_process, &prev_process);

                // Assuming the length of the slice remains the same,
                // but this needs to be known or managed carefully.
                let length = p_proc_rows.len();

                // Assuming new_ptr is a raw pointer (*mut i32) and length is the length of the slice
                let new_slice = std::slice::from_raw_parts_mut(&mut (new_ptr as f64), length);

                // Reassign p_proc_rows to the new slice
                p_proc_rows.clone_from_slice(new_slice);
            }
        }
    }

    if let Some(next_process_rank) = prev_proc_num {
        let next_process = world.process_at_rank(next_process_rank);

        if let Some(prev_process_rank) = next_proc_num {
            let prev_process = world.process_at_rank(prev_process_rank);
            // (*p_proc_rows, _) = send_receive(&(p_proc_rows.as_ptr() as i32), &next_process, &prev_process);
            unsafe {
                let new_ptr: i32;
                // Assuming send_receive returns a raw pointer to i32
                (new_ptr, _) =
                    send_receive(&(p_proc_rows.as_ptr() as i32), &prev_process, &next_process);

                // Assuming the length of the slice remains the same,
                // but this needs to be known or managed carefully.
                let length = p_proc_rows.len();

                // Assuming new_ptr is a raw pointer (*mut i32) and length is the length of the slice
                let new_slice = std::slice::from_raw_parts_mut(&mut (new_ptr as f64), length);

                // Reassign p_proc_rows to the new slice
                p_proc_rows.clone_from_slice(new_slice);
            }
        }
    }
}

fn iteration_calculation(p_proc_rows: &mut [f64], size: usize, row_num: usize) -> f64 {
    let mut dmax = 0.0;
    for i in 1..row_num - 1 {
        for j in 1..size - 1 {
            let temp = p_proc_rows[i * size + j];
            p_proc_rows[i * size + j] = 0.25
                * (p_proc_rows[i * size + (j + 1)]
                    + p_proc_rows[i * size + (j - 1)]
                    + p_proc_rows[(i + 1) * size + j]
                    + p_proc_rows[(i - 1) * size + j]);
            let dm = (p_proc_rows[i * size + j] - temp).abs();
            if dmax < dm {
                dmax = dm;
            }
        }
    }
    dmax
}

fn result_collection(
    world: &SimpleCommunicator,
    p_matrix: &mut [f64],
    p_proc_rows: &[f64],
    size: usize,
) {
    let counts: Vec<i32> = (0..world.size())
        .map(|rank| (calculate_row_num(size, world.size() as usize, rank as usize) * size) as i32)
        .collect();
    let dispels: Vec<i32> = get_dispels(&counts);

    {
        let mut partition = PartitionMut::new(p_matrix, counts, &dispels[..]);
        world.all_gather_varcount_into(p_proc_rows, &mut partition);
    }
}

fn print_matrix(p_matrix: &[f64], size: usize) {
    // Print matrix
    for i in 0..size {
        for j in 0..size {
            print!("{:7.4} ", p_matrix[i * size + j]);
        }
        println!();
    }
}

pub fn copy_matrix(p_matrix: &[f64]) -> Vec<f64> {
    p_matrix.to_vec()
}
