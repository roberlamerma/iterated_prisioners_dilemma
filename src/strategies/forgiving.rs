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
        constructor: || Box::new(ForgivingStrategy::new(3)), // Configure here the forgiveness threshold
    }
}
