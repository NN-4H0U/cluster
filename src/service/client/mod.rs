mod udp;
mod config;
mod kind;
mod error;
mod state;
mod client;

pub use error::{Error, Result};
pub use config::ClientConfig as Config;
pub use kind::ClientKind as Kind;
pub use state::ClientStateEnum as State;
pub use client::Client;


pub const INIT_MSG_TIMEOUT_MS: u64 = 5000;
pub const BUFFER_SIZE: usize = 1500;
