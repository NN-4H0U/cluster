use std::str::FromStr;

use arcstr::{ArcStr, literal};
use crate::coach::command::CommandKind;

pub struct CommandStart;
impl super::Command for CommandStart {
    type Ok = ();
    type Error = CommandStartError;

    fn kind(&self) -> CommandKind {
        CommandKind::Start
    }

    fn encode(&self) -> ArcStr {
        literal!("(start)")
    }

    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> {
        tokens.is_empty().then_some(())
    }

    // never error
}

#[derive(thiserror::Error, Debug)]
pub enum CommandStartError {}

impl FromStr for CommandStartError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandStartError as FromStr>::Err> {
        match s {
            _ => Err(()),
        }
    }
}
