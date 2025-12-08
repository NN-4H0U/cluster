mod process;
mod trainer;
mod test;
mod service;
mod client;

pub use crate::service::Service;
pub use process::Config as ServerConfig;

pub const RCSS_PROCESS_NAME: &'static str = "rcssserver";
pub const PEER_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
