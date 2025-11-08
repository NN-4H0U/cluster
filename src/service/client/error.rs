use std::backtrace::Backtrace;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::mpsc;

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum UdpError {
    #[snafu(display("Failed to bind UDP socket on \"{host}\""))]
    Open {
        host: SocketAddr,
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to connect \"{peer}\""))]
    Connect {
        peer: SocketAddr,
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to send"))]
    Send {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to receive"))]
    Recv {
        source: std::io::Error,
        backtrace: Backtrace,
    },
}

pub type UdpResult<T> = std::result::Result<T, UdpError>;

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Client[{client_name}]: Udp Error: {source}"))]
    Udp {
        client_name: String,
        source: UdpError,
        backtrace: Backtrace,
    },

    #[snafu(display("Client[{client_name}]: Timeout({duration_s} s) waiting an initial message."))]
    TimeoutInit {
        client_name: String,
        duration_s: f32,
        backtrace: Backtrace,
    },

    #[snafu(display("Client[{client_name}]: Timeout waiting for UDP response, {source}"))]
    TimeoutUdp {
        client_name: String,
        source: UdpError,
        backtrace: Backtrace
    },

    #[snafu(display("Client[{client_name}]: Channel closed unexpectedly."))]
    ChannelClosed {
        client_name: String,
        backtrace: Backtrace
    },

    #[snafu(display("Client[{client_name}]: Failed to send to channel, {source}"))]
    ChannelSend {
        client_name: String,
        source: mpsc::error::SendError<Arc<str>>,
        backtrace: Backtrace,
    },

    #[snafu(display("Client[{client_name}]: Task Join Error in \"{task_desc}\", {source}"))]
    TaskJoin {
        client_name: String,
        task_desc: String,
        source: tokio::task::JoinError,
        backtrace: Backtrace,
    }
    
}

pub type Result<T> = std::result::Result<T, Error>;