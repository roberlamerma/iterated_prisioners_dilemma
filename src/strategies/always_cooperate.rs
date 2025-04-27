use crate::{Move, Strategy};
use std::fmt;

pub struct AlwaysCooperateStrategy;

impl Strategy for AlwaysCooperateStrategy {
    fn next_move(&mut self, _own_history: &[Move], _opponent_history: &[Move]) -> Move {
        Move::Cooperate
    }
}

impl fmt::Display for AlwaysCooperateStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Always Cooperate")
    }
}