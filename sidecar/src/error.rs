#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Server is not running, current status: {status:?}")]
    ServerNotRunning {
        status: crate::sidecar::SidecarStatus,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
