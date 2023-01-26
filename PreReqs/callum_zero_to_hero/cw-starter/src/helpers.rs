// allow unused imports
#![allow(unused_imports)]

// import dependencies
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// import dependencies from cosmwasm_std library
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

// import crate dependencies from msg.rs file
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this. Rename it to your contract name.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

// define a public struct called CwTemplateContract with a single field called pub Addr which is the address of the contract
pub struct CwTemplateContract(pub Addr);

// implement CwTemplateContract
impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    // define the call function with allows for a WasmMsg to execute contract messages
    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
/* commenting out for now so I can compile! No custom responses are being used and I don't really understand this part yet.
    /// Get Custom
    pub fn custom_query<Q, T, CQ>(&self, querier: &Q, val: String) -> StdResult<CustomResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        };
    }
}
*/
