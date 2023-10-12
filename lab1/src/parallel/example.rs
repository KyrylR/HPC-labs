use mpi::traits::*;
use mpi::topology::SystemCommunicator;

use crate::parallel::error::Error;
use crate::parallel::{data_distribution, input_size_with_checks};
use crate::serial::{print_matrix, print_vector, random_data_initialization};

pub fn example() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::MPI);
    };

    let world = universe.world();

    let mut size = 0;
    if world.rank() == 0 {
        println!("Parallel matrix-vector multiplication program");

        size = input_size_with_checks()?;
    }

    let root_process = world.process_at_rank(0);

    root_process.broadcast_into(&mut size);

    // Determine the number of matrix rows stored on each process
    // let rows_per_process = size / world.size() as i64;

    let mut matrix: Vec<Vec<i64>> = vec![];
    let mut vector: Vec<i64> = vec![];

    if world.rank() == 0 {
        (matrix, vector) = random_data_initialization(size);
    }

    if world.rank() == 0 {
        println!("Initial Matrix:");
        print_matrix(&matrix);

        println!("Initial Vector:");
        print_vector(&vector);
    }

    data_distribution(&mut matrix, &mut vector, size, &world);

    test_distribution(&matrix, &vector, &world);

    Ok(())
}

fn test_distribution(matrix: &Vec<Vec<i64>>, vector: &Vec<i64>, world: &SystemCommunicator) {
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