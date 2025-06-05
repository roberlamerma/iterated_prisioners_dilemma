use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::collections::HashMap;
use std::fmt;

#[derive(Hash, PartialEq, Eq)]
struct HistoryKey(Vec<Move>, Vec<Move>);

pub struct LookerUpStrategy {
    lookup_table: HashMap<HistoryKey, Move>,
    history_length: usize,
}

impl LookerUpStrategy {
    pub fn new() -> Self {
        Self {
            lookup_table: HashMap::new(),
            history_length: 0,
        }
    }
}

impl Strategy for LookerUpStrategy {
    fn next_move(&mut self, own_history: &[Move], opponent_history: &[Move]) -> Move {
        if own_history.len() < self.history_length || opponent_history.len() < self.history_length {
            return Move::Cooperate;
        }

        let own_recent = own_history[own_history.len() - self.history_length..].to_vec();
        let opp_recent = opponent_history[opponent_history.len() - self.history_length..].to_vec();

        let key = HistoryKey(own_recent, opp_recent);

        self.lookup_table
            .get(&key)
            .copied()
            .unwrap_or(Move::Cooperate)
    }

    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        // Expecting: { "history_length": N, "lookup_table": [ { "own": ["C",...], "opp": ["D",...], "move": "C" }, ... ] }
        let history_length = params.get("history_length")
            .and_then(|v| v.as_u64())
            .ok_or("Missing or invalid 'history_length'")? as usize;
        let table = params.get("lookup_table")
            .and_then(|v| v.as_array())
            .ok_or("Missing or invalid 'lookup_table'")?;
        let mut lookup_table = HashMap::new();
        for entry in table {
            let own = entry.get("own")
                .and_then(|v| v.as_array())
                .ok_or("Each entry must have an 'own' array")?;
            let opp = entry.get("opp")
                .and_then(|v| v.as_array())
                .ok_or("Each entry must have an 'opp' array")?;
            let move_str = entry.get("move")
                .and_then(|v| v.as_str())
                .ok_or("Each entry must have a 'move' string")?;
            let own_moves = own.iter().map(|m| match m.as_str() {
                Some("C") => Ok(Move::Cooperate),
                Some("D") => Ok(Move::Defect),
                _ => Err("Invalid move in 'own' array, must be 'C' or 'D'"),
            }).collect::<Result<Vec<_>,_>>()?;
            let opp_moves = opp.iter().map(|m| match m.as_str() {
                Some("C") => Ok(Move::Cooperate),
                Some("D") => Ok(Move::Defect),
                _ => Err("Invalid move in 'opp' array, must be 'C' or 'D'"),
            }).collect::<Result<Vec<_>,_>>()?;
            let mv = match move_str {
                "C" => Move::Cooperate,
                "D" => Move::Defect,
                _ => return Err("Invalid 'move' value, must be 'C' or 'D'".to_string()),
            };
            lookup_table.insert(HistoryKey(own_moves, opp_moves), mv);
        }
        self.lookup_table = lookup_table;
        self.history_length = history_length;
        Ok(())
    }
}

impl fmt::Display for LookerUpStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Looker Up")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Looker Up",
        aliases: &["lookerup"],
        description: "Makes decisions based on the last N moves of both players using a predefined lookup table. If no lookup is provided, it defaults to Cooperate.",
        constructor: || Box::new(LookerUpStrategy::new()),
        supports_parameters: true,
    }
}
