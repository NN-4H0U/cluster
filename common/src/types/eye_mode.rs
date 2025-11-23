#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum EyeMode {
    On, Off
}
impl EyeMode {
    pub fn encode(self) -> &'static str {
        match self {
            EyeMode::On => "on",
            EyeMode::Off => "off"
        }
    }
}
