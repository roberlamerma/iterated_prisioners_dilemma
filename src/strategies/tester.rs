use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

use super::tit_for_tat::TitForTatStrategy;

pub struct TesterStrategy {
    turn: usize,
    fallback: TitForTatStrategy,
    opponent_defected: bool,
}

impl TesterStrategy {
    pub fn new() -> Self {
        Self {
            turn: 0,
            fallback: TitForTatStrategy,
            opponent_defected: false,
        }
    }
}

impl Strategy for TesterStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        self.turn += 1;

        match self.turn {
            1 => Move::Defect, // Test opponent
            2 => Move::Cooperate,
            _ => {
                if opponent_history.len() >= 2 && opponent_history[1] == Move::Defect {
                    self.opponent_defected = true;
                }

                if self.opponent_defected {
                    Move::Defect
                } else {
                    self.fallback.next_move(_own_history, opponent_history)
                }
            }
        }
    }
}

impl fmt::Display for TesterStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tester")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Tester",
        aliases: &["test"],
        description: "A strategy that tests the opponent's response and then cooperates or defects based on their behavior.",
        constructor: || Box::new(TesterStrategy::new()),
    }
}