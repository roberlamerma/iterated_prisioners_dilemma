use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct MajorityStrategy {
    pub cooperation_threshold: f64,
}

impl MajorityStrategy {
    pub fn new() -> Self {
        Self {
            cooperation_threshold: 0.25, // Default 25% cooperation threshold
        }
    }
}

impl Strategy for MajorityStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        let coop_count = opponent_history.iter().filter(|&&m| m == Move::Cooperate).count();
        let total = opponent_history.len();

        if total == 0 || (coop_count as f64 / total as f64) > self.cooperation_threshold {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(prob) = params.get("cooperation_threshold") {
            if let Some(prob) = prob.as_f64() {
                if prob >= 0.0 && prob <= 1.0 {
                    self.cooperation_threshold = prob;
                    return Ok(());
                }
            }
        }
        Err("Invalid parameters. Expected 'cooperation_threshold' as a number between 0 and 1".to_string())
    }
}

impl fmt::Display for MajorityStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Majority (cooperation_threshold: {})", self.cooperation_threshold)
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Majority",
        aliases: &["majority"],
        description: "Cooperates as long as the opponent has cooperated more than the provided 'cooperation_threshold' (in decimal, i.g. 25% would be 0.25) of the time. Notice that if you provide a value of 50% (.50) this is the same as the Soft Majority strategy. Also, if you provide a value of 75% (.75) this is the same as the Firm Majority strategy.",
        constructor: || Box::new(MajorityStrategy::new()),
        supports_parameters: true,
    }
}
