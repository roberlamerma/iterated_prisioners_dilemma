use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use rand::Rng;
use std::fmt;

pub struct StochasticTitForTatStrategy {
    defection_chance: f64,
}

impl StochasticTitForTatStrategy {
    pub fn new() -> Self {
        Self {
            defection_chance: 0.1, // 10% chance to defect even after cooperation
        }
    }
}

impl Strategy for StochasticTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {

        match opponent_history.last() {
            Some(&last_move) => {
                if rand::rng().random_bool(self.defection_chance) {
                    Move::Defect
                } else {
                    last_move
                }
            }
            None => Move::Cooperate, // First move
        }
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(chance) = params.get("defection_chance") {
            if let Some(chance) = chance.as_f64() {
                if chance >= 0.0 && chance <= 1.0 {
                    self.defection_chance = chance;
                    return Ok(());
                }
            }
        }
        Err("Invalid parameters. Expected 'defection_chance' as a number between 0 and 1".to_string())
    }
}

impl fmt::Display for StochasticTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stochastic Tit for Tat (defection_chance: {})", self.defection_chance)
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Stochastic Tit for Tat",
        aliases: &["stochastictitfortat", "sto_tft"],
        description: "Like Tit for Tat, but sometimes defects at random (e.g. 10% of the time), even after cooperation.",
        constructor: || Box::new(StochasticTitForTatStrategy::new()),
        supports_parameters: true,
    }
}
