use std::str::FromStr;

use arcstr::{ArcStr, format};
use common::types;
use crate::coach::command::CommandKind;

pub struct CommandMove {
    pub todo: (),
}

impl super::Command for CommandMove {
    type Ok = ();
    type Error = CommandMoveError;

    fn kind(&self) -> CommandKind {
        CommandKind::Move
    }

    fn encode(&self) -> ArcStr {
        todo!()
    }

    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> {
        tokens.is_empty().then_some(())
    }

    fn parse_ret_err(tokens: &[&str]) -> Option<Self::Error> {
        todo!("really complex too")
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CommandMoveError {
    #[error("The specified mode was not valid.")]
    IllegalMode,
    #[error("The PLAY_MODE argument was omitted")]
    IllegalCommandForm,
}

impl FromStr for CommandMoveError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandMoveError as FromStr>::Err> {
        match s {
            "illegal_mode" => Ok(Self::IllegalMode),
            "illegal_command_form" => Ok(Self::IllegalCommandForm),
            _ => Err(()),
        }
    }
}
