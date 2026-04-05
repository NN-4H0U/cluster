use std::collections::HashMap;
use std::fmt::Debug;
use std::net::IpAddr;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, PostParams};

use common::errors::{BuilderError, BuilderResult};

use crate::k8s::crd::{
    AllocationMetadata, GameServerAllocation, GameServerAllocationSpec, GameServerPort,
    GameServerSelector,
};
use crate::k8s::K8sClient;
use crate::metadata::MetaData;
use super::{Error, Result};

#[derive(Debug, Clone)]
pub struct GsAllocation {
    pub name: String,
    pub host: IpAddr,
    pub ports: HashMap<String, u16>,
}

impl GsAllocation {
    pub fn builder() -> GsAllocationBuilder {
        GsAllocationBuilder::new()
    }
}

#[derive(Default, Debug, Clone)]
pub struct GsAllocationBuilder {
    name: Option<String>,
    host: Option<IpAddr>,
    ports: HashMap<String, u16>,
}
impl GsAllocationBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn parse_host(&mut self, host: Option<&String>) -> BuilderResult<&mut Self> {
        let host = host.ok_or(BuilderError::MissingField { field: "host" })?;
        let host = host.parse().map_err(|_|BuilderError::InvalidValue {
            field: "host",
            value: host.to_string(),
            expected: "an IpAddr".to_string(),
        })?;
        
        self.host = Some(host);
        Ok(self)
    }
    
    pub fn with_host(&mut self, host: impl Into<IpAddr>) -> &mut Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn parse_ports(&mut self, ports: Vec<GameServerPort>) -> &mut Self {
        for port in ports {
            self.add_port(port.name, port.port);
        }
        self
    }
    
    pub fn add_port(&mut self, name: impl Into<String>, port: u16) -> &mut Self {
        self.ports.insert(name.into(), port);
        self
    }
    
    pub fn with_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn set_name(&mut self, name: Option<String>) -> &mut Self {
        self.name = name;
        self
    }
    
    pub fn build_into(self) -> BuilderResult<GsAllocation> {
        let name = self.name.ok_or(BuilderError::MissingField { field: "name" })?;
        let host = self.host.ok_or(BuilderError::MissingField { field: "host" })?;
        let ports = self.ports;
        if ports.is_empty() {
            return Err(BuilderError::MissingField { field: "ports" });
        }
        
        Ok(GsAllocation { name, host, ports })
    }
}

impl K8sClient {
    pub async fn allocate(
        &self,
        fleet_name: &str,
        scheduling: &str,
        metadata: impl TryInto<MetaData, Error: Debug>,
    ) -> Result<GsAllocation> {
        let api: Api<GameServerAllocation> = Api::namespaced(self.client.clone(), &self.agones_ns);

        let metadata = metadata.try_into()
            .map_err(|e| Error::InvalidMetaData(format!("{e:?}")))?;

        // Build allocation metadata with annotations
        let allocation_metadata = AllocationMetadata {
            annotations: Some(metadata.annotations.into_map()),
        };

        // Build selector to match fleet
        let match_labels = {
            let mut labels = metadata.labels.into_map();
            labels.insert("agones.dev/fleet".to_string(), fleet_name.to_string());
            labels
        };

        let selector = GameServerSelector {
            match_labels: Some(match_labels),
            match_expressions: None,
        };

        // Create allocation request
        let allocation = GameServerAllocation {
            api_version: "allocation.agones.dev/v1".to_string(),
            kind: "GameServerAllocation".to_string(),
            metadata: ObjectMeta {
                namespace: Some(self.agones_ns.to_string()),
                ..Default::default()
            },
            spec: GameServerAllocationSpec {
                selectors: vec![selector],
                scheduling: Some(scheduling.to_string()),
                metadata: Some(allocation_metadata),
            },
            status: None,
        };

        // Submit allocation
        let result = api.create(&PostParams::default(), &allocation).await
            .map_err(Error::NoSuchGs)?;
        
        // Parse response
        let status = result.status
            .ok_or(Error::GsaBadResponse(BuilderError::MissingField { field: "status" }))?;

        // Check allocation state
        if status.state != "Allocated" {
            return Err(Error::GsaExhausted(
                "当前训练资源已满，请稍后重试".to_string(),
            ));
        }

        let res = {
            let mut builder = GsAllocationBuilder::new();
            builder
                .parse_host(status.address.as_ref()).map_err(Error::GsaBadResponse)?
                .parse_ports(status.ports.unwrap_or_default())
                .set_name(status.game_server_name.clone());
            builder.build_into().map_err(Error::GsaBadResponse)?
        };
        
        Ok(res)
    }
}

