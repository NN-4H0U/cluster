use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::{mpsc, OnceCell};
use dashmap::DashMap;
use arcstr::ArcStr;
use log::{debug, trace};
use uuid::Uuid;

use common::client;

use super::addon::AddOn;
use super::command::{self, Command};
use super::resolver::{CallResolver, Sender};
use super::{Error, Result, Builder};

#[derive(Debug)]
pub struct OfflineCoach {
    conn: client::Client,
    resolver_tx: OnceCell<Sender>,
    addons: DashMap<&'static str, Box<dyn AddOn>>,
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
        Self { conn, resolver_tx: OnceCell::new(), addons: DashMap::new() }
    }
    
    pub fn from_client_config(config: client::Config) -> Self {
        assert_eq!(config.kind, client::Kind::Trainer, "ClientKind::Trainer expected");
        let conn = client::Client::new(config);
        Self { conn, resolver_tx: OnceCell::new(), addons: DashMap::new() }
    }

    fn add_addon<A: AddOn>(&self, name: &'static str) -> Uuid {
        trace!("[Coach] Adding addon '{name}'");
        let (tx, rx) = mpsc::channel(32);
        let id = self.conn.subscribe(tx);
        self.addons.insert(name, Box::new(
            A::new(self.conn.signal_sender(), self.conn.data_sender(), rx)
        ));

        trace!("[Coach] Addon '{name}' added, id = {id}");
        id
    }

    fn init_resolver(&self) -> Result<Uuid> {
        trace!("[Coach] Initializing CallResolver addon.");
        let resolver = CallResolver::new(32);
        self.resolver_tx.set(resolver.sender(self.conn.data_sender())).unwrap();
        let id = self.subscribe(resolver.ingest_tx());
        trace!("[Coach] CallResolver addon initialized, id = {id}");
        self.addons.insert("call_resolver", Box::new(resolver));

        Ok(id)
    }

    pub async fn connect(&self) -> Result<()> {
        trace!("[Coach] Connecting to host {:?} via peer {:?}", self.conn.config().host, self.conn.config().peer);
        self.conn.connect().await.expect("Failed to connect");
        debug!("[Coach] Connected.");
        let _ = self.init_resolver()?;
        debug!("[Coach] CallResolver initialized.");
        self.call(command::Init { version: Some(5) }).await.expect("Failed to send init signal");
        Ok(())
    }

    pub async fn call<T: Command>(&self, cmd: T) -> std::result::Result<T::Ok, T::Error> {
        self.resolver_tx.get()
            .expect("CallResolver not initialized")
            .send(cmd).await
    }

    pub fn sender(&self) -> mpsc::Sender<client::TxData> {
        self.conn.data_sender()
    }

    pub fn weak(&self) ->  mpsc::WeakSender<client::TxData> {
        self.conn.data_sender_weak()
    }

    pub fn subscribe(&self, tx: mpsc::Sender<ArcStr>) -> Uuid {
        self.conn.subscribe(tx)
    }

    pub fn unsubscribe(&self, id: Uuid) -> bool {
        self.conn.unsubscribe(id)
    }

    pub async fn send_cmd(&self, ctrl: impl Command) -> Result<()> {
        self.conn.send_data(ctrl.encode()).await
            .map_err(|e| Error::ClientClosed { source: e })?;
        Ok(())
    }

    pub async fn shutdown(self) -> Result<()> {
        self.conn.close().await
            .map_err(|e| Error::ClientCloseFailed { source: e })?;

        for (key, addon) in self.addons.into_iter() {
            addon.close();
            trace!("Addon '{}' closed", key);
        }

        Ok(())
    }
    
    pub fn config(&self) -> &client::Config {
        self.conn.config()
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
