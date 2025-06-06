use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct ReverseTitForTatStrategy;

impl Strategy for ReverseTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        
        match opponent_history.last() {
            Some(&last_move) => 
            match last_move == Move::Defect {
                true => Move::Cooperate,
                false => Move::Defect,
            }
            ,
            None => Move::Defect, // First move
        }
    }
}

impl fmt::Display for ReverseTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reverse Tit for Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Reverse Tit for Tat",
        aliases: &["reversetitfortat", "bully", "rtft"],
        description: "Starts by defecting and then does the opposite of opponent's previous move. This is the complete opposite of Tit For Tat.",
        constructor: || Box::new(ReverseTitForTatStrategy),
        supports_parameters: false,
    }
}