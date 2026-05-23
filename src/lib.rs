pub mod strategies;

use std::fmt;
use serde_json;

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum Move {
    Cooperate,
    Defect
}

#[derive(Clone, Copy, Debug)]
pub struct Payoffs {
    pub temptation: i32, // T: payoff for defecting while the opponent cooperates
    pub reward:     i32, // R: payoff for mutual cooperation
    pub punishment: i32, // P: payoff for mutual defection
    pub sucker:     i32, // S: payoff for cooperating while the opponent defects
}

impl Default for Payoffs {
    fn default() -> Self {
        Self { temptation: 5, reward: 3, punishment: 1, sucker: 0 }
    }
}

impl Payoffs {
    pub fn validate(&self) -> Result<(), String> {
        if !(self.sucker < self.punishment
            && self.punishment < self.reward
            && self.reward < self.temptation)
        {
            return Err(format!(
                "Invalid payoffs: require SUCKER ({}) < PUNISHMENT ({}) < REWARD ({}) < TEMPTATION ({})",
                self.sucker, self.punishment, self.reward, self.temptation
            ));
        }
        if 2 * self.reward <= self.temptation + self.sucker {
            return Err(format!(
                "Invalid payoffs for the iterated PD: require 2*REWARD ({}) > TEMPTATION ({}) + SUCKER ({}) = {}",
                2 * self.reward, self.temptation, self.sucker, self.temptation + self.sucker
            ));
        }
        Ok(())
    }
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

    // Allow strategies that depend on payoff values (e.g. Generous Tit-for-Tat,
    // Adaptive) to receive the active Payoffs before the simulation starts.
    fn set_payoffs(&mut self, _payoffs: Payoffs) {
        // Default implementation does nothing
    }
}

pub fn calculate_payoffs(move1: Move, move2: Move, p: &Payoffs) -> (i32, i32) {
    match (move1, move2) {
        (Move::Cooperate, Move::Cooperate) => (p.reward,     p.reward),     // Mutual cooperation
        (Move::Cooperate, Move::Defect)    => (p.sucker,     p.temptation), // Sucker's payoff / Temptation
        (Move::Defect,    Move::Cooperate) => (p.temptation, p.sucker),     // Temptation / Sucker's payoff
        (Move::Defect,    Move::Defect)    => (p.punishment, p.punishment), // Mutual defection
    }
}