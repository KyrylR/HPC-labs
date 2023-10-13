use mpi::topology::SystemCommunicator;
use mpi::traits::*;

use crate::parallel::error::Error;
use crate::parallel::{
    data_distribution, input_size_with_checks, process_rows_and_vector_multiplication,
    result_replication,
};
use crate::serial::{dummy_data_init, matrix_vector_product, print_matrix, print_vector, random_data_initialization};

pub fn example() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();

    let mut size = 0;
    if world.rank() == 0 {
        println!("Parallel matrix-vector multiplication program");

        size = input_size_with_checks()?;
    }

    let root_process = world.process_at_rank(0);

    root_process.broadcast_into(&mut size);

    let mut matrix: Vec<Vec<i32>> = vec![];
    let mut vector: Vec<i32> = vec![];

    let mut serial_result = vec![0; size as usize];

    if world.rank() == 0 {
        (matrix, vector) = random_data_initialization::<i32>(size as usize);

        serial_result = matrix_vector_product::<i32, i64>(&matrix, &vector);
    }

    let mut flat_matrix: Vec<i32> = matrix.iter().flatten().cloned().collect();

    let t_start = mpi::time();
    data_distribution(&mut flat_matrix, &mut vector, size, &world);

    let mul_res = process_rows_and_vector_multiplication(&flat_matrix, &vector);

    let mut global_res = vec![0; size as usize];

    result_replication(&mul_res, &mut global_res, size, &world);
    let t_end = mpi::time();
    let duration = t_end - t_start;

    if world.rank() == 0 {
        test_result(&serial_result, &global_res, size);

        println!("Time elapsed in parallel matrix_vector_product() is: {:?}", duration);

        println!(
            "the clock has a resolution of {} seconds",
            mpi::time_resolution()
        );
    }

    Ok(())
}

fn test_distribution(matrix: &[Vec<i32>], vector: &[i32], world: &SystemCommunicator) {
    for i in 0..world.size() {
        if world.rank() == i {
            println!("\nProcRank = {}", world.rank());
            println!("Matrix Stripe:");
            print_matrix(matrix);
            println!("Vector:");
            print_vector(vector);
        }

        world.barrier();
    }
}

fn test_partial_results(p_proc_result: &[i64], world: &SystemCommunicator) {
    for i in 0..world.size() {
        if world.rank() == i {
            println!("\nProcRank = {}", world.rank());
            println!("Part of result vector:");
            print_vector(p_proc_result);
        }

        world.barrier();
    }
}

fn test_result(serial_result: &[i64], p_result: &[i64], size: i32) {
    let mut equal = 0;

    for i in 0..size {
        if p_result[i as usize] != serial_result[i as usize] {
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
