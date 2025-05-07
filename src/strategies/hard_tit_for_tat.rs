use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct HardTitForTatStrategy;

impl Strategy for HardTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        
        match opponent_history.last() {
            Some(&last_move) => last_move,
            None => Move::Defect, // First move
        }
    }
}

impl fmt::Display for HardTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hard Tit for Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Hard Tit for Tat",
        aliases: &["hardtitfortat", "h_tft"],
        description: "A strategy that mimics the opponent's last move. It starts defecting though.",
        constructor: || Box::new(HardTitForTatStrategy),
    }
}