use std::str::FromStr;

use arcstr::{ArcStr, literal};
use crate::coach::command::CommandKind;

pub struct CommandLook;
impl super::Command for CommandLook {
    type Ok = ();
    type Error = CommandLookError;

    fn kind(&self) -> CommandKind {
        CommandKind::Look
    }

    fn encode(&self) -> ArcStr {
        literal!("(look)")
    }

    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> {
        todo!("really complex to implement")
    }

    // never error
}

#[derive(thiserror::Error, Debug)]
pub enum CommandLookError {}

impl FromStr for CommandLookError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandLookError as FromStr>::Err> {
        match s {
            _ => Err(()),
        }
    }
}
