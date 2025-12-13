mod client;
mod controller;
mod error;
mod process;
mod server;
mod service;
mod test;
mod trainer;

pub use error::{Error, Result};
pub use process::Config as ServerConfig;
pub use server::{Server, SidecarStatus};

pub const RCSS_PROCESS_NAME: &str = "rcssserver";
pub const PEER_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
