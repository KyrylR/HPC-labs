use mpi::point_to_point::send_receive_replace_into;
use mpi::topology::{CartesianCommunicator, SystemCommunicator};
use mpi::traits::*;
use mpi::Count;

use lab1::parallel::Error;

use lab2::parallel::{create_grid_communicator, input_size_with_checks};
use lab2::serial::{matrix_matrix_product, random_data_initialization};

pub fn main() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();

    let grid_size = (world.size() as f64).sqrt() as u64;

    if grid_size * grid_size != world.size() as u64 {
        println!("The number of processes must be a perfect square.");

        return Ok(());
    }

    let mut size = 0_u64;
    if world.rank() == 0 {
        println!("Parallel matrix-matrix multiplication program");

        size = input_size_with_checks(grid_size, world.size() as u32)?;
    }

    let root_process = world.process_at_rank(0);
    root_process.broadcast_into(&mut size);

    let block_size = size / grid_size;

    let mut a_block = vec![0_u64; (block_size * block_size) as usize];
    let mut b_block = vec![0_u64; (block_size * block_size) as usize];
    let mut c_block = vec![0_u64; (block_size * block_size) as usize];

    let mut a_matrix: Vec<u64> = vec![0_u64];
    let mut b_matrix: Vec<u64> = vec![0_u64];
    let c_matrix: &mut Vec<u64> = &mut vec![0_u64];

    if world.rank() == 0 {
        *c_matrix = vec![0_u64; (size * size) as usize];

        (a_matrix, b_matrix) = random_data_initialization(size);
    }

    let (coords, row_comm, col_comm) = create_grid_communicator(grid_size, &world);

    let t_start = mpi::time();
    // Data distribution
    checkerboard_matrix_scatter(
        &a_matrix,
        &mut a_block,
        size,
        block_size,
        &coords,
        &world,
        &row_comm,
        &col_comm,
    );
    checkerboard_matrix_scatter(
        &b_matrix,
        &mut b_block,
        size,
        block_size,
        &coords,
        &world,
        &row_comm,
        &col_comm,
    );

    // Parallel result calculation
    parallel_result_calculation(
        &mut a_block,
        &mut b_block,
        &mut c_block,
        block_size,
        grid_size,
        &coords,
        &row_comm,
        &col_comm,
    );

    // Gathering the result matrix
    result_collection(
        c_matrix, &c_block, size, block_size, &coords, &world, &row_comm, &col_comm,
    );

    if world.rank() == 0 {
        println!(
            "Time elapsed in parallel matrix_matrix_product() is: {:?}",
            mpi::time() - t_start
        );
        test_result(&a_matrix, &b_matrix, c_matrix, size);
    }

    Ok(())
}

fn parallel_result_calculation(
    p_matrix_a_block: &mut [u64],
    p_b_block: &mut Vec<u64>,
    p_c_block: &mut [u64],
    block_size: u64,
    grid_size: u64,
    coords: &[Count],
    row_comm: &CartesianCommunicator,
    col_comm: &CartesianCommunicator,
) {
    let mut p_a_block = vec![0_u64; (block_size * block_size) as usize];
    // Function for parallel execution of the Fox method
    for iter in 0..grid_size {
        // Sending blocks of matrix A to the process grid rows
        a_block_communication(
            iter,
            &mut p_a_block,
            p_matrix_a_block,
            block_size,
            grid_size,
            coords,
            row_comm,
        );

        // Block multiplication
        block_multiplication(&p_a_block, p_b_block, p_c_block, block_size);

        // Cyclic shift of blocks of matrix B in process grid columns
        b_block_communication(p_b_block, coords, grid_size, col_comm);
    }
}

fn a_block_communication(
    iter: u64,
    p_a_block: &mut [u64],
    p_matrix_a_block: &mut [u64],
    block_size: u64,
    grid_size: u64,
    cords: &[Count],
    row_comm: &CartesianCommunicator,
) {
    // Defining the leading process of the process grid row
    let pivot = (iter + cords[0] as u64) % grid_size;

    // Copying the transmitted block in a separate memory buffer
    if cords[1] as u64 == pivot {
        for i in 0..block_size * block_size {
            p_a_block[i as usize] = p_matrix_a_block[i as usize];
        }
    }

    // Block broadcasting
    row_comm
        .process_at_rank(pivot as i32)
        .broadcast_into(&mut p_a_block[..]);
}

fn b_block_communication(
    p_b_block: &mut Vec<u64>,
    cords: &[Count],
    grid_size: u64,
    col_comm: &CartesianCommunicator,
) {
    let mut next_process = cords[0] + 1;
    if cords[0] == grid_size as i32 - 1 {
        next_process = 0;
    }

    let mut prev_process = cords[0] - 1;
    if cords[0] == 0 {
        prev_process = grid_size as i32 - 1;
    }

    send_receive_replace_into(
        p_b_block,
        &col_comm.process_at_rank(prev_process),
        &col_comm.process_at_rank(next_process),
    );
}

fn block_multiplication(
    p_a_block: &[u64],
    p_b_block: &[u64],
    p_c_block: &mut [u64],
    block_size: u64,
) {
    for i in 0..block_size {
        for j in 0..block_size {
            for k in 0..block_size {
                p_c_block[(i * block_size + j) as usize] += p_a_block
                    [(i * block_size + k) as usize]
                    * p_b_block[(k * block_size + j) as usize];
            }
        }
    }
}

fn result_collection(
    p_c_matrix: &mut Vec<u64>,
    p_c_block: &[u64],
    size: u64,
    block_size: u64,
    coords: &[Count],
    world: &SystemCommunicator,
    row_comm: &CartesianCommunicator,
    col_comm: &CartesianCommunicator,
) {
    let mut p_result_row = vec![0_u64; (size * block_size) as usize];

    let row_root_process = row_comm.process_at_rank(0);

    for i in 0..block_size {
        if coords[1] == 0 {
            row_root_process.gather_into_root(
                &p_c_block[(i * block_size) as usize..((i + 1) * block_size) as usize],
                &mut p_result_row[(i * size) as usize..((i + 1) * size) as usize],
            );
        } else {
            row_root_process.gather_into(
                &p_c_block[(i * block_size) as usize..((i + 1) * block_size) as usize],
            );
        }
    }

    let col_root_process = col_comm.process_at_rank(0);

    if coords[1] == 0 {
        if world.rank() == 0 {
            col_root_process.gather_into_root(&p_result_row, p_c_matrix);
        } else {
            col_root_process.gather_into(&p_result_row);
        }
    }
}

fn test_result(p_a_matrix: &[u64], p_b_matrix: &[u64], p_c_matrix: &[u64], size: u64) {
    let serial_result = matrix_matrix_product(p_a_matrix, p_b_matrix, size as usize);

    let mut equal = 0;

    for i in 0..size {
        if p_c_matrix[i as usize] as u128 != serial_result[i as usize] {
            equal = 1;
        }
    }

    if equal == 1 {
        println!(
            "The results of serial and parallel algorithms are NOT identical. Check your code."
        );
    } else {
        println!("The results of serial and parallel algorithms are identical.");
    }
}

fn checkerboard_matrix_scatter(
    p_matrix: &[u64],
    p_matrix_block: &mut [u64],
    size: u64,
    block_size: u64,
    coords: &[Count],
    world: &SystemCommunicator,
    row_comm: &CartesianCommunicator,
    col_comm: &CartesianCommunicator,
) {
    let mut p_matrix_row = vec![0_u64; (block_size * size) as usize];

    let col_root_process = col_comm.process_at_rank(0);

    if coords[1] == 0 {
        if world.rank() == 0 {
            col_root_process.scatter_into_root(p_matrix, &mut p_matrix_row);
        } else {
            col_root_process.scatter_into(&mut p_matrix_row);
        }
    }

    let row_root_process = row_comm.process_at_rank(0);

    for i in 0..block_size {
        if coords[1] == 0 {
            row_root_process.scatter_into_root(
                &p_matrix_row[(i * size) as usize..((i + 1) * size) as usize],
                &mut p_matrix_block[(i * block_size) as usize..((i + 1) * block_size) as usize],
            );
        } else {
            row_root_process.scatter_into(
                &mut p_matrix_block[(i * block_size) as usize..((i + 1) * block_size) as usize],
            );
        }
    }
}

pub fn test_blocks(p_block: &[u64], block_size: u64, str: &str, world: &SystemCommunicator) {
    world.barrier();

    if world.rank() == 0 {
        println!("{}", str);
    }

    for i in 0..world.size() {
        if world.rank() == i {
            println!("ProcRank = {}", world.rank());
            lab1::serial::print_matrix(p_block, block_size as usize);
        }

        world.barrier();
    }
}
