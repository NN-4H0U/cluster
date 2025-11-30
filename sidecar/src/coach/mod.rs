mod coach;
pub mod signal;
mod error;
mod builder;
mod resolver;
mod addon;

use common::client;

pub use coach::OfflineCoach;
pub use coach::OfflineCoach as Trainer;
pub use error::{Error, Result};
pub use builder::OfflineCoachBuilder as Builder;
