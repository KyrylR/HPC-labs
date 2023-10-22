use lab1::common::input_size;
use mpi::topology::{CartesianCommunicator, Communicator, SystemCommunicator};
use mpi::Count;

pub fn create_grid_communicator(
    grid_size: u64,
    world: &SystemCommunicator,
) -> (Vec<Count>, CartesianCommunicator, CartesianCommunicator) {
    let dims = [grid_size as mpi::Count, grid_size as mpi::Count];
    let periodic = [true, true];
    let reorder = true;

    // Create a cartesian communicator
    let cart_comm = world
        .create_cartesian_communicator(&dims, &periodic, reorder)
        .unwrap();

    // Determine the coordinates of the process in the cartesian communicator
    let coords = cart_comm.rank_to_coordinates(world.rank());

    let row_comm = cart_comm.subgroup(&[false, true]);
    let col_comm = cart_comm.subgroup(&[true, false]);

    (coords, row_comm, col_comm)
}

pub fn input_size_with_checks(grid_size: u64, process_number: u32) -> Result<u64, std::io::Error> {
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

        if size % grid_size != 0 {
            println!("Size of the matrix must be divisible by the grid size!");

            continue;
        }
    }

    Ok(size)
}
