use std::net::SocketAddr;
use crate::room::RoomConfig;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Room[{room_name}] not found")]
    RoomNotFound {
        room_name: String,
    },
    
    #[error("Room[{room_name}] already dropped")]
    RoomDropped {
        room_name: String,
    },
    
    #[error(transparent)]
    Room(#[from] crate::room::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
