mod agones;
mod room;
mod utils;
mod error;
mod proxy;

use error::{Error, Result};
use crate::proxy::ProxyServerConfig;

#[tokio::main]
async fn main() {
    let proxy_server = proxy::ProxyServer::new(ProxyServerConfig {});
    let room = proxy_server.create_room("test".to_string(), 8888).await;
    assert!(room.is_ok());
}
