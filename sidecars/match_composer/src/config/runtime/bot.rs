use std::path::PathBuf;

use common::types::Side;

use super::{ImageConfig, PlayerProcessConfig, ServerConfig};

#[derive(Debug, Clone)]
pub struct BotConfig {
    pub unum: u8,
    pub side: Side,
    pub team: String,
    pub goalie: bool,
    pub image: ImageConfig,
    pub server: ServerConfig,
    pub log_path: PathBuf,
}

impl BotConfig {
    pub fn player(&self) -> PlayerProcessConfig {
        PlayerProcessConfig {
            host: self.server.host,
            port: self.server.port,
            unum: self.unum,
            goalie: self.goalie,
            team_name: self.team.clone(),
            log_path: self.log_path.clone(),
        }
    }
}
