use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientStateEnum {
    Idle = 0,
    WaitingRedirection = 1,
    Connected = 2,
    Disconnected = 3,
}

impl Default for ClientStateEnum {
    fn default() -> Self {
        ClientStateEnum::Disconnected
    }
}
impl From<u8> for ClientStateEnum {
    fn from(value: u8) -> Self {
        match value {
            0 => ClientStateEnum::Idle,
            1 => ClientStateEnum::WaitingRedirection,
            2 => ClientStateEnum::Connected,
            3 => ClientStateEnum::Disconnected,
            _ => panic!("Invalid ClientStateEnum"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientState(Arc<AtomicU8>);
impl ClientState {
    pub fn set(&self, status: ClientStateEnum) {
        self.0.store(status as u8, Ordering::Relaxed);
    }
    pub fn kind(&self) -> ClientStateEnum {
        ClientStateEnum::from(self.0.load(Ordering::Relaxed))
    }
}

impl Default for ClientState {
    fn default() -> Self {
        ClientState(Arc::new(AtomicU8::new(ClientStateEnum::default() as u8)))
    }
}

impl PartialEq<ClientStateEnum> for &ClientState {
    fn eq(&self, other: &ClientStateEnum) -> bool {
        self.kind() == *other
    }
}

impl PartialEq<ClientStateEnum> for ClientState {
    fn eq(&self, other: &ClientStateEnum) -> bool {
        self == other
    }
}