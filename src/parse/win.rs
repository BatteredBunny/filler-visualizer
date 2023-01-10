use std::fmt::Display;

use bevy::{ecs::schedule::ShouldRun, prelude::Res};

use super::State;

#[derive(Debug, Default, Clone)]
pub struct Win {
    pub seed: String,
    pub player1_score: usize,
    pub player2_score: usize,
    pub winner: usize,

    pub start: Option<usize>,
    pub finished: bool,
}

impl Display for Win {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Seed: {}", self.seed)?;

        writeln!(f, "Player 1 score: {}", self.player1_score)?;
        writeln!(f, "Player 2 score: {}", self.player2_score)?;

        write!(f, "Player {} won!", self.winner)
    }
}

pub fn game_hasnt_finished(state: Res<State>) -> ShouldRun {
    if state.win.finished {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}
