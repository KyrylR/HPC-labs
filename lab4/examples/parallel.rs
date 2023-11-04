use std::time::Instant;

use mpi::point_to_point::{Destination, Source};
use mpi::topology::SimpleCommunicator;
use mpi::traits::*;
use mpi::Rank;
use rand::Rng;

use lab1::common::input_size;
use lab1::parallel::Error;

fn main() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();

    let size = world.size();
    let rank = world.rank();

    if rank == 0 {
        println!("Parallel bubble sort program");
    }

    let mut data_size = if rank == 0 { input_size()? } else { 0 };

    // Broadcasting the data size
    let root_process = world.process_at_rank(0);
    root_process.broadcast_into(&mut data_size);

    let data = if rank == 0 {
        let mut rng = rand::thread_rng();
        (0..data_size).map(|_| rng.gen_range(0.0..1000.0)).collect()
    } else {
        vec![0.0; data_size as usize]
    };

    let block_size = data_size / size as u64;
    let mut proc_data = vec![0.0; block_size as usize];

    if rank == 0 {
        println!("Sorting {} data items", data_size);
    }

    let start = Instant::now();

    // Distributing the initial data between processes
    if rank == 0 {
        root_process.scatter_into_root(&data, &mut proc_data);
    } else {
        root_process.scatter_into(&mut proc_data);
    }

    // Parallel bubble sort
    parallel_bubble_sort(&world, &mut proc_data);

    // Execution of data collection
    let mut gathered_data = if rank == 0 {
        Some(vec![0.0; data_size as usize])
    } else {
        None
    };

    if rank == 0 {
        root_process.gather_into_root(&proc_data, gathered_data.as_mut().unwrap());
    } else {
        root_process.gather_into(&proc_data);
    }

    let duration = start.elapsed();

    if rank == 0 {
        println!("Time of execution: {:?}", duration);
        println!("Sorted data: {:?}", gathered_data.unwrap());
    }

    Ok(())
}

fn parallel_bubble_sort(world: &SimpleCommunicator, proc_data: &mut [f64]) {
    let rank = world.rank();
    let size = world.size();

    // Local sorting
    bubble_sort(proc_data);

    for phase in 0..size {
        let partner = if phase % 2 == 0 {
            // Even phase
            if rank % 2 != 0 {
                rank - 1
            } else {
                rank + 1
            }
        } else {
            // Odd phase
            if rank % 2 != 0 {
                rank + 1
            } else {
                rank - 1
            }
        };

        if partner < 0 || partner == size {
            continue;
        }

        let mut partner_data = vec![0.0; proc_data.len()];
        let partner_rank = partner as usize;

        mpi::request::scope(|scope| {
            world
                .process_at_rank(partner_rank as Rank)
                .immediate_send(scope, proc_data)
                .wait();
            world
                .process_at_rank(partner_rank as Rank)
                .immediate_receive_into(scope, &mut partner_data)
                .wait();
        });

        // Merge and keep the appropriate half
        let mut merged_data = merge(proc_data, &partner_data);
        let (first_half, second_half) = merged_data.split_at_mut(proc_data.len());

        if rank < partner {
            proc_data.copy_from_slice(first_half);
        } else {
            proc_data.copy_from_slice(second_half);
        }
    }
}

fn bubble_sort(data: &mut [f64]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..data.len() {
            if data[i - 1] > data[i] {
                data.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}

fn merge(left: &[f64], right: &[f64]) -> Vec<f64> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_next = left_iter.next();
    let mut right_next = right_iter.next();

    while left_next.is_some() || right_next.is_some() {
        match (left_next, right_next) {
            (Some(&l), None) => {
                merged.push(l);
                left_next = left_iter.next();
            }
            (None, Some(&r)) => {
                merged.push(r);
                right_next = right_iter.next();
            }
            (Some(&l), Some(&r)) => {
                if l < r {
                    merged.push(l);
                    left_next = left_iter.next();
                } else {
                    merged.push(r);
                    right_next = right_iter.next();
                }
            }
            (None, None) => break,
        }
    }

    merged
}
