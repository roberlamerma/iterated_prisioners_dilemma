use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct FirmMajorityStrategy;

impl Strategy for FirmMajorityStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        let coop_count = opponent_history.iter().filter(|&&m| m == Move::Cooperate).count();
        let total = opponent_history.len();

        if total == 0 || (coop_count as f64 / total as f64) > 0.75 {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }
}

impl fmt::Display for FirmMajorityStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Firm Majority")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Firm Majority",
        aliases: &["firmmajority", "firm"],
        description: "Defects unless the opponent has cooperated more than 75% of the time.",
        constructor: || Box::new(FirmMajorityStrategy),
    }
}
