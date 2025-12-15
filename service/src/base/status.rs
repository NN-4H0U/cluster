#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ServerStatus {
    Uninitialized,
    Idle,
    Simulating,
    Finished,
}

impl ServerStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, ServerStatus::Simulating)
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, ServerStatus::Idle)
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, ServerStatus::Uninitialized)
    }
}

impl From<ServerStatus> for u8 {
    fn from(val: ServerStatus) -> Self {
        val as u8
    }
}

impl TryFrom<u8> for ServerStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ServerStatus::Uninitialized),
            1 => Ok(ServerStatus::Idle),
            2 => Ok(ServerStatus::Simulating),
            3 => Ok(ServerStatus::Finished),
            _ => Err(()),
        }
    }
}
