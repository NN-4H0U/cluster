use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};
use chrono::{Utc, Duration};
use log::{debug, error, info};

use service::Service;
use crate::proxy::manager::SessionManager;

#[derive(Clone)]
pub struct AppState {
    pub(crate) service: Arc<Mutex<Service>>,
    pub(crate) session: Arc<SessionManager>,
}

impl AppState {
    pub const CLEANER_POLL_INTERVAL: Duration = Duration::seconds(1);
    pub const CLEANER_TIMEOUT: Duration = Duration::seconds(30);
    
    pub fn new(service: Service, shutdown_notifier: Option<oneshot::Receiver<()>>) -> Self {
        let service = Arc::new(Mutex::new(service));
        
        if let Some(shutdown_notifier) = shutdown_notifier {
            tokio::spawn(Self::run_wait_for_shutdown_cleaner(
                service.clone(), shutdown_notifier));
        }
        
        Self {
            service,
            session: Arc::new(SessionManager::new()),
        }
    }
    
    async fn run_wait_for_shutdown_cleaner(
        service: Arc<Mutex<Service>>,
        shutdown_notifier: oneshot::Receiver<()>,
    ) {
        shutdown_notifier.await.ok();
        debug!("[AppState] Shutdown notifier received, starting polling service shutdown...");
        
        let mut interval = tokio::time::interval(Self::CLEANER_POLL_INTERVAL.to_std().unwrap());
        let start_at = Utc::now();
        
        loop {
            interval.tick().await;
            
            match service.lock().await.shutdown().await {
                Ok(_) => {
                    info!("[AppState] Service shutdown completed in {}ms.", 
                        (Utc::now() - start_at).num_milliseconds());
                    break;
                },
                Err(e) => {
                    error!("[AppState] Service shutdown failed: {}. Retrying...", e);
                },
            }
            
            if (Utc::now() - start_at) > Self::CLEANER_TIMEOUT {
                error!("[AppState] Service shutdown timed out after {} seconds.", 
                    Self::CLEANER_TIMEOUT.num_seconds());
                break;
            }
        }
    }
}
