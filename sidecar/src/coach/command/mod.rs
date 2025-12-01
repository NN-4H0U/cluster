mod change_mode;
mod check_ball;
mod ear;
mod eye;
mod init;
mod look;
mod r#move;
mod recover;
mod start;
mod team_names;

use std::str::FromStr;
use std::any::Any;
use arcstr::{ArcStr, format, literal};

pub use change_mode::CommandChangeMode as ChangeMode;
pub use check_ball::CommandCheckBall as CheckBall;
pub use r#move::CommandMove as Move;
pub use ear::CommandEar as Ear;
pub use eye::CommandEye as Eye;
pub use init::CommandInit as Init;
pub use look::CommandLook as Look;
pub use recover::CommandRecover as Recover;
pub use start::CommandStart as Start;
pub use team_names::CommandTeamNames as TeamNames;


pub trait Command {
    type Ok: std::fmt::Debug + Send + 'static;
    type Error: std::error::Error + Send + 'static;
    
    fn kind(&self) -> CommandKind;
    fn encode(&self) -> ArcStr;
    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> where Self: Sized {
        None // default never ok
    }
    fn parse_ret_err(tokens: &[&str]) -> Option<Self::Error> where Self: Sized {
        None // default never error
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum CommandKind {
    ChangeMode,
    Move,
    CheckBall,
    Start,
    Recover,
    Ear,
    Init,
    Look,
    Eye,
    TeamNames,
}

impl CommandKind {
    pub fn encode(&self) -> ArcStr {
        match self {
            CommandKind::ChangeMode => literal!("change_mode"),
            CommandKind::Move       => literal!("move"),
            CommandKind::CheckBall  => literal!("check_ball"),
            CommandKind::Start      => literal!("start"),
            CommandKind::Recover    => literal!("recover"),
            CommandKind::Ear        => literal!("ear"),
            CommandKind::Init       => literal!("init"),
            CommandKind::Look       => literal!("look"),
            CommandKind::Eye        => literal!("eye"),
            CommandKind::TeamNames  => literal!("team_names"),
        }
    }

    pub fn decode(s: &str) -> Option<Self> {
        match s {
            "change_mode" => Some(CommandKind::ChangeMode),
            "move"        => Some(CommandKind::Move),
            "check_ball"  => Some(CommandKind::CheckBall),
            "start"       => Some(CommandKind::Start),
            "recover"     => Some(CommandKind::Recover),
            "ear"         => Some(CommandKind::Ear),
            "init"        => Some(CommandKind::Init),
            "look"        => Some(CommandKind::Look),
            "eye"         => Some(CommandKind::Eye),
            "team_names"  => Some(CommandKind::TeamNames),
            _             => None,
        }
    }

    pub fn parse_ret_ok(&self, tokens: &[&str]) -> Option<Box<dyn Any + Send>> {
        match self {
            CommandKind::ChangeMode => {
                ChangeMode::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Move => {
                Move::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::CheckBall => {
                CheckBall::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Start => {
                Start::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Recover => {
                Recover::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Ear => {
                Ear::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Init => {
                Init::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Look => {
                Look::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::Eye => {
                Eye::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            CommandKind::TeamNames => {
                TeamNames::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
        }
    }

    pub fn parse_ret_err(&self, tokens: &[&str]) -> Option<Box<dyn Any + Send>> {
        match self {
            CommandKind::ChangeMode => {
                ChangeMode::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Move => {
                Move::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::CheckBall => {
                CheckBall::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Start => {
                Start::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Recover => {
                Recover::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Ear => {
                Ear::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Init => {
                Init::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Look => {
                Look::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::Eye => {
                Eye::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            CommandKind::TeamNames => {
                TeamNames::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
        }
    }
}

impl FromStr for CommandKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <CommandKind as FromStr>::Err> {
        CommandKind::decode(s).ok_or(())
    }
}
