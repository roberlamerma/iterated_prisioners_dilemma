use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct TitForTwoTatsStrategy;

/// It cooperates unless the opponent has defected twice in the last two rounds.
impl Strategy for TitForTwoTatsStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        let len = opponent_history.len();
        if len > 2 && opponent_history[len - 1] == Move::Defect && opponent_history[len - 2] == Move::Defect {
            Move::Defect
        } else {
            Move::Cooperate
        }
        
    }
}

impl fmt::Display for TitForTwoTatsStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tit for 2 Tats")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Tit for Two Tats",
        aliases: &["titfortwotats", "tf2t", "tftt"],
        description: "Cooperates unless the opponent has defected twice in the last two rounds.",
        constructor: || Box::new(TitForTwoTatsStrategy),
        supports_parameters: false,
    }
}