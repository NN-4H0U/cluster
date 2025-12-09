mod process;
mod trainer;
mod test;
mod service;
mod client;
mod sidecar;
mod error;

pub use sidecar::{Sidecar, SidecarStatus};
pub use process::Config as ServerConfig;
pub use error::{Error, Result};

pub const RCSS_PROCESS_NAME: &'static str = "rcssserver";
pub const PEER_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
