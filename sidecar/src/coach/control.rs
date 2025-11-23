use std::borrow::Cow;
use common::types;

pub enum ControlMessage {
    ChangeMode {
        play_mode: types::PlayMode,
    },
    Move {
        todo: (),
    },
    CheckBall,
    Start,
    Recover,
    Ear {
        mode: types::EarMode,
    },
    Init {
        version: usize,
    },
    Look,
    Eye {
        mode: types::EyeMode,
    },
    TeamNames,
}

impl ControlMessage {
    pub fn encode(&self) -> Cow<'static, str> {
        use ControlMessage::*;
        match self {
            ChangeMode { play_mode } => {
                Cow::Owned(format!("(change_mode {})", play_mode.encode()))
            },
            Move { .. } => {
                todo!()
            }
            CheckBall => Cow::Borrowed("(check_ball)"),
            Start => Cow::Borrowed("(start)"),
            Recover => Cow::Borrowed("(recover)"),
            Ear { mode } => {
                Cow::Owned(format!("(ear {})", mode.encode()))
            },
            Init { version } => {
                Cow::Owned(format!("(init version {})", version))
            },
            Look => Cow::Borrowed("(look)"),
            Eye { mode } => {
                Cow::Owned(format!("(eye {})", mode.encode()))
            },
            TeamNames => Cow::Borrowed("(team_names)"),
        }
    }
}
