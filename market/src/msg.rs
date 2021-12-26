use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Food

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FoodHandleMsg {
    Mint {
        recipient: HumanAddr,
        amount: Uint128,
        padding: Option<String>,
    },
    SetMinters {
        minters: Vec<HumanAddr>,
        padding: Option<String>,
    },
}

// Market

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub food_token_addr: HumanAddr,
    pub food_token_code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Buy {},
}
