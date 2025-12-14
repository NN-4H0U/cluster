use log::{debug, info};
use std::sync::Arc;
use std::sync::atomic::AtomicU8;

use tokio::sync::{RwLock, watch};
use tokio::task::JoinHandle;

use common::command::trainer::TrainerCommand;
use common::command::{Command, CommandResult};
use process::{self, CoachedProcess, CoachedProcessSpawner, ProcessConfig};

use super::AddonProcess;
use crate::{Error, Result, ServiceStatus};

#[derive(Debug)]
pub enum OptionedService {
    Uninitialized,
    Running(AddonProcess),
}

pub fn set_status(atomic: &AtomicU8, status: ServiceStatus) {
    atomic.store(status.into(), std::sync::atomic::Ordering::SeqCst);
}

pub fn get_status(atomic: &AtomicU8) -> ServiceStatus {
    ServiceStatus::try_from(atomic.load(std::sync::atomic::Ordering::SeqCst)).unwrap()
}

impl OptionedService {
    pub fn service(&self) -> Option<&AddonProcess> {
        match self {
            OptionedService::Uninitialized => None,
            OptionedService::Running(s) => Some(s),
        }
    }

    pub fn service_mut(&mut self) -> Option<&mut AddonProcess> {
        match self {
            OptionedService::Uninitialized => None,
            OptionedService::Running(s) => Some(s),
        }
    }

    pub async fn send_trainer_command<C>(&self, command: C) -> Option<CommandResult<C>>
    where
        C: Command<Kind = TrainerCommand>,
    {
        Some(self.service()?.send_trainer_command(command).await)
    }

    pub fn is_initialized(&self) -> bool {
        !matches!(self, OptionedService::Uninitialized)
    }

    pub fn is_running(&self) -> bool {
        matches!(self, OptionedService::Running(_))
    }
}

#[derive(Debug)]
pub struct StandaloneService {
    spawner: CoachedProcessSpawner,
    service: RwLock<OptionedService>,
    status: Arc<AtomicU8>,
}

impl StandaloneService {
    pub async fn new() -> Self {
        let spawner = CoachedProcess::spawner().await;
        let service = RwLock::new(OptionedService::Uninitialized);
        let status = Arc::new(AtomicU8::new(ServiceStatus::Uninitialized.into()));

        Self {
            spawner,
            service,
            status,
        }
    }
    fn status_tracing_task(
        atomic: Arc<AtomicU8>,
        mut time_rx: watch::Receiver<Option<u16>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let timestep = match time_rx.changed().await {
                    Ok(_) => *time_rx.borrow(),
                    Err(_) => {
                        set_status(&atomic, ServiceStatus::Finished);
                        debug!(
                            "[StandaloneService] Status Tracking ended due to time_rx channel closed."
                        );
                        break;
                    }
                };

                let next_status = match (get_status(&atomic), timestep) {
                    (ServiceStatus::Uninitialized, Some(0)) => ServiceStatus::Idle,
                    (ServiceStatus::Uninitialized, Some(_)) => ServiceStatus::Simulating,
                    (ServiceStatus::Idle, Some(t)) if t > 0 && t < 6000 => {
                        ServiceStatus::Simulating
                    }
                    (ServiceStatus::Idle, Some(t)) if t >= 6000 => ServiceStatus::Finished,
                    (ServiceStatus::Simulating, Some(t)) if t >= 6000 => ServiceStatus::Finished,
                    _ => continue,
                };

                debug!(
                    "[StandaloneService] Status Tracking: {:?} -> {:?}",
                    get_status(&atomic),
                    next_status
                );
                set_status(&atomic, next_status);
            }
        })
    }

    fn set_status(&self, status: ServiceStatus) {
        set_status(&self.status, status);
    }

    pub fn status(&self) -> ServiceStatus {
        get_status(&self.status)
    }

    pub async fn send_trainer_command<C: Command<Kind = TrainerCommand>>(
        &self,
        command: C,
    ) -> Result<CommandResult<C>> {
        self.service
            .read()
            .await
            .send_trainer_command(command)
            .await
            .ok_or(Error::ServerNotRunning {
                status: self.status(),
            })
    }

    pub async fn spawn(&self) -> Result<()> {
        // >- service WRITE lock -<
        let mut service_guard = self.service.write().await;

        if let Some(service) = service_guard.service_mut() {
            // is running
            service.shutdown().await.expect("JB 没关掉");
        }
        let process = self.spawner.spawn().await.expect("JB 没开起来");
        let service = AddonProcess::from_coached_process(process);
        info!("[StandaloneService] AddonProcess spawned");

        let time_rx = service.time_watch();
        StandaloneService::status_tracing_task(self.status.clone(), time_rx);

        *service_guard = OptionedService::Running(service);
        self.set_status(ServiceStatus::Idle);

        Ok(())
        // >- service WRITE free -<
    }

    pub async fn restart(&self, force: bool) -> Result<()> {
        if force || !self.status().is_running() {
            return self.spawn().await;
        }

        Err(Error::ServerStillRunningToRestart)
    }

    pub fn config(&self) -> &ProcessConfig {
        &self.spawner.process.config
    }
}
