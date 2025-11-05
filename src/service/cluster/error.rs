use std::backtrace::Backtrace;
use uuid::Uuid;
use crate::service::room;

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Cluster: Room[{room_id}] not found."))]
    RoomNotFound {
        room_id: Uuid,
    },
    
    #[snafu(transparent)]
    Room {
        source: room::Error,
        backtrace: Backtrace,
    }
}

pub type Result<T> = std::result::Result<T, Error>;