use cosmwasm_std::{Addr, Coin};
// import address and coin structs
use cw_storage_plus::Item;
// import Item from storage plus
use injective_cosmwasm::{MarketId, SubaccountId};
// import structures to identify market IDs, and sub-account IDs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const STATE: Item<ContractConfigState> = Item::new("state"); //  this is the actual singleton item that we'll call state / STATE

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ContractConfigState {
    // create a custom struct for the contract's config state
    pub market_id: MarketId,
    // contains a MarketId structure/data type
    pub owner: Addr,
    // contains an owner address
    pub contract_subaccount_id: SubaccountId,
    // contains a contract sub-account ID of SubaccountId structure/data type
    pub base_denom: String,
    // contains a base denomination of a type string
    pub quote_denom: String, // contains a quote denomination of type string, probably for price, bid, and ask quotes for the specific market
}

pub const SWAP_OPERATION_STATE: Item<SwapCacheState> = Item::new("cache"); // A second singleton Item to hold the Swap Cache's state which holds the parameters of the custom struct defined below

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SwapCacheState {
    // custom structure to represent the state of the Swap Cache
    pub sender_address: String,
    // sender's address of type string
    pub deposited_amount: Coin, // deposited amount, of a Coin data type, which is a !Vec[] of coins
}
