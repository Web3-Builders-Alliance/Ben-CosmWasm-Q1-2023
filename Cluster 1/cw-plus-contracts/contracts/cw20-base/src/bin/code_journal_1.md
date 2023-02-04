# Week 1 Code Journal for cw20-base Contract

@japarjam

* What are the concepts (borrowing, ownership, vectors etc)

The cw20-base contract uses the following concepts:
Singleton struct items for storage.
Map items for storage that store an address and a token amount.

* What is the organization?

There are 7 .rs files in the cw20-base contract: allowances, contract, enumerable, error, lib, msg, and state. I would
add an 8th file for tests specifically.

* What is the contract doing? What is the mechanism?

The cw20-base contract is the base contract for creating and managing tokens that follow the cw20 fungible token
standard.
The sender of the messages can set the marketing information of the token, such as the logo, token information, total
supply, mint cap, and other allowances for specific users or contract addresses. It allows the contract's admin to burn
tokens, mint tokens, set spending allowances, or transfer certain permissions to other user addresses or contract
addresses.

* How could it be better? More efficient? Safer?

I would start by splitting tests and other functions into specific files, as the code is sort of mashed into the
contract file, and it is a little difficult to parse out which functions are associated with certain actions. I'd group
functions, messages, and tests into specific files that are more representative of the scope of their shared
functionality. The allowances.rs file is a good example of this, as it aggregates all of the functions associated with
altering allowances of users or contract addresses.

* The code could be safer and better if…..

It was more easily readable and more organized. I'm not quite sure how it could be made safer yet.
