use mpi::datatype::PartitionMut;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;

use crate::common::input_size;
pub use crate::parallel::error::Error;

mod error;

pub fn init_program() -> Result<SystemCommunicator, Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
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

pub fn input_size_with_checks(process_number: u32) -> Result<u64, std::io::Error> {
    let mut size = 0_u64;

    while size == 0 {
        size = input_size()?;

        if size == 0 {
            println!("Size of the objects must be greater than 0!");

            continue;
        }

        if size < process_number as u64 {
            println!("Size of the matrix must be greater than the number of processes!");

            continue;
        }
    }

    Ok(size)
}

pub fn process_rows_and_vector_multiplication(
    flatten_matrix_stripe: &[u64],
    vector: &[u64],
) -> Vec<u64> {
    flatten_matrix_stripe
        .chunks(vector.len())
        .map(|row| row.iter().zip(vector).map(|(m, v)| m * v).sum())
        .collect()
}

pub fn result_replication(
    p_proc_result: &[u64],
    p_result: &mut Vec<u64>,
    size: u64,
    world: &SystemCommunicator,
) {
    let process_count = world.size();
    let bigger_count = size as i32 % process_count;

    let counts: Vec<i32> = (0..world.size())
        .map(|rank| compute_rows_for_rank(rank, size, process_count, bigger_count))
        .collect();
    let dispels: Vec<i32> = get_dispels(&counts);

    {
        let mut partition = PartitionMut::new(p_result, counts, &dispels[..]);
        world.all_gather_varcount_into(p_proc_result, &mut partition);
    }
}

pub fn compute_rows_for_rank(rank: i32, size: u64, process_count: i32, bigger_count: i32) -> i32 {
    let mut rows_per_process = size as i32 / process_count;
    if rank < bigger_count {
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
