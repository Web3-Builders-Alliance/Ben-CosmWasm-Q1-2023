/*
Cluster 2 Challenge 😈

You need two actors, Sender and Receiver.

The Sender:
- Receives native tokens from anyone and forwards them to the Receiver.
- Stores how much tokens have been received/forwarded, which can be returned in a Query.

The Receiver:
- Stores the received tokens until the owner of the contract claims them.
- The owner can claim part of the tokens held by the Receiver, or all at once.

Optional:
- The Sender gets notified when the Receiver has transferred the funds.
- The Sender gets notified when the Receiver funds have been claimed by its owner.
Assume happy paths, though minor validations are expected. Pass any relevant information you need on the messages.

Additional Resources:
- cw-template: https://github.com/CosmWasm/cw-template
- cw-storage-plus: https://github.com/CosmWasm/cw-storage-plus
- cw-plus: https://github.com/CosmWasm/cw-plus

Let the game begin.
 */

// import Coin and Addr from cosmwasm_std
use cosmwasm_std::Coin;
// import ITEM from storage-plus
use cw_storage_plus::Item;

// define a TOTAL constant in storage as a single item with key total
pub const TOKENS_SENT: Item<Coin> = Item::new("tokens_sent");
