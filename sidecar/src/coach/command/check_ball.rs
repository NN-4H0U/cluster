use std::str::FromStr;

use arcstr::{ArcStr, literal};
use common::types::BallPosition;

use super::CommandKind;

pub struct CommandCheckBall;
impl super::Command for CommandCheckBall {
    type Ok = (u16, BallPosition);
    type Error = CommandCheckBallError;

    fn kind(&self) -> CommandKind {
        CommandKind::CheckBall
    }
    fn encode(&self) -> ArcStr {
        literal!("(check_ball)")
    }

    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> where Self: Sized {
        if tokens.len() != 2 { return None }
        let time = tokens[0].parse::<u16>().ok()?;
        let position = tokens[1].parse::<BallPosition>().ok()?;
        Some((time, position))
    }

    // never error
}

#[derive(thiserror::Error, Debug)]
pub enum CommandCheckBallError {}

impl FromStr for CommandCheckBallError {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandCheckBallError as FromStr>::Err> {
        match s {
            _ => Err(()),
        }
    }
}
