use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use rand::Rng;
use serde_json;
use std::fmt;

pub struct ImperfectTitForTatStrategy {
    imitate_probability: f64,
}

impl ImperfectTitForTatStrategy {
    pub fn new() -> Self {
        // Default imitate probability is set to 0.9
        ImperfectTitForTatStrategy {
            imitate_probability: 0.9,
        }
    }
}

impl Strategy for ImperfectTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        if opponent_history.is_empty() {
            return Move::Cooperate;
        }
        let last_opponent_move = opponent_history.last().unwrap();
        if rand::rng().random_bool(self.imitate_probability) {
            *last_opponent_move
        } else {
            match last_opponent_move {
                Move::Cooperate => Move::Defect,
                Move::Defect => Move::Cooperate,
            }
        }
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(prob) = params.get("imitate_probability") {
            if let Some(prob) = prob.as_f64() {
                if prob >= 0.0 && prob <= 1.0 {
                    self.imitate_probability = prob;
                    return Ok(());
                }
            }
        }
        Err("Invalid parameters. Expected 'imitate_probability' as a number between 0 and 1".to_string())
    }
}

impl fmt::Display for ImperfectTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Imperfect Tit For Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "ImperfectTitForTat",
        aliases: &["imperfect_tft", "imperfect_tit_for_tat"],
        description: "Imitates opponent's last move with a given (configurable) probability.",
        constructor: || Box::new(ImperfectTitForTatStrategy::new()),
        supports_parameters: true,
    }
}
