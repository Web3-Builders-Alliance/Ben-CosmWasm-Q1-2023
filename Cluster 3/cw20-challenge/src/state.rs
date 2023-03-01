/*
Cluster 3 challenge:

- Create a contract that instantiates a cw20 token, and that mints an amount of cw20 token equal to the amount of native tokens that are sent to the contract. For example, if you send 100 uluna to the contract, the contract should mint you 100 tokens of the cw20 token you create.
    Instantiate an existing cw20 token contract with the instantiate2 entry point
    Use the mint module from the cosmos sdk to mint the cw20 tokens in the exact amount of received coins


- The contract should be migratable, and you should deploy this on testnet and run a migration on it.
    use the migrate entry point from the cw-base contract

- You should write tests (unit or integration) validating the actions on the first point.
    Validate that the number of tokens received are the number of tokens created
    validate the token being minted is the token you expect to be minted
    validate the tokens received are locked by the contract as long as the cw20 tokens minted still exist

- Optional: it should be possible to withdraw your original tokens by sending the LP tokens back to the contract.


Use Mint, Burn, and Transfer from the cw20 base

Interacting with another contract from a contract requires wasm contracts
    So when you mint or burn the token then you need to use a wasm message.

Store the contract address of the token in your contract so you can interact with it.

We are creating a contract that imports the libraries from another contract and interacts with it.

It is contracts communicating with each other

Use Instantiate2 to instantiate another contract from your contract


--------------------------

1. Someone sends funds to our contract, in this case uluna.
     - Validate the funds sent to the contract by the sender are uluna. The contract does not accept anything other than uluna.

2. The contract sends a message to the cw20-base contract to mint the exact of number of cw20 tokens as were sent to the first contract in uluna by the sender.

3. The cw20-token contract sends those newly minted B&B tokens to the original sender of the uluna.


 */

use cw_standard::Addr;
use cw_storage::Item;

// constant state item to hold the hard-coded contract address
pub const CW20_ADDRESS: Item<CW20Address> = Item::new("terra1n6ktwxmyr48rackdv4xapvggc4kg0rrvyc4ufhd543mx96knnh4sf78tsy");

// structure to hold the address of our contract
pub struct CW20Address {
    address: Addr,
}
