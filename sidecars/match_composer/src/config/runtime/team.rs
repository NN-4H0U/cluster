use std::path::PathBuf;

use common::types::Side;

use super::{PlayerConfig, ServerConfig};
use crate::config::schema::TeamSchema;

#[derive(Clone, Debug)]
pub struct TeamConfig {
    pub name: String,
    pub side: Side,
    pub players: Vec<PlayerConfig>,
}

impl TeamConfig {
    pub fn from_schema(
        team: TeamSchema,
        side: Side,
        server: &ServerConfig,
        log_root: &PathBuf,
    ) -> Result<Self, String> {
        let players = team
            .players
            .iter()
            .map(|p| {
                let log_path = log_root.join(format!("player_{:02}.log", p.unum));
                PlayerConfig::from_schema(p, &team.name, side, server, &log_path)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TeamConfig {
            name: team.name,
            side,
            players,
        })
    }
}
