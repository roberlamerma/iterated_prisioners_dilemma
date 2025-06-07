use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct GradualTitForTatStrategy {
    retaliation: usize,
    retaliation_count: usize,
    apology_count: usize,
    last_opponent_defections: usize,
}

impl GradualTitForTatStrategy {
    pub fn new() -> Self {
        Self {
            retaliation: 0,
            retaliation_count: 0,
            apology_count: 0,
            last_opponent_defections: 0,
        }
    }
}

// Implements Gradual Tit For Tat: punishes incrementally, then apologizes with two cooperations
impl Strategy for GradualTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        let defections = opponent_history.iter().filter(|&&m| m == Move::Defect).count();

        // If in apology phase, cooperate for two rounds
        if self.apology_count > 0 {
            self.apology_count -= 1;
            return Move::Cooperate;
        }

        // If in retaliation phase, defect
        if self.retaliation_count < self.retaliation {
            self.retaliation_count += 1;
            // If this is the last retaliation, set up apology phase
            if self.retaliation_count == self.retaliation {
                self.apology_count = 2;
            }
            return Move::Defect;
        }

        // If new defections detected, set up new retaliation
        if defections > self.last_opponent_defections {
            let new_defections = defections - self.last_opponent_defections;
            self.retaliation = self.retaliation + new_defections;
            self.last_opponent_defections = defections;
            self.retaliation_count = 0;
            // Start retaliation
            if self.retaliation > 0 {
                self.retaliation_count += 1;
                if self.retaliation_count == self.retaliation {
                    self.apology_count = 2;
                }
                return Move::Defect;
            }
        }

        // Otherwise, cooperate
        self.retaliation = 0;
        self.retaliation_count = 0;
        Move::Cooperate
    }
}

impl fmt::Display for GradualTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gradual Tit For Tat")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Gradual Tit For Tat",
        aliases: &["gradual", "gradual_tft", "grad_tft"],
        description: "A strategy that retaliates gradually based on the opponent's defections, then apologizes with two cooperations.",
        constructor: || Box::new(GradualTitForTatStrategy::new()),
        supports_parameters: false,
    }
}