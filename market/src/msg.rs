use cosmwasm_std::{Binary, CanonicalAddr, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Food
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FoodInitMsg {
    pub name: String,
    pub admin: Option<HumanAddr>,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Option<Vec<FoodInitialBalance>>,
    pub prng_seed: Binary,
    pub config: Option<FoodInitConfig>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct FoodInitialBalance {
    pub address: HumanAddr,
    pub amount: Uint128,
}

impl FoodInitMsg {
    pub fn config(&self) -> FoodInitConfig {
        self.config.clone().unwrap_or_default()
    }
}

/// This type represents optional configuration values which can be overridden.
/// All values are optional and have defaults which are more private by default,
/// but can be overridden if necessary
#[derive(Serialize, Deserialize, JsonSchema, Clone, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct FoodInitConfig {
    /// Indicates whether the total supply is public or should be kept secret.
    /// default: False
    public_total_supply: Option<bool>,
}

impl FoodInitConfig {
    pub fn public_total_supply(&self) -> bool {
        self.public_total_supply.unwrap_or(false)
    }
}

// Market

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Register { snip_20: CanonicalAddr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
