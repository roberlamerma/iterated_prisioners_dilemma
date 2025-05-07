use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct SoftMajorityStrategy;

impl Strategy for SoftMajorityStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        let coop_count = opponent_history.iter().filter(|&&m| m == Move::Cooperate).count();
        let total = opponent_history.len();

        if total == 0 || (coop_count as f64 / total as f64) > 0.5 {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }
}

impl fmt::Display for SoftMajorityStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Soft Majority")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Soft Majority",
        aliases: &["softmajority", "soft"],
        description: "Cooperates if the opponent has cooperated more than half the time.",
        constructor: || Box::new(SoftMajorityStrategy),
    }
}
