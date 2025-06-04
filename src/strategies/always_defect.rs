use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct AlwaysDefectStrategy;

impl Strategy for AlwaysDefectStrategy {
    fn next_move(&mut self, _own_history: &[Move], _opponent_history: &[Move]) -> Move {
        Move::Defect
    }
}

impl fmt::Display for AlwaysDefectStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Always Defect")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Always Defect",
        aliases: &["alwaysdefect", "defect", "alld"],
        description: "A strategy that always defects.",
        constructor: || Box::new(AlwaysDefectStrategy),
        supports_parameters: false,
    }
}