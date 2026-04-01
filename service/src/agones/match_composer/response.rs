use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponse {
    pub in_match: bool,
}
