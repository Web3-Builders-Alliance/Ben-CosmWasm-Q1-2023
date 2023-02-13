use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

// state structure has two variables in it: a signed integer and an address
pub struct State {
    // the data structure is an abstraction defined in the cw-storage-plus crate by our cosmwasm friends
    pub count: i32,
    // this should probably be changed to a Uint128 so that the count can not be negative, as that doesn't make sense, and the Uint128 is a wrapper around a u128 from the cosmwasm standard library.
    pub owner: Addr, // the owner/admin of the contract -- this should probably be changed to a String, as the Addr type seems to generally be untrusted, as it could be a potential security risk. Validating the String as an address using the deps api is probably the better way to go.
}

// the STATE item is now stored in state, as a singleton struct Item, and is a "State" data structure, which is defined in the cw-storage-plus crate
pub const STATE: Item<State> = Item::new("state");  // singleton struct Item now referred to as "state" in storage
