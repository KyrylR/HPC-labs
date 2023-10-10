use mpi::topology::SystemCommunicator;
use mpi::traits::*;

mod example;
pub use crate::parallel::example::example;

mod error;
use crate::parallel::error::Error;

pub fn init_program() -> Result<SystemCommunicator, Error> {
    let Some(universe) = mpi::initialize() else {
        return Err(Error::MPIError);
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
