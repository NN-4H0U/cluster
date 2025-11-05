mod error;

use std::sync::Weak;
use dashmap::DashMap;
use uuid::Uuid;

use super::team::{self, Team};
use super::room::{self, Room};
use super::udp::UdpConnection;

use error::*;

pub struct Cluster {
    rooms: DashMap<Uuid, Room>,
    clients: DashMap<Uuid, Weak<UdpConnection>>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
            clients: DashMap::new(),
        }
    }

    pub fn create_room(&self, config: room::Config) -> Uuid {
        let id = Uuid::now_v7();
        self.rooms.insert(id, Room::new(config));
        id
    }

    pub fn drop_room(&self, room_id: Uuid) -> Result<()> {
        if self.rooms.remove(&room_id).is_none() {
            return RoomNotFoundSnafu { room_id }.fail();
        }
        Ok(())
    }

    pub async fn create_team_in_room(&self, room_id: Uuid, config: team::Config) -> Result<String> {
        let room = match self.rooms.get(&room_id) {
            Some(room) => room,
            None => return RoomNotFoundSnafu { room_id }.fail(),
        };

        let team_name = room.add_team(None, config).await?;
        Ok(team_name)
    }

    // pub async fn create_client_in_team(&self, room_id: Uuid, team_name: String, config: client::Config) -> Result<Uuid> {
    //     todo!()
    // }
}