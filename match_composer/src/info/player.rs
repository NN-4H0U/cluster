use serde::Serialize;

use crate::model::{PlayerKind};
use crate::declaration::{ImageDeclaration, Unum};

#[derive(Serialize, Debug, Clone)]
pub struct PlayerInfo {
    pub unum: Unum,
    pub kind: PlayerKind,
    pub image: ImageDeclaration,
}
