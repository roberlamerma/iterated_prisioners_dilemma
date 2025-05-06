use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;
use rand::Rng;

pub struct GenerousTitForTatStrategy {
    cooperation_probability: f64,
}

impl GenerousTitForTatStrategy {
    pub fn new() -> Self {
        Self {
            cooperation_probability: 0.1, // 10% chance of cooperating even after defection
        }
    }
}

impl Strategy for GenerousTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        match opponent_history.last() {
            Some(&Move::Defect) => {
                // With some probability, cooperate even after defection
                //if rand::thread_rng().gen::<f64>() < self.cooperation_probability {
                if rand::rng().random_bool(self.cooperation_probability) {
                    Move::Cooperate
                } else {
                    Move::Defect
                }
            },
            Some(&Move::Cooperate) => Move::Cooperate,
            None => Move::Cooperate, // First move
        }
    }
}

impl fmt::Display for GenerousTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generous Tit for Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Generous Tit for Tat",
        aliases: &["generoustitfortat", "gtft"],
        description: "A strategy that mimics the opponent's last move, but with a small probability of cooperating even after defection.",
        constructor: || Box::new(GenerousTitForTatStrategy::new()),
    }
} 