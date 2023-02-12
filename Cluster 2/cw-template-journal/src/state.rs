use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    // global count of increment function calls
    pub count: i32,
    // owner of the contract who instantiated it
    pub owner: String, // validate as an Addr using the deps.api.addr_validate() function
}

// define the state item in storage with the key state
pub const STATE: Item<State> = Item::new("state");

// define the user_count map in storage with the key user_count
pub const USER_COUNT: Map<&Addr, i32> = Map::new("user_count");
