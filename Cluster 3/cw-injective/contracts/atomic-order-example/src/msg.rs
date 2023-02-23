use injective_cosmwasm::MarketId;
use injective_math::FPDecimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub market_id: MarketId, // the instantiate message only requires a market_id
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg { // only 1 possible execute message -- which is to create a spot swap order of a given token quantity at a given price
    SwapSpot { // spot swap structure which takes two required parameters
        quantity: FPDecimal, // quantity to be swapped
        price: FPDecimal, // price to be swapped at
    },
}
