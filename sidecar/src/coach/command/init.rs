use std::str::FromStr;

use arcstr::{ArcStr, literal, format};
use crate::coach::command::CommandKind;

pub struct CommandInit {
    pub version: Option<u8>,
}

impl super::Command for CommandInit {
    type Ok = ();
    type Error = CommandInitError;

    fn kind(&self) -> CommandKind {
        CommandKind::Init
    }

    fn encode(&self) -> ArcStr {
        if let Some(version) = self.version {
            format!("(init {})", version)
        } else {
            literal!("(init)")
        }
    }

    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> {
        tokens.is_empty().then_some(())
    }
    
    // never error
}

#[derive(thiserror::Error, Debug)]
pub enum CommandInitError {}

impl FromStr for CommandInitError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandInitError as FromStr>::Err> {
        match s {
            _ => Err(()),
        }
    }
}
