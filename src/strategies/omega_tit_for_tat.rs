use crate::{Move, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;

#[derive(Default)]
pub struct OmegaTitForTatStrategy {
    deadlock_threshold: usize,
    randomness_threshold: usize,
    deadlock_count: usize,
    randomness_count: usize,
    unconditional_defection: bool,
    params_set: bool,
}

impl OmegaTitForTatStrategy {
    pub fn new() -> Self {
        Self {
            deadlock_threshold: 3,
            randomness_threshold: 3,
            deadlock_count: 0,
            randomness_count: 0,
            unconditional_defection: false,
            params_set: false,
        }
    }
}

impl Strategy for OmegaTitForTatStrategy {
    fn next_move(&mut self, own_history: &[Move], opponent_history: &[Move]) -> Move {
        if self.unconditional_defection {
            return Move::Defect;
        }
        if own_history.is_empty() {
            return Move::Cooperate;
        }
        // Deadlock: both players alternate C/D for several rounds
        if own_history.len() >= 2 && opponent_history.len() >= 2 {
            let n = own_history.len();
            let my_last = own_history[n-1];
            let my_prev = own_history[n-2];
            let opp_last = opponent_history[n-1];
            let opp_prev = opponent_history[n-2];
            // Deadlock pattern: CD/DC or DC/CD
            if (my_last != my_prev) && (opp_last != opp_prev) && (my_last == opp_prev) && (my_prev == opp_last) {
                self.deadlock_count += 1;
            } else {
                self.deadlock_count = 0;
            }
            // Randomness: both players change move frequently
            if (my_last != my_prev) || (opp_last != opp_prev) {
                self.randomness_count += 1;
            } else {
                self.randomness_count = 0;
            }
        }
        // Check thresholds
        if self.deadlock_count >= self.deadlock_threshold {
            self.deadlock_count = 0;
            return Move::Cooperate;
        }
        if self.randomness_count >= self.randomness_threshold {
            self.unconditional_defection = true;
            return Move::Defect;
        }
        // Tit for Tat
        let idx = own_history.len();
        if idx == 0 { return Move::Cooperate } else { opponent_history[idx-1] }
    }
 
    fn set_parameters(&mut self, params: serde_json::Value) -> Result<(), String> {
        if let Some(dl) = params.get("deadlock_threshold") {
            if let Some(v) = dl.as_u64() {
                self.deadlock_threshold = v as usize;
            }
        }
        if let Some(rn) = params.get("randomness_threshold") {
            if let Some(v) = rn.as_u64() {
                self.randomness_threshold = v as usize;
            }
        }
        self.params_set = true;
        Ok(())
    }
}

impl fmt::Display for OmegaTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Omega Tit For Tat Strategy")
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Omega Tit For Tat",
        aliases: &["omega_tft", "otft"],
        description: "Plays 'Tit for Tat' unless deadlock or randomness thresholds are exceeded. Cooperates to break deadlock, defects unconditionally if randomness threshold is exceeded. Both 'deadlock' and 'randomness' can be passed as params.",
        constructor: || Box::new(OmegaTitForTatStrategy::new()),
        supports_parameters: true,
    }
}
