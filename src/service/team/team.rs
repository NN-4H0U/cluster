use std::sync::{Arc, Weak};

use uuid::Uuid;
use dashmap::DashMap;
use tokio::sync::RwLock;
use crate::service::client::{self, Client};
use crate::service::team::{Config, Side, Status};

use super::error::{Result};

#[derive(Default, Debug)]
pub struct Team {
    side:       Side,
    config:     Config,
    clients:    RwLock<DashMap<Uuid, Weak<Client>>>,
    status:     RwLock<Status>,
}

impl Team {
    pub fn new(side: Side, config: Config) -> Self {
        Self {
            side,
            config,
            ..Default::default()
        }
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.clients.write().await.clear();

        *self.status.write().await = Status::Idle;
        Ok(())
    }

    pub async fn add_client(&self, id: Uuid, client: Weak<Client>) -> Result<Uuid> {
        todo!()
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn is_some(&self) -> bool {
        todo!()
    }
}
