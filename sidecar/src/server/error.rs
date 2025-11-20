#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to start server: system process limit is reached. source: {0}")]
    MaxProcessReached(#[source] std::io::Error),

    #[error(transparent)]
    Io(std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;