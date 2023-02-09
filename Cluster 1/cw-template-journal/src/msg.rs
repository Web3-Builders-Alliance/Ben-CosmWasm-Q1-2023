use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: u128,
    // add owner to the instantiate message
    pub owner: String, // an Addr is expected, but we will validate it as an address and return it
}

#[cw_serde]
pub enum ExecuteMsg {
    // increment the count is the only possible execution message in the contract
    Increment {},
    // removed the Reset function because we don't want that ability in the contract
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},

    // GetOwner returns the current owner which expects an Addr
    #[returns(GetOwnerResponse)]
    GetOwner {},

    #[returns(GetUserCountResponse)]
    GetUserCount { user: String, user_count: u128 }, // return an Addr that was an address-validated String and the count for that Addr as a u128
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: u128,
}

// define GetOwnerResponse
#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: String, // return an Addr, not just a String
}

// define GetUserCountResponse
#[cw_serde]
pub struct GetUserCountResponse {
    pub user: String,
    // validate it as an address
    pub user_count: u128,
}
