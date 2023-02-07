use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    // global count of increment function calls
    pub count: i32,
    // owner of the contract who instantiated it
    pub owner: Addr,
    // add a user_count variable, which is a map of user addresses to the number of times they have called the increment function
    pub user_count: Map<Addr, i32>,
}

pub const STATE: Item<State> = Item::new("state");
