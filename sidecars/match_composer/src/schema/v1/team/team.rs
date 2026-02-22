use serde::{Deserialize, Serialize};
use crate::schema::v1::{Schema, PlayerV1};
use super::TeamSideV1;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TeamV1 {
    pub name: String,
    pub side: TeamSideV1,
    pub players: Vec<PlayerV1>,
}

impl Schema for TeamV1 {
    fn verify(&self) -> Result<(), &'static str> {
        if self.name.is_empty() {
            return Err("Team name cannot be empty")
        }

        if !self.name.is_ascii() {
            return Err("Team name cannot contain non-ASCII characters")
        }

        if self.name.len() > 16 {
            return Err("Team name cannot be longer than 16 characters")
        }

        if self.players.len() > 11 {
            return Err("Team cannot have more than 11 players")
        }

        for player in self.players.iter() {
            player.verify()?;
        }

        Ok(())
    }
}