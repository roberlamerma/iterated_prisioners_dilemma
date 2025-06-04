use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct GrimTriggerStrategy {
    triggered: bool,
}

impl GrimTriggerStrategy {
    pub fn new() -> Self {
        GrimTriggerStrategy { triggered: false }
    }
}

impl Strategy for GrimTriggerStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        if opponent_history.iter().any(|&m| m == Move::Defect) {
            self.triggered = true;
        }

        if self.triggered {
            Move::Defect
        } else {
            Move::Cooperate
        }
    }
}

impl fmt::Display for GrimTriggerStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grim Trigger")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Grim Trigger",
        aliases: &["grim", "grimtrigger"],
        description: "Cooperates until the opponent defects once, then always defects.",
        constructor: || Box::new(GrimTriggerStrategy::new()),
        supports_parameters: false,
    }
}