use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-template-code-journal";
// these constants are global, but not public
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); // they can be accessed by any function in the contract but not imported into another file using the contract crate

#[cfg_attr(not(feature = "library"), entry_point)]
//////////////////////////////////////////////////////////
pub fn instantiate( // this line begins to define the public function called instantiate
                    deps: DepsMut, // this line defines the first parameter of the function -- mutable state must be a parameter of the message as we are changing the state of the contract with instantiation
                    _env: Env, // this line defines the second parameter of the function -- holds information about the blockchain environment at time of execution such as block height
                    info: MessageInfo, // this line defines the third parameter of the function -- which holds info about the message being passed such as who is sending it
                    msg: InstantiateMsg, // this line defines the fourth parameter of the function -- message being passed, which in this case must be instantiate
) -> Result<Response, ContractError> { // this line defines the return value of the function, which is a Result type, of an Ok Response or an Error ContractError
//////////////////////////////////////////////////////////
// the lines below, which are still part of the function are the actual working code of the function itself. This is important to keep in mind because of the concept of function scope in the Rust language
    let state = State { // define a state object to hold the count and owner which come from the msg being passed, and the info contained in the message
        count: msg.count, // if a count is passed in the message, then set the count to that value in state
        owner: info.sender.clone(), // set the owner to the sender of the message
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; // set the contract version
    STATE.save(deps.storage, &state)?; // save the state to the storage

    // it is important to keep in mind that in Rust/CosmWasm Smart Contracts, that we are essentially calling functions within associated functions, within associated functions.
    // the functions called all have a name, a set of parameters, an expected return with result types, and then the actual working code of that function, which is usually object oriented -- such as using the STATE object to update state with whatever we want to do

/////////////////////////////////////////////////
    // this section of the contract adds attributes, also known as metadata, upon the successful execution of the instantiate function
    Ok(Response::new()
        .add_attribute("method", "instantiate") // tells the viewer that the instantiate method was called
        .add_attribute("owner", info.sender) // tells the viewer that the owner of the contract is the sender of the message, found in the info of the message
        .add_attribute("count", msg.count.to_string())) // tells the viewer that a count was passed and that it's equal to the count passed in the message by the sender
}
///////////////////////////////////////////////


///////////////////////////////////////////////
// original definition section of the execute function -- which defines the function name, the input parameters, and the expected return and result types
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut, // execute entrypoint changes state, so we must use mutable deps, or DepsMut
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
///////////////////////////////////////////////
    // second section of the execute function, which defines the actual working code of the execute function
    match msg { // the contract is going to match the message sent, and then execute a subsequent function based on the message receieved
        ExecuteMsg::Increment {} => execute::increment(deps), // if the Increment message is received, then call the increment function, which takes deps as a parameter
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count), // if the Reset message is sent, which contains a count within that message (passed by the user), then call the reset function, which takes deps, msginfo, and count as parameters
    }
}


///////////////////////////////////////////////
// this section defines a public module called execute which then defines each of the associated possible execute message functions to be called given the appropriate execute message receieved
pub mod execute {
    use super::*;

// this line imports all of the functions and types from the parent module

    // the incrememnt function takes mutable state as a parameter and returns a response
    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {

        // this section is the working code of the increment function starting with the update method which is called on STATE, and this returns an internal result or a contract error
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> { // state is updated and a result is returned by this method
            state.count += 1; // the count is incremented by 1 in state
            Ok(state) // the state is returned
        })?; // unwrap and check for any errors

        // add an attribute if the increment function is successful
        Ok(Response::new().add_attribute("action", "increment"))
    }

    // this first section of the reset function takes mutable state, msg info, and a count as parameters and returns a response or an error
    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> { // a count of type i32 is passed in the message by the user on the front end

        // this section defines the what the update method being called on STATE does, which returns an internal result of a contract error
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> { // deps and mutable state are inputs to the update method

            // if the sender of the message is not equal to the owner currently defined in state
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {}); // return the unauthorized error
            }
            state.count = count; // if the sender is the owner of the contract then set the count in state equal to the count that was passed in the message
            Ok(state) //  return state if successful
        })?; // unwrap the returned Ok state and check for errors

        // add this attribute/metadata to the front end's response if the reset function is successful
        Ok(Response::new().add_attribute("action", "reset"))
    }
}
///////////////////////////////////////////////

///////////////////////////////////////////////

// this section defines the query messages that can be sent to the contract
#[cfg_attr(not(feature = "library"), entry_point)] // these query functions and and query module are public, which means they can be accessed outside of this contract file, as well as outside of the query module itself
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> { // this line defines the query function name, its parameters, and its return type
    match msg {  // this line matches the msg sent by the user to the contract
        QueryMsg::GetCount {} => to_binary(&query::count(deps)?), //  If the GetCount message was called, then first called the to_binary method on it, and call the query function which takes state as a parameter
    }
}

///////////////////////////////////////////////

// this section defines the query module and the functions associated with the query entrypoint
pub mod query {
    use super::*;

// this line imports all of the functions and types from the parent module

    // this section defines the count function, its parameters, and its expected result
    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {

        // these lines define a new state object and load the state from storage
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count }) // return the count from state if the count function is successful
    }
}

//  unit tests for the contract which I still don't really understand tbh.
#[cfg(test)]
// this tests module is not a public module, and is therefore not accessible outside of this file
mod tests {
    use cosmwasm_std::{coins, from_binary};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    // these functions are not public, and are therefore not accessible outside of the tests module, nor the contract file.
    #[test]
    fn proper_initialization() { // name the proper_initialization function
        let mut deps = mock_dependencies(); // mock dependencies method

        let msg = InstantiateMsg { count: 17 }; // define the msg sent as the instantiate message with an original count of 17
        let info = mock_info("creator", &coins(1000, "earth")); // define mock info using the mock_info method

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap(); // define a result of the instantiate function that takes mock state, mock env, mock info, and a mock msg
        assert_eq!(0, res.messages.len()); // assert that the length of the messages array is 0

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
