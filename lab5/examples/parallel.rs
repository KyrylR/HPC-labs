use mpi::datatype::{Partition, PartitionMut};
use mpi::topology::SimpleCommunicator;
use mpi::traits::{Communicator, CommunicatorCollectives, Root};
use mpi::Rank;

use lab1::common::input_size;
use lab1::parallel::{get_dispels, Error};
use lab5::serial::{min, random_data_initialization, serial_floyd};

// Calculate the number of rows for each process
fn calculate_row_num(proc_rank: i32, proc_num: i32, size: usize) -> usize {
    let mut rest_rows = size;
    for i in 0..proc_rank {
        rest_rows -= rest_rows / (proc_num - i) as usize;
    }
    rest_rows / (proc_num - proc_rank) as usize
}

fn data_distribution(
    p_matrix: &mut Vec<i32>,
    p_proc_rows: &mut [i32],
    size: usize,
    world: &SimpleCommunicator,
) {
    let root_process = world.process_at_rank(0);
    if world.rank() == 0 {
        let counts: Vec<i32> = (0..world.size())
            .map(|rank| calculate_row_num(rank, world.size(), size) as i32)
            .collect();
        let dispels = get_dispels(&counts);

        let partition: &Partition<Vec<i32>, Vec<i32>, &[i32]> =
            &Partition::new(p_matrix, counts, &dispels[..]);

        root_process.scatter_varcount_into_root(partition, p_proc_rows);
    } else {
        root_process.scatter_varcount_into(p_proc_rows);
    }
}

// Parallel Floyd algorithm implementation
fn parallel_floyd(
    p_proc_rows: &mut [i32],
    size: usize,
    row_num: usize,
    world: &SimpleCommunicator,
) {
    let mut p_row = vec![0; size];

    for k in 0..size {
        // Distribute row among all processes
        row_distribution(p_proc_rows, size, k, &mut p_row, world);

        // Update adjacency matrix elements
        for i in 0..row_num {
            for j in 0..size {
                if p_proc_rows[i * size + k] != -1 && p_row[j] != -1 {
                    let t1 = p_proc_rows[i * size + j];
                    let t2 = p_proc_rows[i * size + k] + p_row[j];
                    p_proc_rows[i * size + j] = min(t1, t2);
                }
            }
        }
    }
}

// Function for row broadcasting among all processes
fn row_distribution(
    p_proc_rows: &[i32],
    size: usize,
    k: usize,
    p_row: &mut [i32],
    world: &SimpleCommunicator,
) {
    let mut proc_row_rank: usize = 0;
    let proc_row_num: usize;

    // Finding the process rank with the row k
    let mut rest_rows = size;
    let mut ind = 0;
    let mut num = size / world.size() as usize;

    for rank in 1..=world.size() {
        proc_row_rank = rank as usize;
        if k < ind + num {
            break;
        }
        rest_rows -= num;
        ind += num;
        num = rest_rows / (world.size() - rank) as usize;
    }

    proc_row_rank -= 1;
    proc_row_num = k - ind;

    if proc_row_rank == world.rank() as usize {
        p_row.copy_from_slice(&p_proc_rows[proc_row_num * size..(proc_row_num + 1) * size]);
    }

    let root_process = world.process_at_rank(proc_row_rank as Rank);
    root_process.broadcast_into(p_row);
}

fn result_collection(
    p_matrix: &mut [i32],
    p_proc_rows: &[i32],
    size: usize,
    world: &SimpleCommunicator,
) {
    let counts: Vec<i32> = (0..world.size())
        .map(|rank| (calculate_row_num(rank, world.size(), size) * size) as i32)
        .collect();
    let dispels: Vec<i32> = get_dispels(&counts);

    {
        let mut partition = PartitionMut::new(p_matrix, counts, &dispels[..]);
        world.all_gather_varcount_into(p_proc_rows, &mut partition);
    }
}

fn copy_matrix(p_matrix: &[i32]) -> Vec<i32> {
    p_matrix.to_vec()
}

fn compare_matrices(p_matrix1: &[i32], p_matrix2: &[i32]) -> bool {
    p_matrix1 == p_matrix2
}

fn main() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();

    let world_size = world.size();
    let world_rank = world.rank();

    if world_rank == 0 {
        println!("PParallel Floyd algorithm");
    }

    let mut size: usize = if world_rank == 0 { input_size()? } else { 0 } as usize; // Size of adjacency matrix

    // Broadcasting the size
    let root_process = world.process_at_rank(0);
    root_process.broadcast_into(&mut size);

    let mut p_matrix = vec![0; size * size]; // Adjacency matrix

    if world_rank == 0 {
        random_data_initialization(&mut p_matrix, size);
    }

    let mut p_serial_matrix = vec![0; size * size];

    if world_rank == 0 {
        p_serial_matrix = copy_matrix(&p_matrix);
    }

    let row_num = calculate_row_num(world_rank, world_size, size); // Number of process rows
    let mut p_proc_rows = vec![0; size * row_num]; // Process rows

    let start = mpi::time();
    // Distributing the initial data between processes
    data_distribution(&mut p_matrix, &mut p_proc_rows, size, &world);

    // Parallel Floyd algorithm
    parallel_floyd(&mut p_proc_rows, size, row_num, &world);

    // Process data collection
    result_collection(&mut p_matrix, &p_proc_rows, size, &world);

    if world_rank == 0 {
        println!("Parallel Floyd algorithm");
        println!("Execution time: {} ms", (mpi::time() - start) * 1000.0);

        test_result(&p_matrix, &mut p_serial_matrix, size);
    }

    Ok(())
}

fn test_result(p_matrix: &[i32], p_serial_matrix: &mut [i32], size: usize) {
    serial_floyd(p_serial_matrix, size);

    if !compare_matrices(p_matrix, p_serial_matrix) {
        println!("Results of serial and parallel algorithms are NOT identical. Check your code");
    } else {
        println!("Results of serial and parallel algorithms are identical");
    }
}
