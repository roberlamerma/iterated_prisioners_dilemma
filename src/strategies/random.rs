use crate::{Move, Strategy};
use rand::Rng;
use std::fmt;

pub struct RandomStrategy;

impl Strategy for RandomStrategy {
    fn next_move(&mut self, _own_history: &[Move], _opponent_history: &[Move]) -> Move {
        if rand::rng().random_bool(0.5) {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }
}

impl fmt::Display for RandomStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Random")
    }
}