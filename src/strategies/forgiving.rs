use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct ForgivingStrategy {
    forgiveness_timer: u8,
    forgiveness_threshold: u8,
}

impl ForgivingStrategy {
    pub fn new(forgiveness_threshold: u8) -> Self {
        ForgivingStrategy {
            forgiveness_timer: 0,
            forgiveness_threshold,
        }
    }
}

impl Strategy for ForgivingStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        if opponent_history.is_empty() {
            return Move::Cooperate;
        }

        let last_opponent_move = opponent_history.last().unwrap();

        match last_opponent_move {
            Move::Defect => {
                if self.forgiveness_timer == 0 {
                    self.forgiveness_timer = self.forgiveness_threshold;
                    Move::Defect
                } else {
                    self.forgiveness_timer -= 1;
                    Move::Cooperate
                }
            }
            Move::Cooperate => {
                self.forgiveness_timer = 0;
                Move::Cooperate
            }
        }
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(threshold) = params.get("forgiveness_threshold") {
            if let Some(threshold) = threshold.as_u64() {
                if threshold > 0 && threshold <= u8::MAX as u64 {
                    self.forgiveness_threshold = threshold as u8;
                    return Ok(());
                }
            }
        }
        Err("Invalid parameters. Expected 'forgiveness_threshold' as a positive integer between 1 and 255".to_string())
    }
}

impl fmt::Display for ForgivingStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Forgiving (forgives after {} rounds)",
            self.forgiveness_threshold
        )
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Forgiving",
        aliases: &["forgives", "forgive"],
        description: "Defects after a defection, but returns to cooperation after a set number of rounds. Encourages reconciliation.",
        constructor: || Box::new(ForgivingStrategy::new(3)), // Default forgiveness threshold of 3 rounds
        supports_parameters: true,
    }
}
