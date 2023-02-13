use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: Uint128, // changed from i32 to Uint128 type for count variable
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    // removed Reset message
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: Uint128, // changed from i32 to Uint128 response type
}
