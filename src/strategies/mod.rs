// src/strategies/mod.rs
use crate::Strategy;
use std::fmt;

pub mod random;
pub mod tit_for_tat;
pub mod tit_for_two_tats;
pub mod suspicious_tit_for_tat;
pub mod always_cooperate;
pub mod always_defect;
pub mod gradual_tit_for_tat;
pub mod tester;
pub mod configurable_generous_tit_for_tat;
pub mod win_stay_lose_shift;
pub mod win_stay_lose_shift_axelrod;
pub mod grim_trigger;
pub mod hard_tit_for_tat;
pub mod soft_majority;
pub mod firm_majority;
pub mod majority;
pub mod stochastic_tit_for_tat;
pub mod looker_up;
pub mod forgiving;
pub mod reverse_tit_for_tat;
pub mod probability_cooperator;
pub mod adaptative_strategy;
pub mod cycler;
pub mod generous_tit_for_tat;
pub mod imperfect_tit_for_tat;
pub mod two_tit_for_tats;
pub mod omega_tit_for_tat;

// Alias for the strategy constructor function
// Must return a Box<dyn Strategy>
pub type StrategyConstructor = fn() -> Box<dyn Strategy>;

// Define the struct that will be collected by `inventory`
pub struct StrategyInfo {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str, // Optional: Add a description
    pub constructor: StrategyConstructor,
    pub supports_parameters: bool,
}

// Implement Display for StrategyInfo if needed (e.g., for listing)
impl fmt::Display for StrategyInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (aliases: {:?}, accepts parameters: {})",
            self.name,
            self.description,
            self.aliases,
            if self.supports_parameters { "Yes" } else { "No" }
        )
    }
}

// Tell `inventory` to collect all instances of `StrategyInfo`
inventory::collect!(StrategyInfo);

// Helper function to create strategies by name
pub fn create_strategy_by_name(name: &str, params: Option<serde_json::Value>) -> Result<Box<dyn Strategy>, String> {
    let lower_name = name.to_lowercase();
    for strategy_info in inventory::iter::<StrategyInfo> {
        // Check the main name (case-insensitive)
        if strategy_info.name.to_lowercase() == lower_name {
            let mut strategy = (strategy_info.constructor)();
            if let Some(params) = params {
                if !strategy_info.supports_parameters {
                    return Err(format!("Strategy '{}' does not support parameters", name));
                }
                strategy.set_parameters(params)?;
            }
            return Ok(strategy);
        }
        // Check aliases (case-insensitive)
        for alias in strategy_info.aliases {
            if alias.to_lowercase() == lower_name {
                let mut strategy = (strategy_info.constructor)();
                if let Some(params) = params {
                    if !strategy_info.supports_parameters {
                        return Err(format!("Strategy '{}' does not support parameters", name));
                    }
                    strategy.set_parameters(params)?;
                }
                return Ok(strategy);
            }
        }
    }

    // If not found, provide a helpful error message
    let available_strategies: Vec<String> = inventory::iter::<StrategyInfo>()
        .map(|si| si.name.to_string())
        .collect();
    Err(format!(
        "Unknown strategy: '{}'. Available strategies are: {}",
        name,
        available_strategies.join(", ")
    ))
}