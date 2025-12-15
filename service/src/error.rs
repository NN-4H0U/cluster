#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Server is not running, current status: {status:?}")]
    ServerNotRunning { status: crate::ServerStatus },

    #[error("Server is still running, try to set force=true to force respawn.")]
    ServerStillRunningToSpawn,

    #[error("Operation '{op}' timed out.")]
    Timeout { op: &'static str, }
}

pub type Result<T> = std::result::Result<T, Error>;
