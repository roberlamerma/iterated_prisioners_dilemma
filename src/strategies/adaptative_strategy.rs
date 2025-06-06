use crate::{Move, Strategy, calculate_payoffs};
use crate::strategies::StrategyInfo;
use std::fmt;

#[derive(Default)]
pub struct AdaptiveStrategy {
    initial_sequence: Vec<Move>,
    params_set: bool,
}

impl AdaptiveStrategy {
    pub fn new() -> Self {
        Self { initial_sequence: Vec::new(), params_set: false }
    }
}

impl Strategy for AdaptiveStrategy {
    fn next_move(&mut self, own_history: &[Move], opponent_history: &[Move]) -> Move {
        // Play initial sequence if available
        if own_history.len() < self.initial_sequence.len() {
            return self.initial_sequence[own_history.len()];
        }
        // If no history, default to Cooperate
        if own_history.is_empty() && self.initial_sequence.is_empty() {
            return Move::Cooperate;
        }
        // After initial sequence, pick the move (C or D) that has yielded the highest average payoff
        let mut c_total = 0;
        let mut c_count = 0;
        let mut d_total = 0;
        let mut d_count = 0;
        for (i, &m) in own_history.iter().enumerate() {
            if i >= opponent_history.len() { break; }
            let opp = opponent_history[i];
            let (my_payoff, _) = calculate_payoffs(m, opp);
            match m {
                Move::Cooperate => { c_total += my_payoff; c_count += 1; },
                Move::Defect => { d_total += my_payoff; d_count += 1; },
            }
        }
        let c_avg = if c_count > 0 { c_total as f64 / c_count as f64 } else { 0.0 };
        let d_avg = if d_count > 0 { d_total as f64 / d_count as f64 } else { 0.0 };
        // println!("C avg: {} (c_total/c_count -> {}/{}), D avg: {} (d_total/d_count -> {}/{})", c_avg, c_total, c_count, d_avg, d_total, d_count); // useful for debugging
        if c_avg >= d_avg { Move::Cooperate } else { Move::Defect }
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(seq) = params.get("initial_sequence") {
            if let Some(arr) = seq.as_array() {
                let mut moves = Vec::new();
                for v in arr {
                    match v.as_str() {
                        Some("C") => moves.push(Move::Cooperate),
                        Some("D") => moves.push(Move::Defect),
                        _ => return Err("Invalid move in initial_sequence (must be 'C' or 'D')".to_string()),
                    }
                }
                self.initial_sequence = moves;
                self.params_set = true;
            }
        }
        Ok(())
    }
}

impl fmt::Display for AdaptiveStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Adaptive Strategy")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Adaptive",
        aliases: &["adp"],
        description: "Starts with a given sequence, then plays the move (C or D) that has yielded the highest average payoff so far.",
        constructor: || Box::new(AdaptiveStrategy::new()),
        supports_parameters: true,
    }
}
