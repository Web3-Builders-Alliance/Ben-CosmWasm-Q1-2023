/*
Cluster 2-3ish challenge:

- Create a contract that instantiates a cw20 token, and that mints an amount of cw20 token equal to the amount of native tokens that are sent to the contract. For example, if you send 100 uluna to the contract, the contract should mint you 100 tokens of the cw20 token you create.

- The contract should be migratable, and you should deploy this on testnet and run a migration on it.

- You should write tests (unit or integration) validating the actions on the first point.

- Optional: it should be possible to withdraw your original tokens by sending the LP tokens back to the contract.


Use Mint, Burn, and Transfer from the cw20 base

Interacting with another contract from a contract requires wasm contracts
    So when you mint or burn the token then you need to use a wasm message.

Store the contract address of the token in your contract so you can interact with it.

We are creating a contract that imports the libraries from another contract and interacts with it.

It is contracts communicating with each other

Use Instantiate2 to instantiate another contract from your contract


 */


use cw_storage::Item;

