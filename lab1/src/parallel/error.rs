use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MPI Error")]
    MPIError,
    #[error("System Error: {0}")]
    SystemError(#[from] FromUtf8Error),
}
