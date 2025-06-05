pub mod strategies;

use std::fmt;
use serde_json;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum Move {
    Cooperate,
    Defect
}

// Defines the interface for any strategy
//pub trait Strategy: fmt::Display {
pub trait Strategy: fmt::Display + Send + Sync {

    // next move based on the historical moves.
    // `own_history` contains the all the strategy's past moves (from oldest to newest)
    // `opponent_history` contains the oponent's past moves (from oldest to newest)
    fn next_move(&mut self, own_history: &[Move], opponent_history: &[Move]) -> Move;

    // Reset needed?
    //fn reset(&mut self);

    // Add a method to set parameters
    fn set_parameters(&mut self, _params: serde_json::Value) -> Result<(), String> {
        // Default implementation does nothing
        Ok(())
    }
}

pub const REWARD: i32 = 3;
pub const TEMPTATION: i32 = 5;
pub const SUCKER: i32 = 0;
pub const PUNISHMENT: i32 = 1;

pub fn calculate_payoffs(move1: Move, move2: Move) -> (i32, i32) {
    match (move1, move2) {
        (Move::Cooperate, Move::Cooperate) => (REWARD,     REWARD),     // Mutual cooperation
        (Move::Cooperate, Move::Defect)    => (SUCKER,     TEMPTATION), // Sucker's payoff / Temptation
        (Move::Defect,    Move::Cooperate) => (TEMPTATION, SUCKER),     // Temptation / Sucker's payoff
        (Move::Defect,    Move::Defect)    => (PUNISHMENT, PUNISHMENT), // Mutual defection
    }
}