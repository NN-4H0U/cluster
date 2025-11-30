use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::mpsc;
use dashmap::DashMap;
use arcstr::ArcStr;
use log::debug;
use uuid::Uuid;

use common::client::TxMessage;

use super::{client, signal};
use super::addon::AddOn;
use super::signal::Signal;
use super::resolver::CallResolver;
use super::{Error, Result, Builder};

#[derive(Debug)]
pub struct OfflineCoach {
    conn: client::Client,
    addons: DashMap<&'static str, Box<dyn AddOn>>
}

impl OfflineCoach {
    pub fn builder() -> Builder {
        Builder::default()
    }
    
    pub fn new(
        name: String,
        host: Option<SocketAddr>,
        peer: Option<SocketAddr>
    ) -> Self {
        
        let mut config = client::Config::builder();
        config.with_name(name).with_kind(client::Kind::Trainer);
        config.host = host;
        config.peer = peer;

        let conn = client::Client::new(config.build());
        Self { conn, addons: DashMap::new() }
    }
    
    pub fn from_client_config(config: client::Config) -> Self {
        assert_eq!(config.kind, client::Kind::Trainer, "ClientKind::Trainer expected");
        let conn = client::Client::new(config);
        Self { conn, addons: DashMap::new() }
    }

    fn add_addon<A: AddOn>(&self, name: &'static str) -> Uuid {
        let (tx, rx) = mpsc::channel(32);
        let id = self.conn.subscribe(tx);
        self.addons.insert(name, Box::new(A::new(self.conn.sender(), rx)));

        id
    }

    pub async fn connect(&self) -> Result<()> {
        self.conn.connect().await.expect("Failed to connect");
        self.add_addon::<CallResolver>("call_resolver");
        self.send_ctrl(signal::Init { version: Some(5) }).await.expect("Failed to send init signal");
        Ok(())
    }

    pub fn sender(&self) -> mpsc::WeakSender<TxMessage> {
        self.conn.sender()
    }

    pub fn subscribe(&self, tx: mpsc::Sender<ArcStr>) -> Uuid {
        self.conn.subscribe(tx)
    }

    pub fn unsubscribe(&self, id: Uuid) -> bool {
        self.conn.unsubscribe(id)
    }

    pub async fn send_ctrl(&self, ctrl: impl Signal) -> Result<()> {
        let ctrl = ctrl.encode();
        self.conn
            .send(client::Signal::Data(ctrl)).await
            .map_err(|e| Error::ClientClosed { source: e })?;
        Ok(())
    }

    pub async fn shutdown(self) -> Result<()> {
        self.conn.close().await
            .map_err(|e| Error::ClientCloseFailed { source: e })?;
        Ok(())
    }
    
    pub fn config_mut(&mut self) -> &mut client::Config {
        self.conn.config_mut()
    }
}

impl Default for OfflineCoach {
    fn default() -> Self {
        Self::builder().build()
    }
}
