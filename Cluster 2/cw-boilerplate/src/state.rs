use cw_storage_plus::Item;
use schmears::JsonSchema;
use serde::{Deserialize, Serialize};

// If you intend to use Item and Map then use the following line and remove line 3
// use cw_storage_plus::{ Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    // unimplemented state fields
}

pub const STATE: Item<State> = Item::new("state");