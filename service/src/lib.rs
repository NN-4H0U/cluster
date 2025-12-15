#[cfg(all(feature = "agones", feature = "standalone"))]
compile_error!(
    "Features 'agones' and 'standalone' are mutually exclusive. Please choose one to enable."
);
#[cfg(not(any(feature = "agones", feature = "standalone")))]
compile_error!("Either feature 'agones' or 'standalone' must be enabled.");

mod addons;
mod base;
pub mod error;
#[cfg(feature = "standalone")]
mod standalone;
#[cfg(feature = "agones")]
mod agones;

#[cfg(feature = "standalone")]
pub use standalone::StandaloneService as Service;

#[cfg(feature = "agones")]
pub use agones::AgonesService as Service;

pub use error::{Error, Result};
pub use base::ServerStatus;

pub const GAME_END_TIMESTEP: u16 = 6000;
