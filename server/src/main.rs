mod controller;

use std::env;


pub const PEER_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

#[tokio::main]
async fn main() {
    unsafe { env::set_var("RUST_LOG", "debug") }
    env_logger::init();

    let app = controller::listen("0.0.0.0:55555").await;
    app.await.unwrap().unwrap();
}
