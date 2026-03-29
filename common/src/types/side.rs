use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(i8)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    LEFT = 1,
    NEUTRAL = 0,
    RIGHT = -1
}
