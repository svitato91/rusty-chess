use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error)
}
