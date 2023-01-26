use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

// derive traits and create config struct with an admin address
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

// derive traits and create poll struct with a creator address, title, question, and options
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub title: String,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

// derive traits and create ballot struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

// define the storage keys for config, polls, and ballots in the contract
pub const CONFIG: Item<Config> = Item::new("config");

pub const POLLS: Map<String, Poll> = Map::new("polls");

pub const BALLOTS: Map<String, Addr> = Map::new("ballots");
