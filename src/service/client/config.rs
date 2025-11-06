use std::net::SocketAddr;
use super::kind::ClientKind;

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub name: String,
    pub kind: ClientKind,
    pub host: SocketAddr,
    pub peer: SocketAddr,
}