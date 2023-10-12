use mpi::traits::*;
use mpi::topology::SystemCommunicator;

use crate::common::input_size;

mod example;
pub use crate::parallel::example::example;

mod error;
use crate::parallel::error::Error;

pub fn init_program() -> Result<SystemCommunicator, Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::MPI);
    };

    let world = universe.world();
    let size = world.size();
    let rank = world.rank();
    let processor_name = mpi::environment::processor_name()?;

    println!(
        "Hello, world! I am rank {} of {} running on {}.",
        rank, size, processor_name
    );

    Ok(world)
}

pub fn input_size_with_checks() -> Result<i64, std::io::Error> {
    let mut size = -1;

    while size < 0 || size % 2 != 0 {
        size = input_size()?;

        if size < 0 {
            println!("Size of the objects must be greater than 0!");

            continue;
        }

        if size % 2 != 0 {
            println!("Size of objects must be divisible by 2!");

            continue;
        }
    }

     Ok(size)
}

pub fn data_distribution(
    matrix: &mut Vec<Vec<i64>>,
    vector: &mut Vec<i64>,
    size: i64,
    world: &SystemCommunicator,
) {
    let process_rank = world.rank() as i64;
    let root_process = world.process_at_rank(0);

    if process_rank != 0 {
        *vector = vec![0; size as usize];
    }

    root_process.broadcast_into(vector);

    let rows_per_process = size / world.size() as i64;

    // Flatten the matrix into a single Vec<i64>.
    let flat_matrix: Vec<i64> = matrix.iter().flatten().cloned().collect();
    let mut received_matrix: Vec<i64> = vec![0; (rows_per_process * size) as usize];

    if process_rank == 0 {
        root_process.scatter_into_root(
            &flat_matrix,
            &mut received_matrix
        );
    } else {
        root_process.scatter_into(
            &mut received_matrix
        );
    }

    *matrix = received_matrix
        .chunks(size as usize)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<i64>>>();
}
