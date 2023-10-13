use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MPI Error")]
    Mpi,
    #[error("System Error: {0}")]
    System(#[from] FromUtf8Error),
    #[error("Input Error: {0}")]
    Input(#[from] std::io::Error),
}
