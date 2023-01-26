// original state, or configuration of the contract, including all of its possible structures, their fields, and their data types

// import dependencies
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// import Addr from cosmwasm_std library
use cosmwasm_std::Addr;

// import Item and Map from cw_storage_plus library so we can store data in the contract's storage/state
use cw_storage_plus::{Item, Map};

// define the Config struct with a single field called admin which is an Addr
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

// define the Poll struct with three fields: creator, question, and options
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,
}

// define the Ballot struct with a single field called option which is a String
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ballot {
    pub option: String,
}

// create a constant called CONFIG which is an Item of struct data type Config, stored as the "config" key in the contract's storage/state
pub const CONFIG: Item<Config> = Item::new("config");

// create a constant called POLLS which is a Map of struct data type Poll, stored as the "polls" key in the contract's storage/state
pub const POLLS: Map<&str, Poll> = Map::new("polls");

// create a constant called BALLOTS which is a Map of struct data type Ballot, stored as the "ballots" key in the contract's storage/state
pub const BALLOTS: Map<(Addr, &str), Ballot> = Map::new("ballots");
