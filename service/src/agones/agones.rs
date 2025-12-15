use std::fmt::Display;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use log::{debug, info, warn};
use tokio::sync::{mpsc, watch, RwLock};
use agones::Sdk as AgonesSdk;

use process::CoachedProcessSpawner;

use crate::{Error, Result, ServerStatus};
use super::BaseService;

pub struct Config {
    pub health_check_interval: Duration,
    pub auto_shutdown_on_finish: bool,
}

pub struct AgonesService {
    sdk:    Arc<RwLock<AgonesSdk>>,
    cfg:    Config,
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
        let config = Config {
            health_check_interval: Duration::from_secs(5),
            auto_shutdown_on_finish: true,
        };

        Self { sdk, service, cfg: config }
    }

    pub async fn spawn(&self) -> Result<()> {
        // >- sdk WRITE lock -<
        let mut sdk_guard = self.sdk.write().await;

        self.service.spawn(false).await?;

        let status_rx = self.service.status();
        let health_tx = sdk_guard.health_check();
        let _health_task = tokio::spawn(
            Self::run_health_check(status_rx, health_tx, self.health_check_interval()));

        sdk_guard.ready().await.expect("TODO: panic message");

        Ok(())
        // >- sdk WRITE free -<
    }

    async fn run_health_check(
        status_rx: watch::Receiver<ServerStatus>,
        health_tx: mpsc::Sender<()>,
        duration: Duration,
    ) -> () {
        let mut ticker = tokio::time::interval(duration);

        loop {
            ticker.tick().await;

            if !status_rx.borrow().is_healthy() { break }

            debug!("[AgonesService] Sending health ping to Agones SDK");
            if health_tx.send(()).await.is_err() {
                warn!("[AgonesService] Health check task ending: Health channel closed");
                break;
            }
        }
    }

    pub async fn shutdown_signal(&self) {

        let shutdown_on_finish: Box<dyn Future<Output = Option<()>> + Unpin>
            = if self.cfg.auto_shutdown_on_finish {
            let mut status_rx = self.service.status();
            Box::new(
                Box::pin(async move {
                    status_rx.wait_for(|s| s.is_finished()).await.ok().map(|_| ())
                })
            )
        } else {
            Box::new(
                Box::pin(async {
                    futures::future::pending::<()>().await;
                    None
                })
            )
        };

        tokio::select! {
            _ = shutdown_on_finish => {
                info!("[AgonesService] Shutdown signal received: Server finished and auto-shutdown is enabled.");
            }
        }
    }

    pub async fn shutdown(&self) -> Result<()> {
        // >- sdk WRITE lock -<
        let mut sdk_guard = self.sdk.write().await;
        self.service.shutdown().await?;
        sdk_guard.shutdown()
            .await.expect("TODO: panic message");

        Ok(())
        // >- sdk WRITE free -<
    }

    fn health_check_interval(&self) -> Duration {
        self.cfg.health_check_interval
    }
}