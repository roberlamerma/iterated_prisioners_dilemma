use crate::{Move, Strategy, REWARD, TEMPTATION, SUCKER, PUNISHMENT};
use crate::strategies::StrategyInfo;
use std::fmt;
use rand::Rng;


pub struct GenerousTitForTatStrategy;

impl GenerousTitForTatStrategy {
    fn cooperation_probability() -> f64 {
        let r = REWARD as f64;
        let t = TEMPTATION as f64;
        let s = SUCKER as f64;
        let p = PUNISHMENT as f64;
        let v1 = 1.0 - ((t - r) / (r - s));
        let v2 = (r - p) / (t - p);
        v1.min(v2) // With the standard REWARD, TEMPTATION, SUCKER, PUNISHMENT values (3,5,0,1), this will yield a probability of 0.33
    }
}

impl Strategy for GenerousTitForTatStrategy {
    fn next_move(&mut self, _own_history: &[Move], opponent_history: &[Move]) -> Move {
        match opponent_history.last() {
            None => Move::Cooperate, // First move
            Some(&Move::Cooperate) => Move::Cooperate,
            Some(&Move::Defect) => {
                if rand::rng().random_bool(GenerousTitForTatStrategy::cooperation_probability()) {
                    Move::Cooperate
                } else {
                    Move::Defect
                }
            }
        }
    }
}

impl fmt::Display for GenerousTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generous Tit for Tat (probability: {:.3})", Self::cooperation_probability())
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Generous Tit for Tat",
        aliases: &["generoustitfortat", "gen_tft"],
        description: "Cooperates on the first round and after cooperation. After a defection, cooperates with a calculated probability.",
        constructor: || Box::new(GenerousTitForTatStrategy),
        supports_parameters: false,
    }
}