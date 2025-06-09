use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct TwoTitsForTatsStrategy {
    defect_countdown: u8,
}

impl TwoTitsForTatsStrategy {
    pub fn new() -> Self {
        Self { defect_countdown: 0 }
    }
}

impl Strategy for TwoTitsForTatsStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        // If currently in countdown, defect and decrement
        if self.defect_countdown > 0 {
            self.defect_countdown -= 1;
            return Move::Defect;
        }
        // If opponent defected last round, start countdown
        if let Some(&Move::Defect) = opponent_history.last() {
            self.defect_countdown = 1; // Will defect this and next round
            return Move::Defect;
        }
        Move::Cooperate
    }
}

impl fmt::Display for TwoTitsForTatsStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Two Tit For Tats")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Two Tit For Tats",
        aliases: &["twotitfortats", "2tft", "ttft"],
        description: "Defects twice after being defected against, otherwise cooperates.",
        constructor: || Box::new(TwoTitsForTatsStrategy::new()),
        supports_parameters: false,
    }
}
