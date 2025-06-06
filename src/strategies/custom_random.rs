use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use rand::Rng;
use std::fmt;

pub struct CustomRandomStrategy {
    cooperate_probability: f64,
}

impl CustomRandomStrategy {
    pub fn new() -> Self {
        Self {
            cooperate_probability: 0.5, // Default 50% chance to cooperate
        }
    }
}

impl Strategy for CustomRandomStrategy {
    fn next_move(&mut self, _own_history: &[Move], _opponent_history: &[Move]) -> Move {
        if rand::rng().random_bool(self.cooperate_probability) {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(prob) = params.get("cooperate_probability") {
            if let Some(prob) = prob.as_f64() {
                if prob >= 0.0 && prob <= 1.0 {
                    self.cooperate_probability = prob;
                    return Ok(());
                }
            }
        }
        Err("Invalid parameters. Expected 'cooperate_probability' as a number between 0 and 1".to_string())
    }
}

impl fmt::Display for CustomRandomStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom Random (cooperate_probability: {})", self.cooperate_probability)
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Custom Random",
        aliases: &["customrandom", "crand"],
        description: "Returns Cooperate with a configurable probability, otherwise Defect. When the parameter 'cooperate_probability' is set to 50% (0.5), it behaves like the 'Random' strategy.",
        constructor: || Box::new(CustomRandomStrategy::new()),
        supports_parameters: true,
    }
}
