use std::fmt::Display;
use std::sync::Arc;

use log::{debug, info};
use tokio::sync::{mpsc, watch, RwLock};
use agones::Sdk as AgonesSdk;

use process::CoachedProcessSpawner;

use crate::{Error, Result};
use super::BaseService;

pub struct AgonesService {
    sdk:    Arc<RwLock<AgonesSdk>>,
    service: BaseService,
}

impl std::ops::Deref for AgonesService {
    type Target = BaseService;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl std::ops::DerefMut for AgonesService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.service
    }
}

impl Display for AgonesService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AgonesService {{ {:?} }}", self.service)
    }
}

impl AgonesService {
    pub async fn new(spawner: CoachedProcessSpawner) -> Self {
        let sdk = AgonesSdk::new(None, None).await.unwrap();
        let sdk = Arc::new(RwLock::new(sdk));

        let service = BaseService::new(spawner).await;
        Self { sdk, service }
    }

    pub async fn spawn(&self) -> Result<()> {
        // >- sdk WRITE lock -<
        let mut sdk_guard = self.sdk.write().await;

        self.service.spawn(false).await?;

        let time_rx = self.service.time().await
            .ok_or(Error::ServerNotRunning { status: self.service.status_now() })?;

        let health_tx = sdk_guard.health_check();
        let _health_task = tokio::spawn(Self::health_check_task(time_rx, health_tx));

        sdk_guard.ready().await.expect("TODO: panic message");

        Ok(())
        // >- sdk WRITE free -<
    }

    async fn health_check_task(mut time_rx: watch::Receiver<Option<u16>>, health_tx: mpsc::Sender<()>) {
        loop {
            if let Ok(_) = time_rx.changed().await {
                // Send a health check signal
                if let Err(e) = health_tx.send(()).await {
                    info!("[AddonProcess] Health check channel closed: {}", e);
                    break;
                }
            } else {
                info!("[AddonProcess] Time status watcher closed");
                break;
            }
        }
    }
}