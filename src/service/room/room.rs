use std::sync::Arc;
use std::collections::HashMap;
use log::{debug, trace, warn};

use uuid::Uuid;
use tokio::sync::RwLock;
use dashmap::DashMap;
use dashmap::mapref::one::Ref;
use crate::service::team::{Team, Config as TeamConfig, Side as TeamSide};
use crate::service::client::Client;

use super::{Status, Config};
use super::error::*;

#[derive(Default, Debug)]
pub struct Room {
    id:         Uuid,
    config:     Config,

    teams:      RwLock<DashMap<String, Team>>,
    team_l:     DashMap<String, TeamSide>,
    team_r:     DashMap<String, TeamSide>,
    trainer:    DashMap<Uuid, Arc<Client>>,

    status:     Status,
}

impl Room {
    pub const NUM_TEAMS: usize = 2;

    pub fn new(config: Config) -> Self {
        Room {
            config,
            id: Uuid::now_v7(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub async fn add_team(&self, side: Option<TeamSide>, config: TeamConfig) -> Result<String> {
        if self.teams.read().await.len() >= Self::NUM_TEAMS {
            debug!("Room[{}]: is Full, cannot add team {}", self.name(), config.name);
            return RoomIsFullSnafu {
                room_name: self.name().to_string(),
                pending_team: config.name.to_string(),
            }.fail();
        }

        let team_name = { // [self.teams] WRITE LOCK
            trace!("Room[{}]: trying the lock: `self.teams`", self.name());
            let teams_guard = self.teams.write().await;
            trace!("Room[{}]: got the lock: `self.teams`", self.name());

            let side = {
                let mut all_side = {
                    let mut all_side = HashMap::new();
                    for team in teams_guard.iter() {
                        let team_name = team.name().to_string();
                        if config.name == team_name {
                            debug!("Room[{}]: team {} already exists", self.name(), team_name);
                            return RoomNameOccupiedSnafu {
                                room_name: self.name().to_string(),
                                team_name: team_name.to_string(),
                            }.fail();
                        }
                        all_side.insert(team.side(), team.name().to_string());
                    }
                    all_side
                };

                trace!("Room[{}]: all_sides: {:?}", self.name(), all_side);

                if all_side.len() >= Self::NUM_TEAMS { // racing happened
                    warn!("Room[{}]: is Full, cannot add team {}", self.name(), config.name);
                    return RoomIsFullSnafu {
                        room_name: self.name().to_string(),
                        pending_team: config.name,
                    }.fail();
                }

                match side {
                    Some(side) => {
                        if let Some(occupied_team) = all_side.remove(&side) {
                            return RoomSideOccupiedSnafu {
                                room_name: self.name().to_string(),
                                pending_team: config.name,
                                occupied_team,
                                target_side: side,
                            }.fail();
                        }
                        side
                    }
                    None => {
                        all_side.into_iter().next().expect("No side available").0
                    }
                }
            };

            let team_name = config.name.clone();

            debug!("Room[{}]: adding team {} to side {}", self.name(), team_name, side);

            teams_guard.insert(
                team_name.clone(),
                Team::new(side, config),
            );

            trace!("Room[{}]: `self.teams` lock released", self.name());

            team_name
        }; // [self.teams] WRITE RELEASE

        Ok(team_name)
    }
    
    pub async fn with_team<R: Send + 'static>(&self, team_name: &str, f: impl AsyncFn(Option<Ref<String, Team>>) -> R) -> R {
        let teams_guard = self.teams.read().await;
        f(teams_guard.get(team_name)).await
    }
}