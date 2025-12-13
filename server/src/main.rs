#[cfg(target_os = "windows")]
compile_error!("This program currently not supported on Windows.");

mod client;
mod controller;
mod error;
mod process;
mod service;
mod server;
mod test;
mod trainer;

pub use error::{Error, Result};
pub use process::Config as ServerConfig;
pub use server::{Server, SidecarStatus};

use std::env;
use log::info;

pub const RCSS_PROCESS_NAME: &str = "rcssserver";
pub const PEER_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    async {
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }
        info!("Shutdown signal received, aborting...");
    }.await
}

#[tokio::main]
async fn main() {
    unsafe { env::set_var("RUST_LOG", "trace") };
    env_logger::init();

    let app = controller::listen("0.0.0.0:55555", Some(shutdown_signal())).await;
    app.await.unwrap().unwrap();
}
