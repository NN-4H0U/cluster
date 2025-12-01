use std::str::FromStr;

use arcstr::{ArcStr, literal};
use common::types;
use crate::coach::command::CommandKind;

pub struct CommandRecover;

impl super::Command for CommandRecover {
    type Ok = ();
    type Error = CommandRecoverError;

    fn kind(&self) -> CommandKind {
        CommandKind::Recover
    }

    fn encode(&self) -> ArcStr {
        literal!("(recover)")
    }

    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> {
        tokens.is_empty().then_some(())
    }

    // never error
}

#[derive(thiserror::Error, Debug)]
pub enum CommandRecoverError {}

impl FromStr for CommandRecoverError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandRecoverError as FromStr>::Err> {
        match s {
            _ => Err(()),
        }
    }
}
