// Implements the CyclerStrategy: repeats a given sequence of moves (C/D) indefinitely.
use crate::{Strategy, Move};
use crate::strategies::StrategyInfo;
use serde_json::Value;
use std::fmt;

pub struct CyclerStrategy {
    sequence: Vec<Move>,
    index: usize,
}

impl CyclerStrategy {
    pub fn new() -> Self {
        CyclerStrategy {
            sequence: vec![Move::Cooperate], // Default: always cooperate
            index: 0,
        }
    }
}

impl Strategy for CyclerStrategy {
    fn next_move(&mut self, _own_history: &[Move], _opponent_history: &[Move]) -> Move {
        if self.sequence.is_empty() {
            return Move::Cooperate;
        }
        let themove = self.sequence[self.index];
        self.index = (self.index + 1) % self.sequence.len();
        themove
    }

    fn set_parameters(&mut self, params: Value) -> Result<(), String> {
        if let Some(seq) = params.get("sequence") {
            if let Some(seq_str) = seq.as_str() {
                let mut parsed = Vec::new();
                for c in seq_str.chars() {
                    match c {
                        'C' | 'c' => parsed.push(Move::Cooperate),
                        'D' | 'd' => parsed.push(Move::Defect),
                        _ => return Err(format!("Invalid character in sequence: {}", c)),
                    }
                }
                if parsed.is_empty() {
                    return Err("Sequence cannot be empty".to_string());
                }
                self.sequence = parsed;
                self.index = 0;
                Ok(())
            } else {
                Err("'sequence' must be a string of 'C' and 'D'".to_string())
            }
        } else {
            Err("Missing 'sequence' parameter".to_string())
        }
    }
}

impl fmt::Display for CyclerStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CyclerStrategy")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Cycler",
        aliases: &["cyc"],
        description: "Repeats a given sequence of C/D moves indefinitely. Sequence is set via parameters.",
        constructor: || Box::new(CyclerStrategy::new()),
        supports_parameters: true,
    }
}
