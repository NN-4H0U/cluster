use std::str::FromStr;

use arcstr::{ArcStr, format};
use common::types;

use super::CommandKind;

pub struct CommandChangeMode {
    pub play_mode: types::PlayMode,
}

impl super::Command for CommandChangeMode {
    type Ok = ();
    type Error = CommandChangeModeError;

    fn kind(&self) -> CommandKind {
        CommandKind::ChangeMode
    }
    
    fn encode(&self) -> ArcStr {
        format!("({} {})", self.kind().encode(), self.play_mode.encode())
    }

    fn parse_ret_err(tokens: &[&str]) -> Option<Self::Error> {
        if tokens.len() != 1 { return None }
        let tokens = tokens.join(" ");
        if tokens.is_empty() { return None }

        tokens.parse().ok()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CommandChangeModeError {
    #[error("The specified mode was not valid.")]
    IllegalMode,
    #[error("The PLAY_MODE argument was omitted")]
    IllegalCommandForm,
}

impl FromStr for CommandChangeModeError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandChangeModeError as FromStr>::Err> {
        match s {
            "illegal_mode" => Ok(Self::IllegalMode),
            "illegal_command_form" => Ok(Self::IllegalCommandForm),
            _ => Err(()),
        }
    }
}
