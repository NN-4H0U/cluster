use process::CoachedProcessSpawner;

use crate::Result;
use crate::base::BaseService;


#[derive(Debug)]
pub struct StandaloneService {
    service: BaseService,
}

impl StandaloneService {
    pub async fn new(spawner: CoachedProcessSpawner) -> Self {
        let service = BaseService::new(spawner).await;
        Self { service }
    }
    
    pub async fn spawn(&self) -> Result<()> {
        self.service.spawn(true).await
    }

    pub async fn restart(&self, force: bool) -> Result<()> {
        self.service.spawn(force).await
    }
}

impl std::ops::Deref for StandaloneService {
    type Target = BaseService;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl std::ops::DerefMut for StandaloneService {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.service
    }
}
