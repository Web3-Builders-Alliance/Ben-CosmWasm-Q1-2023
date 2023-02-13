use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32, // The instantiate message instantiates the contract with a count variable, which again should probably be changed to a Uint128
}

#[cw_serde]
pub enum ExecuteMsg {
    //  The Increment function increments the count variable
    Increment {},
    // The Reset function resets the count variable to a new value passed in the message, which again should probably be changed to a Uint128 so a negative variable is not allowed
    Reset { count: i32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {}, // currently the only query message is GetCount, which returns the current count
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32, // this is the response type that the GetCount query message returns, which again should probably be changed to a Uint128
}
