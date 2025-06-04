use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct WinStayLoseShiftStrategy;

impl Strategy for WinStayLoseShiftStrategy {
    fn next_move(&mut self, own_history: &[Move], opponent_history: &[Move]) -> Move {
        // For the first move, cooperate
        if own_history.is_empty() {
            return Move::Cooperate;
        }

        // Get the last moves
        let last_own = own_history.last().unwrap();
        let last_opponent = opponent_history.last().unwrap();

        // If both players made the same move (win), stay with the same move
        // If moves were different (lose), switch to the opposite move
        if last_own == last_opponent {
            *last_own
        } else {
            match last_own {
                Move::Cooperate => Move::Defect,
                Move::Defect => Move::Cooperate,
            }
        }
    }
}

impl fmt::Display for WinStayLoseShiftStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Win-Stay, Lose-Shift (Pavlov)")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Win-Stay, Lose-Shift",
        aliases: &["winstayloseshift", "pavlov", "wsls"],
        description: "A strategy that repeats its last move if both players made the same choice, otherwise switches to the opposite move.",
        constructor: || Box::new(WinStayLoseShiftStrategy),
        supports_parameters: false,
    }
} 