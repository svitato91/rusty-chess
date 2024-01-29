use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Connection error: {0}")]
    Tungsetnite(#[from] tungstenite::Error),
}

