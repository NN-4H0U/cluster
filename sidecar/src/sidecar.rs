use std::ops::Deref;
use std::sync::atomic::AtomicU8;
use futures::AsyncReadExt;
use log::info;
use tokio::sync::RwLock;
use common::command::{Command, CommandResult};
use common::command::trainer::TrainerCommand;

use crate::{Error, Result};
use crate::process;
use crate::service::{Service, CoachedProcess, CoachedProcessSpawner};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum SidecarStatus {
    Uninitialized,
    Idle,
    Simulating,
    Finished,
}

impl Into<u8> for SidecarStatus {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for SidecarStatus {
    type Error = ();
    
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(SidecarStatus::Uninitialized),
            1 => Ok(SidecarStatus::Idle),
            2 => Ok(SidecarStatus::Simulating),
            3 => Ok(SidecarStatus::Finished),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum SidecarService {
    Uninitialized,
    Running(Service),
}

impl SidecarService {
    pub fn service(&self) -> Option<&Service> {
        match self {
            SidecarService::Uninitialized => None,
            SidecarService::Running(s) => Some(s),
        }
    }

    pub async fn send_trainer_command<C>(&self, command: C) -> Option<CommandResult<C>>
    where C: Command<Kind=TrainerCommand>,
    {
        Some(self.service()?.send_trainer_command(command).await)
    }
    
    pub fn is_initialized(&self) -> bool {
        !matches!(self, SidecarService::Uninitialized)
    }
}

#[derive(Debug)]
pub struct Sidecar {
    spawner: CoachedProcessSpawner,
    service: RwLock<SidecarService>,
    status: AtomicU8,
}

impl Sidecar {
    pub async fn new() -> Self {
        let spawner = CoachedProcess::spawner().await;
        let service = RwLock::new(SidecarService::Uninitialized);
        let status = AtomicU8::new(SidecarStatus::Uninitialized.into());
        
        Self {
            spawner,
            service,
            status,
        }
    }
    
    fn set_status(&self, status: SidecarStatus) {
        self.status.store(status.into(), std::sync::atomic::Ordering::SeqCst);
    }
    
    pub fn status(&self) -> SidecarStatus {
        let val = self.status.load(std::sync::atomic::Ordering::SeqCst);
        SidecarStatus::try_from(val).expect("invalid sidecar status")
    }

    pub async fn send_trainer_command<C: Command<Kind=TrainerCommand>>(&self, command: C) -> Result<CommandResult<C>> {
        self.service.read().await.send_trainer_command(command).await.ok_or(Error::ServerNotRunning {
            status: self.status()
        })
    }

    pub async fn spawn(&self) {
        // >- service WRITE lock -<
        let mut service_guard = self.service.write().await;
        if service_guard.is_initialized() {
            return todo!("error: service already spawned")
        }

        let process = self.spawner.spawn().await.expect("TODO: spawn error handling");
        let service = Service::from_coached_process(process);
        info!("[Sidecar] Service spawned");
        
        *service_guard = SidecarService::Running(service);
        self.set_status(SidecarStatus::Idle);
        // >- service WRITE free -<
    }
    
    pub fn config(&self) -> &process::Config {
        &self.spawner.process.config
    }

    pub async fn spawn_service(&self) -> Service {
        let process = self.spawner.spawn().await.unwrap();
        info!("[ServiceFactory] Process spawned");

        Service::from_coached_process(process)
    }
}
