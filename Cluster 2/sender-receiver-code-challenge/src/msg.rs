use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ForwardTokens { forward_to_addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
