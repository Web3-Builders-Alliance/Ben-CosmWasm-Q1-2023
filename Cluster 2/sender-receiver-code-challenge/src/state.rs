/*
Cluster 2 Challenge 😈

You need two actors, Sender and Receiver.

The Sender:
- Receives native tokens from anyone and forwards them to the Receiver.
    An address, which takes any generic token and forwards it to the receiver.
- Stores how much tokens have been received/forwarded, which can be returned in a Query.
    saves to state the info.funds it has received and forwarded

The Receiver:
- Stores the received tokens until the owner of the contract claims them.
    Accepts tokens of any generic type, found in the info.funds
- The owner can claim part of the tokens held by the Receiver, or all at once.
    owner is the admin of the contract? and can claim from the receiver

Optional:
- The Sender gets notified when the Receiver has transferred the funds.
- The Sender gets notified when the Receiver funds have been claimed by its owner.
Assume happy paths, though minor validations are expected. Pass any relevant information you need on the messages.

Additional Resources:
- cw-template: https://github.com/CosmWasm/cw-template
- cw-storage-plus: https://github.com/CosmWasm/cw-storage-plus
- cw-plus: https://github.com/CosmWasm/cw-plus

Let the game begin.

---

2nd set of challenges

- Validate that the amount of tokens being sent in the transaction match the ones the execute message intents to deposit (given that you are passing an amount in the execute message)
- Validate that the denom of the funds sent is uluna, considering we are gonna deploy this on terra.
- Validate only 1 type of coin is being sent in the tx.
- Create custom errors for those scenarios where things go wrong.

- Create a query on the sender to check how many tokens have been received/forwarded
- Create a query on the receiver to check if the tokens have been claimed by the owner

 */

// import Coin so we can understand and send Coins in the contract
use cosmwasm_std::Coin;
use cw_storage_plus::Item;

pub const TOKENS_SENT: Item<Coin> = Item::new("tokens_sent");
