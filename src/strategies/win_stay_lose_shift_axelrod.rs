use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

pub struct WinStayLoseShiftStrategyAxelrod;

impl Strategy for WinStayLoseShiftStrategyAxelrod {
    fn next_move(&mut self, own_history: &[Move], opponent_history: &[Move]) -> Move {
        // For the first move, cooperate
        if own_history.is_empty() {
            return Move::Cooperate;
        }

        // Get the last moves
        let last_own = own_history.last().unwrap();
        let last_opponent = opponent_history.last().unwrap();

        match (last_own, last_opponent) {
            (Move::Cooperate, Move::Cooperate) => Move::Cooperate, // good → stay
            (Move::Defect, Move::Cooperate) => Move::Defect,       // good → stay
            (Move::Cooperate, Move::Defect) => Move::Defect,       // bad → switch
            (Move::Defect, Move::Defect) => Move::Cooperate,       // bad → switch
        }
    }
}

impl fmt::Display for WinStayLoseShiftStrategyAxelrod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Win-Stay, Lose-Shift (Pavlov). Axelrod's original version")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Win-Stay, Lose-Shift (Axelrod's original version)",
        aliases: &["winstayloseshiftoriginal", "pavlovoriginal", "wslso"],
        description: "If the last round's payoff was high (i.e. mutual cooperation or sucker's reward), repeat the same move; otherwise, switch.",
        constructor: || Box::new(WinStayLoseShiftStrategyAxelrod),
        supports_parameters: false,
    }
} 