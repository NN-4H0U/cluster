use std::sync::{Arc, Weak};
use arcstr::ArcStr;
use dashmap::DashMap;

use crate::agones::AgonesClient;
use crate::room::{LazyRoom, RoomConfig, WsConfig};
use crate::utils::local_addr;
use crate::{Error, Result};

#[derive(Debug)]
pub struct ProxyServerConfig {
    // pub agones_config: AgonesConfig,
    // pub api_config: ApiConfig,
}

#[derive(Debug)]
pub struct ProxyServer {
    pub config: ProxyServerConfig,
    pub agones: AgonesClient,
    rooms: DashMap<String, Weak<LazyRoom>>,
}

impl ProxyServer {
    pub fn new(config: ProxyServerConfig) -> Self {
        let agones = AgonesClient::new("http://localhost:8080".parse().unwrap());
        Self {
            config,
            agones,
            rooms: DashMap::new(),
        }
    }

    pub async fn create_room(&self, room_name: String, udp_port: u16) -> Result<RoomConfig> {
        let ws_url = self.agones.allocate().await.expect("Failed to allocate room");

        let config = {
            let ws_config = {
                let mut builder = WsConfig::builder();
                builder.with_base_url(ws_url);
                builder.build_into()
            };

            let mut builder = RoomConfig::builder();
            builder
                .with_ws(ws_config)
                .with_name(room_name.clone())
                .with_player_udp(local_addr(udp_port))
                .build();
            builder.build_into()
        };

        let room = Arc::new(LazyRoom::new(config));
        self.rooms.insert(room_name.clone(), Arc::downgrade(&room));
        let _task = room.spawn().await?;

        if  let Some(room) = self.rooms.get(&room_name) &&
            let Some(room) = room.value().upgrade()
            {
            Ok(room.config().unwrap())
        } else {
            Err(Error::RoomDropped { room_name })
        }
    }

    pub fn room(&self, room_name: &str) -> Result<Arc<LazyRoom>> {
        self.rooms.get(room_name)
            .ok_or(Error::RoomNotFound { room_name: room_name.to_string() })?
            .value().upgrade()
            .ok_or(Error::RoomDropped { room_name: room_name.to_string() })
    }
}
