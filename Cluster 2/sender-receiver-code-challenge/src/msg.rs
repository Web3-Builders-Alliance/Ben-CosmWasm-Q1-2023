use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // receive the coins from the sender
    Forward { forward_address: String }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
