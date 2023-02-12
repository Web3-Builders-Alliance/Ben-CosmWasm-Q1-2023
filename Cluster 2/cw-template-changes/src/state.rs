use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};
// added Map from cw-storage-plus
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// implement cw_serde traits
#[cw_serde]
pub struct State {
    pub count: Uint128,
    // changed from i32 to Uint128
    pub owner: String, // changed from Addr to String
}

// singleton struct to store the state item
pub const STATE: Item<State> = Item::new("state");

// add a public constant to store a Map of user addresses and their own count
pub const USER_COUNT: Map<&String, Uint128> = Map::new("user_count");
