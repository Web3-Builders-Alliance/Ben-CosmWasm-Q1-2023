use cosmwasm_schema::{cw_serde, QueryResponses};
// import Coin struct
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ForwardTokens { forward_to_addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetTokensRecievedResponse)]
    TokensReceived { tokens: Coin },

    #[returns(GetTokensClaimedResponse)]
    TokensClaimed { tokens: Coin },
}

#[cw_serde]
pub struct GetTokensClaimedResponse {
    tokens: Coin,
}

#[cw_serde]
pub struct GetTokensReceivedResponse {
    tokens: Coin,
}


/*
- Create a query on the sender to check how many tokens have been received/forwarded - DONE
- Create a query on the receiver to check if the tokens have been claimed by the owner - DONE
 */
