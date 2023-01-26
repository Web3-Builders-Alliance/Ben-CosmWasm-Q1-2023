#[cfg(not(feature = "library"))]
// import entry points from cosmwasm_std
use cosmwasm_std::entry_point;

// import required packages from cosmwasm_std library
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};

// import set_contract_version from cw2 library
use cw2::set_contract_version;

// import required dependencies from state file
use crate::state::{Ballot, Config, Poll, BALLOTS, CONFIG, POLLS};

// import required dependencies from error file
use crate::error::ContractError;

// import required dependencies from msg file
use crate::msg::{
    AllPollsResponse, ExecuteMsg, InstantiateMsg, PollResponse, QueryMsg, VoteResponse,
};

// set contract name and version as constants
const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// define instantiate function as contract's first entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate( 
    // define required inputs to the instantiate function
    deps: DepsMut, // mutable contract state
    _env: Env, // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    info: MessageInfo, // external enviroment info such as sender address, sent funds, etc..
    msg: InstantiateMsg, // message being sent to contract
) -> Result<Response, ContractError> { // define the result response of the instantiate function and any errors that may occur
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; // set contract version and check for errors
    let admin = msg.admin.unwrap_or_else(|| info.sender.to_string()); // set admin to Addr of the message sender if no admin is provided when the message is sent
    let validated_admin = deps.api.addr_validate(&admin)?; // validate the admin address and check for errors
    let config = Config { // define the contract's config as a Config struct
        admin: validated_admin.clone(), // set the admin to the validated admin address as a clone, so the reference is not lost of the original admin variable
    };
    CONFIG.save(deps.storage, &config)?; // save the config to the contract's storage and check for errors
    Ok(Response::new() // define the response if the instantiate function is successful and add new attributes to the metadata
        .add_attribute("action", "instantiate") // lets the client know the action that was performed was an instantiation
        .add_attribute("admin", validated_admin.to_string())) // lets the client know the admin address
}

// define execute function as contract's second entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute( // define required inputs to the execute function
    deps: DepsMut, // mutable contract state
    env: Env, // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    info: MessageInfo, // external enviroment info such as sender address, sent funds, etc..
    msg: ExecuteMsg, // message being sent to contract
) -> Result<Response, ContractError> { // define the result response of the execute function and any errors that may occur
    match msg { // match the message being sent to the execute function
        ExecuteMsg::CreatePoll { // if the message is a CreatePoll message, then look for the following required inputs to the message so it can execute
            poll_id, // poll id
            question, // poll question
            options, // poll answer/choice options
        } => execute_create_poll(deps, env, info, poll_id, question, options), // execute the create_poll_function using the required inputs
        ExecuteMsg::Vote { poll_id, vote } => execute_vote(deps, env, info, poll_id, vote), // if the message is a Vote message, then look for the following required inputs to the message so it can execute
    }
}

// define the execute_create_poll function which is called by the execute function 
fn execute_create_poll( // define required inputs to the execute_create_poll function
    deps: DepsMut, // mutable contract state
    _env: Env, // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    info: MessageInfo, // external enviroment info such as sender address, sent funds, etc..
    poll_id: String, // poll id as a String
    question: String, // poll question as a String
    options: Vec<String>, // poll answer/choice options as a Vec of Strings
) -> Result<Response, ContractError> { // define the result response of the execute_create_poll function and any errors that may occur
    if options.len() > 10 { // if the number of options is greater than 10, then return an error
        return Err(ContractError::TooManyOptions {}); // return the TooManyOptions error if the conditional line above is true
    }

    // create mutable poll options as a vector of tuples of a pair of Strings and u64s
    let mut opts: Vec<(String, u64)> = vec![];
    for option in options { // for each option in the options vector
        opts.push((option, 0)); // push the option and a 0 to the opts vector
    }

    // create a poll struct
    let poll = Poll { 
        creator: info.sender, // set the poll creator to the message sender's address 
        question, // ingest the poll question from the message
        options: opts, // ingest the poll options from the message
    };

    // save the poll to the contract's state and check for errors
    POLLS.save(deps.storage, &poll_id, &poll)?;

    Ok(Response::new()) // return a response if the execute_create_poll function is successful
}

// define the execute_vote function which is called by the execute function
fn execute_vote( // define required inputs to the execute_vote function
    deps: DepsMut, // mutable contract state
    _env: Env, // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    info: MessageInfo, // external enviroment info such as sender address, sent funds, etc..
    poll_id: String, // poll id as a String
    vote: String, // message sender's vote as a String
) -> Result<Response, ContractError> { // define the result response of the execute_vote function and any errors that may occur
    let poll = POLLS.may_load(deps.storage, &poll_id)?; // load the poll from the contract's state and check for errors

    // define the match statement to update the ballot with the sender's vote 
    match poll {
        Some(mut poll) => {
            // The poll exists
            BALLOTS.update( // update the ballot with the sender's vote
                deps.storage, // update contract state
                (info.sender, &poll_id), // update the ballot with the sender's address and the poll id
                |ballot| -> StdResult<Ballot> { 
                    match ballot {
                        Some(ballot) => {
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter() // iterate through the poll options
                                .position(|option| option.0 == ballot.option) // find the position of the old vote
                                .unwrap(); // unwrap the position of the old vote
                            // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1; // decrement the old vote by 1
                            // Update the ballot
                            Ok(Ballot { 
                                option: vote.clone(), // clone the sender's vote
                            })
                        }
                        None => { // if the ballot does not exist
                            // Simply add the ballot
                            Ok(Ballot {
                                option: vote.clone(), // clone the sender's vote
                            })
                        }
                    }
                },
            )?;

            // Find the position of the new vote option and increment it by 1
            let position = poll.options.iter().position(|option| option.0 == vote);
            if position.is_none() { // if the position of the new vote option is none, then return an error
                return Err(ContractError::Unauthorized {}); 
            }
            let position = position.unwrap(); // unwrap the position of the new vote option
            poll.options[position].1 += 1; // increment the new vote option by 1

            POLLS.save(deps.storage, &poll_id, &poll)?; // save the poll to the contract's state and check for errors
            Ok(Response::new()) // return a response if the execute_vote function is successful
        }
        None => Err(ContractError::Unauthorized {}), // The poll does not exist so we just error
    }
}

// define the query function as the contracts 3rd entrypoint
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query( // define required inputs to the query function
    deps: Deps, // immutable contract state because we are just reading data, not writing data
    env: Env, // enviroment info such as block height, time, chain id, contract address, message sender, etc..
    msg: QueryMsg) // query message being passed to the query function 
    -> StdResult<Binary> { // define return result of the query function as binary data
    match msg { // match the query message to the query function
        QueryMsg::AllPolls {} => query_all_polls(deps, env), // query all polls message which takes storage and enviroment info as inputs
        QueryMsg::Poll { poll_id } => query_poll(deps, env, poll_id), // query poll message which takes storage, enviroment info, and poll id as inputs
        QueryMsg::Vote { address, poll_id } => query_vote(deps, env, address, poll_id), // query a specific address' vote of a specific poll which takes storage, enviroment info, address, and poll id as inputs
    }
}

// define the query_all_polls function which is called by the query function and returns all polls in the contract's state
fn query_all_polls( // define required inputs to the query_all_polls function
    deps: Deps, // immutable contract state because we are just reading data, not writing data
    _env: Env) // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    -> StdResult<Binary> { // define return result of the query_all_polls function as binary data
    let polls = POLLS // load the polls from the contract's state
        .range(deps.storage, None, None, Order::Ascending) // iterate through the polls and order them in ascending order
        .map(|p| Ok(p?.1)) // map the polls to the poll id and poll
        .collect::<StdResult<Vec<_>>>()?; // collect the polls into a vector and check for errors

    to_binary(&AllPollsResponse { polls }) // return the polls as binary data
}

// define the query_poll function which is called by the query function and returns a specific poll in the contract's state
fn query_poll( // define required inputs to the query_poll function
    deps: Deps, // immutable contract state because we are just reading data, not writing data
    _env: Env, // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    poll_id: String) // poll id as a String
    -> StdResult<Binary> { // define return result of the query_poll function as binary data
    let poll = POLLS.may_load(deps.storage, &poll_id)?; // load the poll from the contract's state if the poll id exists and check for errors
    to_binary(&PollResponse { poll }) // return the poll as binary data
}

// define the query_vote function which is called by the query function and returns a specific address' vote of a specific poll in the contract's state
fn query_vote( // define required inputs to the query_vote function
    deps: Deps, // immutable contract state because we are just reading data, not writing data
    _env: Env, // internal enviroment info such as block height, time, chain id, contract address, message sender, etc..
    address: String, // specific user's address as a String to be queried
    poll_id: String) // specifc poll id as a String to be queried against the user's address
    -> StdResult<Binary> { // define return result of the query_vote function as binary data
    let validated_address = deps.api.addr_validate(&address).unwrap(); // validate the input address and unwrap the result
    let vote = BALLOTS.may_load(deps.storage, (validated_address, &poll_id))?; // load the ballot from the contract's state if the address is valid and poll id exists and check for errors

    to_binary(&VoteResponse { vote }) // return the vote as binary data
}

// define tests module
#[cfg(test)]
mod tests {

    // import dependencies from contract.rs file
    use crate::contract::{execute, instantiate, query};

    // import dependencies from msg.rs file
    use crate::msg::{
        AllPollsResponse, ExecuteMsg, InstantiateMsg, PollResponse, QueryMsg, VoteResponse,
    };

    // import dependencies from the cosmwasm_std library
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary};

    // create two mock addresses to use during testing
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        // Mock the dependencies, must be mutable so we can pass it as a mutable, empty vector means our contract has no balance
        let mut deps = mock_dependencies();
        // Mock the contract environment, contains the block info, contract address, etc.
        let env = mock_env();
        // Mock the message info, ADDR1 will be the sender, the empty vec means we sent no funds.
        let info = mock_info(ADDR1, &[]);

        // Create a message where we (the sender) will be an admin
        let msg = InstantiateMsg { admin: None };
        // Call instantiate, unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        // Assert admin is ADDR1
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }

    
    #[test]
    fn test_instantiate_with_admin() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        // Send as ADDR1 to show admin is different
        let info = mock_info(ADDR1, &[]);

        // Create a message where ADDR2 will be an admin
        // Have to use .to_string() method
        let msg = InstantiateMsg {
            admin: Some(ADDR2.to_string()),
        };
        // Unwrap to assert success
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        // Assert admin is ADDR2 instead
        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2),]
        );
    }

    #[test]
    fn test_execute_create_poll_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // New execute msg
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };

        // Unwrap to assert success
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_create_poll_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite number?".to_string(),
            options: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],
        };

        // Unwrap error to assert failure
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn test_execute_vote_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, first time voting
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Change the vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Osmosis".to_string(),
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_vote_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, some_id poll is not created yet.
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        // Unwrap to assert error
        let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Vote on a now existing poll but the option "DVPN" does not exist
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "DVPN".to_string(),
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }

    #[test]
    fn test_query_all_polls() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a second poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_2".to_string(),
            question: "What's your colour?".to_string(),
            options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query
        let msg = QueryMsg::AllPolls {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: AllPollsResponse = from_binary(&bin).unwrap();
        assert_eq!(res.polls.len(), 2);
    }

    #[test]
    fn test_query_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query for the poll that exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_1".to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect a poll
        assert!(res.poll.is_some());

        // Query for the poll that does not exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_not_exist".to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect none
        assert!(res.poll.is_none());
    }

    #[test]
    fn test_query_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &[]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id_1".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query for a vote that exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_1".to_string(),
            address: ADDR1.to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to exist
        assert!(res.vote.is_some());

        // Query for a vote that does not exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_2".to_string(),
            address: ADDR2.to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to not exist
        assert!(res.vote.is_none());
    }
}
