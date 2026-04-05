use kube::Client;
use arcstr::ArcStr;

mod crd;
mod fleet;
mod allocation;
pub mod error;

pub use error::{Error, Result};

pub use allocation::GsAllocation;


#[derive(Clone)]
pub struct K8sClient {
    agones_ns: ArcStr,
    client: Client,
}

impl K8sClient {
    pub async fn new(agones_ns: ArcStr) -> Result<Self> {
        let client = Client::try_default().await
            .map_err(Error::CreateClient)?;
        
        Ok(Self {
            agones_ns,
            client,
        })
    }

    fn client(&self) -> &Client {
        &self.client
    }
}
