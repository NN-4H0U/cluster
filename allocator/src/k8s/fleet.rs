use kube::api::{Api, DeleteParams, PostParams};
use serde_json::Value;

use crate::metadata::MetaData;
use crate::schema::v1::ConfigV1;

use super::{Error, Result, K8sClient};
use super::crd::{ContainerBuilder, Fleet, FleetBuilder};


impl K8sClient {
    pub async fn create_fleet(&self, name: String, gs_conf: Value, version: u8) -> Result<()> {
        match version {
            1 => {
                let conf = serde_json::from_value(gs_conf).map_err(Error::InvalidFleetGS)?;
                self.create_fleet_v1(name, conf).await
            }
            _ => Err(Error::UnsupportedVersion {
                version,
                resource: "Fleet",
                supported: &[1],
            }),
        }
    }

    pub async fn create_fleet_v1(&self, name: String, gs_conf: ConfigV1) -> Result<()> {
        let api: Api<Fleet> = Api::namespaced(self.client.clone(), &self.namespace);

        // Convert ConfigV1 into labels & annotations
        let metadata: MetaData = gs_conf.try_into()
            .map_err(|e: common::errors::BuilderError| Error::InvalidMetaData(format!("{e:?}")))?;

        let labels = metadata.labels.into_map();
        let annotations = metadata.annotations.into_map();

        let fleet = {
            let mut fleet = FleetBuilder::new();
            fleet.with_name(name)
                .with_labels(labels)
                .with_annotations(annotations)
                .add_container(ContainerBuilder::default_rcss())
                .map_err(|e| Error::InvalidMetaData(format!("{e:?}")))?;
            
            fleet.build_into()
                .map_err(|e| Error::InvalidMetaData(format!("{e:?}")))?
        };
        
        api.create(&PostParams::default(), &fleet).await
            .map_err(Error::CreateFleet)?;

        Ok(())
    }

    pub async fn drop_fleet(&self, name: &str) -> Result<()> {
        let api: Api<Fleet> = Api::namespaced(self.client.clone(), &self.namespace);

        match api.delete(name, &DeleteParams::default()).await {
            Ok(_) => Ok(()),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(()),
            Err(e) => Err(Error::DeleteFleet(e)),
        }
    }
}