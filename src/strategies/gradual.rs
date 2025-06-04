use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct GradualStrategy {
    retaliation: usize,
    retaliation_count: usize,
}

impl GradualStrategy {
    pub fn new() -> Self {
        Self {
            retaliation: 0,
            retaliation_count: 0,
        }
    }
}

// Uses full opponent history to punish defection incrementally (e.g., defect more the more often you were defected against).
// It then returns to cooperation, allowing forgiveness
impl Strategy for GradualStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        let defections = opponent_history.iter().filter(|&&m| m == Move::Defect).count();

        if self.retaliation_count < self.retaliation {
            self.retaliation_count += 1;
            return Move::Defect;
        } else {
            self.retaliation = defections;
            self.retaliation_count = 0;
            return Move::Cooperate;
        }
    }
}

impl fmt::Display for GradualStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gradual")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Gradual",
        aliases: &["grad"],
        description: "A strategy that retaliates gradually based on the opponent's defections.",
        constructor: || Box::new(GradualStrategy::new()),
        supports_parameters: false,
    }
}