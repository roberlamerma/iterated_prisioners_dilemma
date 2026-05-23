use crate::{Move, Payoffs, Strategy};
use crate::strategies::StrategyInfo;
use std::fmt;
use rand::Rng;


pub struct GenerousTitForTatStrategy {
    payoffs: Payoffs,
}

impl GenerousTitForTatStrategy {
    pub fn new() -> Self {
        Self { payoffs: Payoffs::default() }
    }

    fn cooperation_probability(&self) -> f64 {
        let r = self.payoffs.reward as f64;
        let t = self.payoffs.temptation as f64;
        let s = self.payoffs.sucker as f64;
        let p = self.payoffs.punishment as f64;
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
                if rand::rng().random_bool(self.cooperation_probability()) {
                    Move::Cooperate
                } else {
                    Move::Defect
                }
            }
        }
    }

    fn set_payoffs(&mut self, payoffs: Payoffs) {
        self.payoffs = payoffs;
    }
}

impl fmt::Display for GenerousTitForTatStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generous Tit for Tat (probability: {:.3})", self.cooperation_probability())
    }
}

inventory::submit! {
    StrategyInfo {
        name: "Generous Tit for Tat",
        aliases: &["generoustitfortat", "gen_tft"],
        description: "Cooperates on the first round and after cooperation. After a defection, cooperates with a calculated probability.",
        constructor: || Box::new(GenerousTitForTatStrategy::new()),
        supports_parameters: false,
    }
}