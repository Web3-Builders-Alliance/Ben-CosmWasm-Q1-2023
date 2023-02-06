# cw721 base contract

* What are the concepts (borrowing, ownership, vectors etc)

The cw721 base contract uses more abstractions than the cw20 token standard, such as the cw721Contract struct, that
implements generic types.

The contract itself allows for the creation of a novel NFT token, with optional limit on the number of tokens that can
be minted.

The contract has an owner, authorized minter, and the ability to approve all message types for a given spender address,
or 3rd part operator. These approvals can be limited or expire.

The contract also has the ability to mint, burn, transfer, send, and approve tokens.

Sending a token to a contract requires the receiving contract to implement the cw721Receiver interface.

* What is the organization?

The src folder contains contract_tests, error, execute messages, helers, lib, msg, query, and state files.

* What is the contract doing? What is the mechanism?

The contract itself allows for the creation of a novel NFT token, with optional limit on the number of tokens that can
be minted.

The contract has an owner, authorized minter, and the ability to approve all message types for a given spender address,
or 3rd part operator. These approvals can be limited or expire.

The contract also has the ability to mint, burn, transfer, send, and approve tokens.

* How could it be better? More efficient? Safer?

Not quite sure yet, but the use of generic type and the abstraction of a basic NFT contract seems very powerful.

* The code could be safer and better if…

Add more abstractions and generic types where possible.
