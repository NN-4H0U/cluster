mod udp;
mod config;
mod kind;
mod error;

pub use error::{Error, Result};
pub use config::ClientConfig as Config;
pub use kind::ClientKind as Kind;

use std::sync::Arc;
use snafu::ResultExt;

use udp::UdpConnection;
use error::*;
use crate::model::rcss;

#[derive(Debug)]
pub struct Client {
    config: Config,
    conn: Arc<UdpConnection>,
}

impl Client {
    pub async fn conn(config: Config) -> Result<Self> {
        let conn = UdpConnection::open(config.host, config.peer).await
            .context(UdpSnafu { client_name: config.name.clone() })?;
        
        Ok(Self { config, conn: Arc::new(conn) })
    }
    
    pub async fn send(&self, data: impl rcss::Message) -> Result<()> {
        self.conn.send(data.as_bytes()).await
            .context(UdpSnafu { client_name: self.name().to_string() })
    }
    
    pub async fn recv(&self, buf: &mut [u8]) -> Result<usize> {
        self.conn.recv(buf).await
            .context(UdpSnafu { client_name: self.name().to_string() })
    }
    
    pub fn name(&self) -> &str {
        self.config.name.as_str()
    }
    
}