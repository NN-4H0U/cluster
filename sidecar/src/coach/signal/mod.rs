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

pub use change_mode::SignalChangeMode as ChangeMode;
pub use check_ball::SignalCheckBall as CheckBall;
pub use r#move::SignalMove as Move;
pub use ear::SignalEar as Ear;
pub use eye::SignalEye as Eye;
pub use init::SignalInit as Init;
pub use look::SignalLook as Look;
pub use recover::SignalRecover as Recover;
pub use start::SignalStart as Start;
pub use team_names::SignalTeamNames as TeamNames;


pub trait Signal {
    type Ok: std::fmt::Debug + Send + 'static;
    type Error: std::error::Error + Send + 'static;
    
    fn kind(&self) -> SignalKind;
    fn encode(&self) -> ArcStr;
    fn parse_ret_ok(tokens: &[&str]) -> Option<Self::Ok> where Self: Sized {
        None // default never ok
    }
    fn parse_ret_err(tokens: &[&str]) -> Option<Self::Error> where Self: Sized {
        None // default never error
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum SignalKind {
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

impl SignalKind {
    pub fn encode(&self) -> ArcStr {
        match self {
            SignalKind::ChangeMode => literal!("change_mode"),
            SignalKind::Move       => literal!("move"),
            SignalKind::CheckBall  => literal!("check_ball"),
            SignalKind::Start      => literal!("start"),
            SignalKind::Recover    => literal!("recover"),
            SignalKind::Ear        => literal!("ear"),
            SignalKind::Init       => literal!("init"),
            SignalKind::Look       => literal!("look"),
            SignalKind::Eye        => literal!("eye"),
            SignalKind::TeamNames  => literal!("team_names"),
        }
    }

    pub fn decode(s: &str) -> Option<Self> {
        match s {
            "change_mode" => Some(SignalKind::ChangeMode),
            "move"        => Some(SignalKind::Move),
            "check_ball"  => Some(SignalKind::CheckBall),
            "start"       => Some(SignalKind::Start),
            "recover"     => Some(SignalKind::Recover),
            "ear"         => Some(SignalKind::Ear),
            "init"        => Some(SignalKind::Init),
            "look"        => Some(SignalKind::Look),
            "eye"         => Some(SignalKind::Eye),
            "team_names"  => Some(SignalKind::TeamNames),
            _             => None,
        }
    }

    pub fn parse_ret_ok(&self, tokens: &[&str]) -> Option<Box<dyn Any + Send>> {
        match self {
            SignalKind::ChangeMode => {
                ChangeMode::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Move => {
                Move::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::CheckBall => {
                CheckBall::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Start => {
                Start::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Recover => {
                Recover::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Ear => {
                Ear::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Init => {
                Init::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Look => {
                Look::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::Eye => {
                Eye::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
            SignalKind::TeamNames => {
                TeamNames::parse_ret_ok(tokens).map(|r| Box::new(r) as Box<dyn Any + Send>)
            },
        }
    }

    pub fn parse_ret_err(&self, tokens: &[&str]) -> Option<Box<dyn Any + Send>> {
        match self {
            SignalKind::ChangeMode => {
                ChangeMode::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Move => {
                Move::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::CheckBall => {
                CheckBall::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Start => {
                Start::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Recover => {
                Recover::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Ear => {
                Ear::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Init => {
                Init::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Look => {
                Look::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::Eye => {
                Eye::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
            SignalKind::TeamNames => {
                TeamNames::parse_ret_err(tokens).map(|e| Box::new(e) as Box<dyn Any + Send>)
            },
        }
    }
}

impl FromStr for SignalKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <SignalKind as FromStr>::Err> {
        SignalKind::decode(s).ok_or(())
    }
}
