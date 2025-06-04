use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct SuspiciousTitForTatStrategy;

impl Strategy for SuspiciousTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        
        match opponent_history.last() {
            Some(&last_move) => last_move,
            None => Move::Defect, // First move
        }
    }
}

impl fmt::Display for SuspiciousTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Suspicious Tit for Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Suspicious Tit for Tat",
        aliases: &["suspicioustitfortat", "sus_tft"],
        description: "A strategy that starts with Defect and then mimics the opponent's last move.",
        constructor: || Box::new(SuspiciousTitForTatStrategy),
        supports_parameters: false,
    }
}