use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;
use rand::Rng;
use serde_json;


pub struct GenerousTitForTatStrategy {
    pub cooperation_probability: f64,
}

impl GenerousTitForTatStrategy {
    pub fn new() -> Self {
        Self {
            cooperation_probability: 0.1, // Default 10% chance of cooperating even after defection
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

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(prob) = params.get("cooperation_probability") {
            if let Some(prob) = prob.as_f64() {
                if prob >= 0.0 && prob <= 1.0 {
                    self.cooperation_probability = prob;
                    return Ok(());
                }
            }
        }
        Err("Invalid parameters. Expected 'cooperation_probability' as a number between 0 and 1".to_string())
    }
}

impl fmt::Display for GenerousTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generous Tit for Tat (cooperation_probability: {})", self.cooperation_probability)
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Generous Tit for Tat",
        aliases: &["generoustitfortat", "gtft"],
        description: "A strategy that mimics the opponent's last move, but with a small probability of cooperating even after defection.",
        constructor: || Box::new(GenerousTitForTatStrategy::new()),
        supports_parameters: true,
    }
}