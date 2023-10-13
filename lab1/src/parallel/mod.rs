use mpi::datatype::{Partition, PartitionMut};
use mpi::topology::SystemCommunicator;
use mpi::traits::*;
use mpi::Count;

use crate::common::input_size;

mod example;
pub use crate::parallel::example::example;

mod error;
use crate::parallel::error::Error;

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

pub fn input_size_with_checks() -> Result<i32, std::io::Error> {
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

fn compute_rows_for_rank(rank: i32, size: i32, process_count: i32, bigger_count: i32) -> i32 {
    let mut rows_per_process = size / process_count;
    if rank < bigger_count {
        rows_per_process += 1;
    }
    rows_per_process
}

fn get_dispels_box(counts: &[i32]) -> Box<[i32]> {
    counts.iter().scan(0, |acc, &x| {
        let tmp = *acc;
        *acc += x;
        Some(tmp)
    }).collect::<Vec<i32>>().into_boxed_slice()
}


pub fn data_distribution(
    flatten_matrix: &mut Vec<i32>,
    vector: &mut Vec<i32>,
    size: i32,
    world: &SystemCommunicator,
) {
    let process_rank = world.rank();
    let process_count = world.size();
    let bigger_count = size % process_count;

    let root_process = world.process_at_rank(0);

    if process_rank != 0 {
        *vector = vec![0; size as usize];
    }

    root_process.broadcast_into(vector);

    let mut rows_per_process = size / world.size();

    if process_rank < bigger_count {
        rows_per_process += 1;
    }

    let mut received_matrix: Vec<i32> = vec![0; (rows_per_process * size) as usize];

    if process_rank == 0 {
        let counts: Vec<i32> = (0..world.size())
            .map(|rank| {
                let rows_per_process = size / world.size();

                if rank < bigger_count {
                    return (rows_per_process + 1) * size;
                }

                rows_per_process * size
            })
            .collect();
        let dispels: Vec<i32> = get_dispels(&counts);

        let partition = Partition::new(&flatten_matrix[..], counts, &dispels[..]);
        root_process.scatter_varcount_into_root(&partition, &mut received_matrix[..]);
    } else {
        root_process.scatter_varcount_into(&mut received_matrix);
    }

    *flatten_matrix = received_matrix;
}

pub fn process_rows_and_vector_multiplication(
    flatten_matrix_stripe: &[i32],
    vector: &[i32],
) -> Vec<i64> {
    flatten_matrix_stripe
        .chunks(vector.len())
        .map(|row| {
            row.iter()
                .zip(vector)
                .map(|(&m, &v)| (m as i64) * (v as i64))
                .sum()
        })
        .collect()
}

pub fn result_replication(
    p_proc_result: &[i64],
    p_result: &mut Vec<i64>,
    size: i32,
    world: &SystemCommunicator,
) {
    let process_count = world.size();
    let bigger_count = size % process_count;
    let rows_per_process = size / world.size();

    let counts: Vec<Count> = (0..world.size())
        .map(|rank| {
            if rank < bigger_count {
                return rows_per_process + 1;
            }

            rows_per_process
        })
        .collect();
    let dispels: Vec<Count> = get_dispels(&counts);

    {
        let mut partition = PartitionMut::new(p_result, counts, &dispels[..]);
        world.all_gather_varcount_into(p_proc_result, &mut partition);
    }
}

fn get_dispels(counts: &[Count]) -> Vec<Count> {
    counts
        .iter()
        .scan(0, |acc, &x| {
            let tmp = *acc;
            *acc += x;
            Some(tmp)
        })
        .collect()
}
