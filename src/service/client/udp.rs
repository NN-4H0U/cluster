use std::net::SocketAddr;

use tokio::net::UdpSocket;
use snafu::ResultExt;

use super::error::*;

#[derive(Debug)]
pub(super) struct UdpConnection(UdpSocket);
impl UdpConnection {
    pub(super) async fn open(host: SocketAddr, peer: SocketAddr) -> UdpResult<Self> {
        let host = UdpSocket::bind(host).await.context(OpenSnafu { host })?;
        host.connect(peer).await.context(ConnectSnafu { peer })?;
        Ok(Self(host))
    }

    fn socket(&self) -> &UdpSocket {
        &self.0
    }

    pub(super) async fn send(&self, data: &[u8]) -> UdpResult<()> {
        self.socket().send(data).await.context(SendSnafu {})?;
        Ok(())
    }

    pub(super) async fn recv(&self, buf: &mut [u8]) -> UdpResult<usize> {
        self.socket().recv(buf).await.context(RecvSnafu {})
    }

    pub(super) async fn set_peer(&mut self, peer: SocketAddr) -> UdpResult<()> {
        if self.peer() == peer { return Ok(()) }
        self.socket().connect(peer).await.context(ConnectSnafu { peer })
    }

    pub(super) fn peer(&self) -> SocketAddr {
        self.socket().peer_addr().unwrap()
    }
}