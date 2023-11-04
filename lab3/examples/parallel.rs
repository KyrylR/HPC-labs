use mpi::collective::SystemOperation;
use mpi::datatype::{Partition, PartitionMut};
use mpi::topology::SimpleCommunicator;
use mpi::traits::*;

use lab1::parallel::Error;

use lab3::parallel::{MatrixVectorData, MaxValueProcRank};

fn data_distribution(
    data: &mut MatrixVectorData,
    world: &SimpleCommunicator, // Assuming we have an MPI environment
) {
    let root_process = world.process_at_rank(0);

    if world.rank() == 0 {
        let counts: Vec<i32> = (0..world.size())
            .map(|rank| compute_rows_for_rank(rank, data.size, world.size()))
            .collect();
        let dispels = get_dispels(&counts);

        let partition_matrix = Partition::new(&data.p_matrix, counts.clone(), dispels.clone());
        root_process.scatter_varcount_into_root(&partition_matrix, &mut data.p_proc_rows);

        let partition_vector = Partition::new(&data.p_vector, counts, dispels);
        root_process.scatter_varcount_into_root(&partition_vector, &mut data.p_proc_vector);
    } else {
        root_process.scatter_varcount_into(&mut data.p_proc_rows);
        root_process.scatter_varcount_into(&mut data.p_proc_vector);
    }
}

pub fn compute_rows_for_rank(rank: i32, size: usize, process_count: i32) -> i32 {
    let mut rows_per_process = size as i32 / process_count;
    if rank < size as i32 % process_count {
        rows_per_process += 1;
    }
    rows_per_process
}

pub fn get_dispels(counts: &[i32]) -> Vec<i32> {
    counts
        .iter()
        .scan(0, |acc, &x| {
            let tmp = *acc;
            *acc += x;
            Some(tmp)
        })
        .collect()
}

// Function for gathering the result vector
fn result_collection(data: &mut MatrixVectorData, world: &SimpleCommunicator) {
    let process_count = world.size();

    let counts: Vec<i32> = (0..world.size())
        .map(|rank| compute_rows_for_rank(rank, data.size, process_count))
        .collect();
    let dispels: Vec<i32> = get_dispels(&counts);

    {
        let mut partition = PartitionMut::new(&mut data.p_result, counts, &dispels[..]);
        world.all_gather_varcount_into(&data.p_proc_result, &mut partition);
    }
}

// Function for printing the matrix
fn print_matrix(matrix: &[f64], row_count: usize, col_count: usize) {
    for i in 0..row_count {
        for j in 0..col_count {
            print!("{:7.4} ", matrix[i * col_count + j]);
        }
        println!();
    }
}

// Function for printing the vector
fn print_vector(vector: &[f64], size: usize) {
    for i in 0..size {
        print!("{:7.4} ", vector[i]);
    }
    println!();
}

fn parallel_eliminate_columns(
    data: &mut MatrixVectorData,
    p_pivot_row: &[f64],
    iter: usize,
    p_proc_pivot_iter: &mut [f64],
) {
    let mut pivot_factor: f64;

    for i in 0..data.row_num {
        if p_proc_pivot_iter[i] == -1.0 {
            pivot_factor = data.p_proc_rows[i * data.size + iter] / p_pivot_row[iter];
            for j in iter..data.size {
                data.p_proc_rows[i * data.size + j] -= pivot_factor * p_pivot_row[j];
            }
            data.p_proc_vector[i] -= pivot_factor * p_pivot_row[data.size];
        }
    }
}

fn parallel_gaussian_elimination(
    data: &mut MatrixVectorData,
    p_parallel_pivot_pos: &mut [f64],
    world: &SimpleCommunicator,
) {
    let mut max_value; // Value of the pivot element of the process
    let mut pivot_pos; // Position of the pivot row in the process stripe
    let mut p_proc_pivot_iter = vec![-1.0; data.row_num]; // Tracks pivot iterations

    let mut p_pivot_row = vec![0.0; data.size + 1];

    // The iterations of the Gaussian elimination
    for i in 0..data.size {
        max_value = f64::MIN; // Start with the smallest possible value
        pivot_pos = 0; // Reset the pivot position

        // Calculating the local pivot row
        for j in 0..data.row_num {
            if p_proc_pivot_iter[j] == -1.0 {
                let value = data.p_proc_rows[j * data.size + i].abs();
                if max_value < value {
                    max_value = value;
                    pivot_pos = j;
                }
            }
        }

        // Perform the row reduction based on the pivot row
        let proc_pivot: MaxValueProcRank = MaxValueProcRank {
            max_value,
            proc_rank: world.rank(),
        };
        let mut pivot: MaxValueProcRank = MaxValueProcRank {
            max_value: 0.0,
            proc_rank: world.rank(),
        };

        world.all_reduce_into(&proc_pivot, &mut pivot, SystemOperation::max());

        if world.rank() == pivot.proc_rank {
            p_proc_pivot_iter[pivot_pos] = i as f64;
            p_parallel_pivot_pos[i] = compute_rows_for_rank(world.rank(), data.size, world.size())
                as f64
                + pivot_pos as f64;
        }

        let root_process = world.process_at_rank(pivot.proc_rank);
        root_process.broadcast_into(&mut p_parallel_pivot_pos[i]);

        if world.rank() == pivot.proc_rank {
            for j in 0..data.size {
                p_pivot_row[j] = data.p_proc_rows[pivot_pos * data.size + j];
            }
            p_pivot_row[data.size] = data.p_proc_vector[pivot_pos];
        }
        root_process.broadcast_into(&mut p_pivot_row);
        parallel_eliminate_columns(data, &p_pivot_row, i, &mut p_proc_pivot_iter);
    }
}

fn find_back_pivot_row(
    row_index: usize,
    proc_num: usize,
    p_proc_ind: &[i32], // This would be a slice in Rust
    iter_proc_rank: &mut usize,
    iter_pivot_pos: &mut usize,
) {
    for i in 0..proc_num - 1 {
        if p_proc_ind[i] <= row_index as i32 && row_index < p_proc_ind[i + 1] as usize {
            *iter_proc_rank = i;
            break; // Once found, we break out of the loop in Rust as well.
        }
    }

    if row_index >= p_proc_ind[proc_num - 1] as usize {
        *iter_proc_rank = proc_num - 1;
    }

    *iter_pivot_pos = row_index - p_proc_ind[*iter_proc_rank] as usize;
}

fn parallel_back_substitution(
    data: &mut MatrixVectorData,
    _p_parallel_pivot_pos: &[f64],
    p_proc_ind: &[i32],
    p_proc_pivot_iter: &[i32],
    world: &SimpleCommunicator,
) {
    let proc_rank = world.rank() as usize;
    let mut iter_proc_rank: usize;
    let mut iter_pivot_pos: usize;
    let mut iter_result: f64 = 0.0;

    // The iterations of the back substitution
    for i in (0..data.size).rev() {
        // Calculating the rank of the process, which holds the pivot row
        iter_proc_rank = 0; // Initialize to 0 or some appropriate value
        iter_pivot_pos = 0; // Initialize to 0 or some appropriate value
        find_back_pivot_row(
            i,
            world.rank() as usize,
            p_proc_ind,
            &mut iter_proc_rank,
            &mut iter_pivot_pos,
        );

        // Calculating the unknown
        if proc_rank == iter_proc_rank {
            iter_result = data.p_proc_vector[iter_pivot_pos]
                / data.p_proc_rows[iter_pivot_pos * data.size + i];
            data.p_proc_result[iter_pivot_pos] = iter_result;
        }

        // Broadcasting the value of the current unknown
        // Assuming we have the MPI environment set up properly
        let root_process = world.process_at_rank(iter_proc_rank as i32);
        root_process.broadcast_into(&mut iter_result);

        // Updating the values of the vector
        for j in 0..data.row_num {
            if p_proc_pivot_iter[j] < i as i32 {
                let val = data.p_proc_rows[j * data.size + i] * iter_result;
                data.p_proc_vector[j] -= val;
            }
        }
    }
}

fn test_result(
    p_matrix: &[f64],
    p_vector: &[f64],
    p_result: &[f64],
    p_parallel_pivot_pos: &[f64],
    size: usize,
    proc_rank: usize, // Assuming we have a way to get the process rank
) {
    let mut p_right_part_vector = vec![0.0; size];
    let mut equal = true;
    let accuracy = 1e-6; // Comparison accuracy

    if proc_rank == 0 {
        for i in 0..size {
            for j in 0..size {
                p_right_part_vector[i] +=
                    p_matrix[i * size + j] * p_result[p_parallel_pivot_pos[j].abs() as usize];
            }
        }

        for i in 0..size {
            if (p_right_part_vector[i] - p_vector[i]).abs() > accuracy {
                equal = false;
                break;
            }
        }

        if equal {
            println!("The result of the parallel Gauss algorithm is correct.");
        } else {
            println!("The result of the parallel Gauss algorithm is NOT correct. Check your code.");
        }
    }
}

// Main execution function
fn parallel_result_calculation(
    data: &mut MatrixVectorData,
    world: &SimpleCommunicator,
) -> Vec<f64> {
    let mut p_parallel_pivot_pos = vec![0.0; data.size];
    let p_proc_pivot_iter = vec![-1; data.row_num];
    let mut p_proc_ind = vec![0; world.size() as usize];

    for i in 0..world.size() {
        p_proc_ind[i as usize] = compute_rows_for_rank(i, data.size, world.size());
    }

    parallel_gaussian_elimination(data, &mut p_parallel_pivot_pos, world);
    parallel_back_substitution(
        data,
        &p_parallel_pivot_pos,
        &p_proc_ind,
        &p_proc_pivot_iter,
        world,
    );

    p_parallel_pivot_pos
}

fn main() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    if rank == 0 {
        println!("Parallel Gauss algorithm for solving linear systems");
    }

    let mut matrix_vector_data: MatrixVectorData = MatrixVectorData::new(&world, rank, size);

    if rank == 0 {
        matrix_vector_data.dummy_data_initialization();
        println!("Matrix:");
        print_matrix(
            &matrix_vector_data.p_matrix,
            matrix_vector_data.size,
            matrix_vector_data.size,
        );
        println!("Vector:");
        print_vector(&matrix_vector_data.p_vector, matrix_vector_data.size);
    }

    let t_start = mpi::time();
    data_distribution(&mut matrix_vector_data, &world);

    let p_parallel_pivot_pos = parallel_result_calculation(&mut matrix_vector_data, &world);

    result_collection(&mut matrix_vector_data, &world);

    test_result(
        &matrix_vector_data.p_matrix,
        &matrix_vector_data.p_vector,
        &matrix_vector_data.p_result,
        &p_parallel_pivot_pos,
        matrix_vector_data.size,
        rank as usize,
    );

    if rank == 0 {
        println!(
            "Time elapsed in parallel matrix_vector_product() is: {:?}",
            mpi::time() - t_start
        );
    }

    Ok(())
}

pub fn test_distribution(data: &mut MatrixVectorData, world: &SimpleCommunicator) {
    let proc_rank = world.rank();
    let proc_num = world.size();

    if proc_rank == 0 {
        println!("Initial Matrix:");
        print_matrix(data.p_matrix.as_slice(), data.size, data.size);
        println!("Initial Vector:");
        print_vector(data.p_vector.as_slice(), data.size);
    }

    world.barrier(); // Synchronize before starting

    for i in 0..proc_num {
        if proc_rank == i {
            println!("\nProcRank = {}", proc_rank);
            println!("Matrix Stripe:");
            print_matrix(data.p_proc_rows.as_slice(), data.row_num, data.size);
            println!("Vector:");
            print_vector(data.p_proc_vector.as_slice(), data.row_num);
        }
        world.barrier(); // Synchronize after each process prints
    }
}
