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

// TODO: evaluate implementing passing custom parameters to strategies. Then, this strategy will accept a
// parameter called "history-table", that will contain the history table. Something like:
// history-table = [(C, C), (C, D)] -> C; [(C, C), (C, D)] -> D. Maybe JSON?
impl LookerUpStrategy {
    pub fn new() -> Self {
        let mut table = HashMap::new();

        // Example entries with 2 rounds of history
        table.insert(
            HistoryKey(
                vec![Move::Cooperate, Move::Cooperate],
                vec![Move::Cooperate, Move::Cooperate],
            ),
            Move::Cooperate,
        );
        table.insert(
            HistoryKey(
                vec![Move::Defect, Move::Defect],
                vec![Move::Defect, Move::Defect],
            ),
            Move::Defect,
        );
        table.insert(
            HistoryKey(
                vec![Move::Cooperate, Move::Defect],
                vec![Move::Defect, Move::Cooperate],
            ),
            Move::Defect,
        );
        table.insert(
            HistoryKey(
                vec![Move::Defect, Move::Cooperate],
                vec![Move::Cooperate, Move::Defect],
            ),
            Move::Cooperate,
        );
        table.insert(
            HistoryKey(
                vec![Move::Cooperate, Move::Cooperate],
                vec![Move::Cooperate, Move::Defect],
            ),
            Move::Defect,
        );
        table.insert(
            HistoryKey(
                vec![Move::Cooperate, Move::Cooperate],
                vec![Move::Defect, Move::Cooperate],
            ),
            Move::Defect,
        );

        let history_length = table
            .keys()
            .next()
            .map(|k| k.0.len()) // assume all keys are the same length
            .unwrap_or(0);

        Self {
            lookup_table: table,
            history_length,
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
        description: "Makes decisions based on the last N moves of both players using a predefined lookup table.",
        constructor: || Box::new(LookerUpStrategy::new()),
    }
}
