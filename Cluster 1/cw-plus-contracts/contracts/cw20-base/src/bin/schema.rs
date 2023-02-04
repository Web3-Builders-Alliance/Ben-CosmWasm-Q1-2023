// import the write_api macro to generate the schema
use cosmwasm_schema::write_api;
// use the msg types already defined in the cw20-base contract -- execute, instantiate, and query
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// the main function which writes the schema to the file system for instantiate, execute and query messages using the write_api macro
fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
