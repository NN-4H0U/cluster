#[derive(Clone, Debug)]
pub enum ClientSignal {
    Shutdown,
}

impl ClientSignal {
    pub fn is_shutdown(&self) -> bool {
        matches!(self, ClientSignal::Shutdown)
    }
}
