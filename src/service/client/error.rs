use std::backtrace::Backtrace;
use std::net::SocketAddr;

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
    
}

pub type Result<T> = std::result::Result<T, Error>;