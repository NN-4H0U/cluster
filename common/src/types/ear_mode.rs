#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum EarMode {
    On, Off
}
impl EarMode {
    pub fn encode(self) -> &'static str {
        match self {
            EarMode::On => "on",
            EarMode::Off => "off"
        }
    }
}
