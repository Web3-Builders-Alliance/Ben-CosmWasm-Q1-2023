use cosmwasm_std::{
    BankMsg, Coin, DepsMut, Env, MessageInfo, Reply, Response, StdError, SubMsg, Uint128,
};
// importing the necessary structures from cosmwasm_std; in this case we can see injective will be using submessages, an probably initiating messages once the original messages have returned successfully
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cw2::set_contract_version;
use injective_cosmwasm::{ // importing structures from injective's own novel library
                          create_deposit_msg, create_spot_market_order_msg, create_withdraw_msg,
                          get_default_subaccount_id_for_checked_address, InjectiveMsgWrapper, InjectiveQuerier,
                          InjectiveQueryWrapper, SpotOrder,
};
use injective_math::FPDecimal;
// importing from their own novel math library
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::proto_parser::{parse_protobuf_bytes, parse_protobuf_string, ResultToStdErrExt};
use crate::state::{ContractConfigState, STATE, SWAP_OPERATION_STATE, SwapCacheState};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:atomic-order-example";
// this is a macro that will be replaced by the name of the crate in the Cargo.toml file
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// this is a macro that will be replaced by the contract's version number in the Cargo.toml file
pub const ATOMIC_ORDER_REPLY_ID: u64 = 1u64;
// reply id for the submessage
pub const DEPOSIT_REPLY_ID: u64 = 2u64; // reply id for the submessage

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<InjectiveQueryWrapper>, // mutable state wraps a custom struct called InjectiveQueryWrapper ??
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> { // successful response returns another custom struct
    let querier = InjectiveQuerier::new(&deps.querier);
    if let Some(market) = querier.query_spot_market(&msg.market_id)?.market { // the " if let " structure coupled with the Some option type says that if there is something there and it exists, (and not nothing) then destructure it and create this new object and perform the logic in the proceeding code block
        let state = ContractConfigState { // here we create a new object called state and apply it to the ContractConfigState custom struct that we defined in the state file
            market_id: msg.market_id, // market_id taken from the message passed by the client
            base_denom: market.base_denom, // base denomination taken from the market that was passed
            quote_denom: market.quote_denom, // quote denomination passed in market
            owner: info.sender.clone(), // owner is the sender of the instantiate message from the client
            contract_subaccount_id: get_default_subaccount_id_for_checked_address(
                &env.contract.address, // the contract's address found in the environment is hte contract sub-account's id
            ),
        };
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; // set contract version
        STATE.save(deps.storage, &state)?; // save the STATE singleton Item

        Ok(Response::new() // add events to metadata
            .add_attribute("method", "instantiate")
            .add_attribute("owner", info.sender))
    } else { // if there was no market passed to the Some option above then invoke the custom error response
        Err(ContractError::CustomError {
            val: format!("Market with id: {} not found", msg.market_id.as_str()), // format the market id that does not exist into the string and pass that back to the client/front end
        })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<InjectiveQueryWrapper>, // mutable deps is still this query wrapper? There doesn't seem to be any query messages in this contract, so maybe this wrapper handles that for us?
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> {
    match msg {

        // the only execute message possible for this contract is to try a token swap of a certain quantity at a certain price
        ExecuteMsg::SwapSpot { quantity, price } => try_swap(deps, env, info, quantity, price), // the quantity and price that the user wants to swap for and at; is then passed to the try_swap function which takes state, environment, message info, quantity to swap, and price to execute the swap
    }
}

// try_swap execute function requires mutable deps, the blockchain environment, message info, a quantity of tokens to swap, and the price to swap them at
pub fn try_swap(
    deps: DepsMut<InjectiveQueryWrapper>,
    env: Env,
    info: MessageInfo,
    quantity: FPDecimal,
    price: FPDecimal,
) -> Result<Response<InjectiveMsgWrapper>, ContractError> { // results in a message wrapper or contract error
    let config = STATE.load(deps.storage)?; // access STATE item and call it config
    let contract = env.contract.address; // access the contract address we are dealing with from the blockchain environment
    let subaccount_id = config.contract_subaccount_id; // access the sub_account ID that is now stored in STATE
    let min_deposit = price * quantity;  // minimum deposit of tokens ??
    if info.funds.is_empty() { // if the user doesn't deposit an funds then return a custom error message
        return Err(ContractError::CustomError {
            val: "No funds deposited!".to_string(), // format the custom error message as a string and return it to the client
        });
    }
    let message_deposit = FPDecimal::from(info.funds[0].amount.u128()); // if the contract doesn't error, and the funds are not empty, create a message_deposit object equal to the funds sent in the message and their amount
    if message_deposit < min_deposit { // if the deposited funds are less than the minimum funds then return a custom error message to the front end
        return Err(ContractError::CustomError {
            val: format!("Deposit: {message_deposit} below min_deposit: {min_deposit}"), // format the error message and return it to the client -- include their deposited amount and the minimum deposit required to perform the swap
        });
    }
    let order = SpotOrder::new( // create a new spot order structure called order
        price, // takes a price for the spot order
        quantity, // a quantity of tokens
        true, // boolean parameters required for the SpotOrder struct
        false, // boolean parameters required for the SpotOrder struct
        true, // boolean parameters required for the SpotOrder struct
        &config.market_id, // market to place the spot order in, taken from a referenced config
        subaccount_id.to_owned(), // sub-account ID
        Some(contract.to_owned()), // contract ID if it exists
    );

    let coins = &info.funds[0]; // coins from the message info.funds vector -- the first element (there should only be 1 element in the vector)
    let deposit_message = SubMsg::new(create_deposit_msg( // create a deposit message from a new SubMsg with the following parameters passed to that message structure
        contract.clone(), // clone the contract ID
        subaccount_id, // the sub-account ID
        coins.clone(), // clone the coins being sent
    ));
    let order_message = SubMsg::reply_on_success( // create an order message from a SubMsg  structure that takes another message structure as input
        create_spot_market_order_msg(contract, order), // which takes a contract ID and the SpotOrder order defined above
        ATOMIC_ORDER_REPLY_ID, // atomic order reply id ??
    );
    let response = Response::new() // add sub_message responses if successful to this point
        .add_submessage(deposit_message) // the deposit message defined above
        .add_submessage(order_message); // the order message defined above

    let cache = SwapCacheState { // define a cache object from the SwapCacheState structure which takes the following parameters
        sender_address: info.sender.to_string(), // the sender's address from the message info
        deposited_amount: coins.clone(), // the amount of deposited coins
    };
    SWAP_OPERATION_STATE.save(deps.storage, &cache)?; // save this to the swap_operation_state_singleton defined in the state.rs file

    Ok(response) // Respond with response if successful
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply( // reply entry point on contract success
    deps: DepsMut<InjectiveQueryWrapper>, // another wrapper structure for mutable deps
    env: Env,
    msg: Reply, // reply message
) -> Result<Response<InjectiveMsgWrapper>, StdError> {
    match msg.id { // match the ID from the message being passed
        ATOMIC_ORDER_REPLY_ID => handle_atomic_order_reply(deps, env, msg), // send that message to the handle_atomic_order_reply function which takes deps, env, and msg as inputs
        id => Err(StdError::generic_err(format!("Unknown reply id: {id}"))), // if the ID in the reply message is unknown then format that in a string and return it to the client
    }
}

// most of this function below is foreign to me.
fn handle_atomic_order_reply( // handle_atomic_order_reply function takes:
    deps: DepsMut<InjectiveQueryWrapper>, // another wrapper structure for deps
    env: Env, // env
    msg: Reply, // Reply message
) -> Result<Response<InjectiveMsgWrapper>, StdError> {
    let dec_scale_factor: FPDecimal = FPDecimal::from(1000000000000000000_i128); // not sure what this is
    let mut data = msg.result.unwrap().data.unwrap().to_vec(); // unwrap the message data into a vector
    let _ = parse_protobuf_string(&mut data, 1); // order hash - we need to read to advance reader

    let trade_result = parse_protobuf_bytes(&mut data, 2).with_stderr()?;
    let mut trade_data = trade_result.unwrap().to_vec();
    let field1 = parse_protobuf_string(&mut trade_data, 1).with_stderr()?;
    let field2 = parse_protobuf_string(&mut trade_data, 2).with_stderr()?;
    let field3 = parse_protobuf_string(&mut trade_data, 3).with_stderr()?;
    let quantity = FPDecimal::from_str(&field1)? / dec_scale_factor;
    let price = FPDecimal::from_str(&field2)? / dec_scale_factor;
    let fee = FPDecimal::from_str(&field3)? / dec_scale_factor;

    let config = STATE.load(deps.storage)?;
    let contract_address = env.contract.address;
    let subaccount_id = config.contract_subaccount_id;

    let cache = SWAP_OPERATION_STATE.load(deps.storage)?;

    let purchased_coins = Coin::new(u128::from(quantity), config.base_denom.clone());
    let paid = quantity * price + fee;
    let leftover = cache.deposited_amount.amount - Uint128::from(u128::from(paid));
    let leftover_coins = Coin::new(u128::from(leftover), config.quote_denom);
    // we need to withdraw coins from subaccount to main account so we can transfer them back to a user
    let withdraw_purchased_message = create_withdraw_msg(
        contract_address.clone(),
        subaccount_id.clone(),
        purchased_coins.clone(),
    );
    let withdraw_leftover_message =
        create_withdraw_msg(contract_address, subaccount_id, leftover_coins.clone());

    let send_message = BankMsg::Send {
        to_address: cache.sender_address,
        amount: vec![purchased_coins, leftover_coins],
    };

    let response = Response::new()
        .add_message(withdraw_purchased_message)
        .add_message(withdraw_leftover_message)
        .add_message(send_message);
    Ok(response)
}
