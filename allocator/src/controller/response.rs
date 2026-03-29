use serde::{Deserialize, Serialize};
use crate::schema::v1;

#[derive(Debug, Deserialize)]
pub struct AllocateRequest {
    #[serde(flatten)]
    pub schema: v1::ConfigV1,
}

#[derive(Debug, Serialize)]
pub struct AllocateResponse {
    pub ip: String,
    pub port: u16,
    pub game_server_name: String,
}
