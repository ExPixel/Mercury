use thiserror::Error;

use std::io::Error as IoError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] IoError),
}
