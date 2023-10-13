use mpi::datatype::Partition;
use mpi::topology::SystemCommunicator;
use mpi::traits::*;

use lab1::parallel::{
    compute_rows_for_rank, get_dispels, input_size_with_checks,
    process_rows_and_vector_multiplication, result_replication, Error,
};
use lab1::serial::{matrix_vector_product, random_data_initialization};

pub fn main() -> Result<(), Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::Mpi);
    };

    let world = universe.world();

    let mut size = 0_u64;
    if world.rank() == 0 {
        println!("Parallel matrix-vector multiplication program");

        size = input_size_with_checks(world.size() as u32)?;
    }

    let root_process = world.process_at_rank(0);
    root_process.broadcast_into(&mut size);

    let process_rank = world.rank();
    let process_count = world.size();
    let bigger_count = size as i32 % process_count;

    let rows_per_process = compute_rows_for_rank(process_rank, size, process_count, bigger_count);

    let mut vector = vec![0; size as usize];
    let mut received_matrix: Vec<u64> = vec![0; (rows_per_process * size as i32) as usize];

    let mut global_res = vec![0; size as usize];

    if world.rank() == 0 {
        root_job(&world, size)?;
    } else {
        worker_job(
            &world,
            &mut vector,
            &mut received_matrix,
            &mut global_res,
            size,
        )?;
    }

    Ok(())
}

fn root_job(world: &SystemCommunicator, size: u64) -> Result<(), Error> {
    let root_process = world.process_at_rank(0);

    let mut matrix: Vec<u64> = vec![];
    let mut vector: Vec<u64> = vec![];

    let mut serial_result = vec![0; size as usize];

    (matrix, vector) = random_data_initialization(size);

    let process_rank = world.rank();
    let process_count = world.size();
    let bigger_count = size as i32 % process_count;

    let rows_per_process = compute_rows_for_rank(process_rank, size, process_count, bigger_count);

    let mut global_res = vec![0; size as usize];
    let mut received_matrix: Vec<u64> = vec![0; (rows_per_process * size as i32) as usize];

    let counts: Vec<i32> = (0..world.size())
        .map(|rank| compute_rows_for_rank(rank, size, process_count, bigger_count) * size as i32)
        .collect();
    let dispels = get_dispels(&counts);

    let partition = Partition::new(&matrix, counts, &dispels[..]);

    root_process.broadcast_into(&mut vector);

    root_process.scatter_varcount_into_root(&partition, &mut received_matrix[..]);
    let t_start = mpi::time();

    let mul_res = process_rows_and_vector_multiplication(&received_matrix, &vector);

    result_replication(&mul_res, &mut global_res, size, &world);
    let duration = mpi::time() - t_start;

    serial_result = matrix_vector_product(&matrix, &vector);

    test_result(serial_result.as_slice(), &global_res, size);

    println!(
        "Time elapsed in parallel matrix_vector_product() is: {:?}",
        duration
    );

    println!(
        "the clock has a resolution of {} seconds",
        mpi::time_resolution()
    );

    Ok(())
}

fn worker_job(
    world: &SystemCommunicator,
    vector: &mut Vec<u64>,
    received_matrix: &mut Vec<u64>,
    global_res: &mut Vec<u64>,
    size: u64,
) -> Result<(), Error> {
    let root_process = world.process_at_rank(0);

    root_process.broadcast_into(vector);
    root_process.scatter_varcount_into(received_matrix);

    let mul_res = process_rows_and_vector_multiplication(&received_matrix, &vector);

    result_replication(&mul_res, global_res, size, &world);

    Ok(())
}

fn test_result(serial_result: &[u64], p_result: &[u64], size: u64) {
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
