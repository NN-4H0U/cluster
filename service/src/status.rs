#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum ServiceStatus {
    Uninitialized,
    Idle,
    Simulating,
    Finished,
}

impl ServiceStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, ServiceStatus::Simulating)
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, ServiceStatus::Idle)
    }
}

impl From<ServiceStatus> for u8 {
    fn from(val: ServiceStatus) -> Self {
        val as u8
    }
}

impl TryFrom<u8> for ServiceStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ServiceStatus::Uninitialized),
            1 => Ok(ServiceStatus::Idle),
            2 => Ok(ServiceStatus::Simulating),
            3 => Ok(ServiceStatus::Finished),
            _ => Err(()),
        }
    }
}
