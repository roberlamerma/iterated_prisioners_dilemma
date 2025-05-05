use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct TitForTatStrategy;

impl Strategy for TitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        
        match opponent_history.last() {
            Some(&last_move) => last_move,
            None => Move::Cooperate, // First move
        }
    }
}

impl fmt::Display for TitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tit for Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Tit for Tat",
        aliases: &["tft"],
        description: "A strategy that mimics the opponent's last move.",
        constructor: || Box::new(TitForTatStrategy),
    }
}