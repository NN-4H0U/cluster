use nix::sys::signal::Signal;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to start server: system process limit is reached. source: {0}")]
    MaxProcessReached(#[source] std::io::Error),

    #[error(transparent)]
    Io(std::io::Error),

    #[error(transparent)]
    SignalSend(tokio::sync::mpsc::error::SendError<Signal>),

    #[error(transparent)]
    ProcessJoinTimeout(tokio::time::error::Elapsed),

    #[error(transparent)]
    ProcessJoin(tokio::task::JoinError),

    #[error("FATAL: Can not wind up process!")]
    FatalProcessWindingUp {
        pid: Option<u32>,
        signal: Signal,
        error: std::io::Error,
    },
    
    #[error("Can not get pid, for the child process is already completed.")]
    ChildAlreadyCompleted,
}

pub type Result<T> = std::result::Result<T, Error>;