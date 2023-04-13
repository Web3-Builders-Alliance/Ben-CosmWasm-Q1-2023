# My Capstone Project(s)

## Pet That Dog!

https://github.com/bgoober/pet-that-dog (Currently still a private repo, but will be made public soon after main net launch!)

Allows a user to pet a dog!

Project highlights
- User receives a native token for petting the dog.
- A token_middleware contract is deployed to gatekeep minting of native tokens.
- A user can track their own pets count!
- A top 10 leaderboard encourages competition.
- People can now relax while petting a dog!

## Disburser

https://github.com/bgoober/Disburser -- fully public and open source

Project highlights
- Designed to be set as a contract's x/feeshare module withdraw_address.
- Each owner address has some whole-numbered ownership of the generated fees. 
- Each owner address may call disburse at any time, and no one is locked out.
- Simple, and immutable by design.


### Summary
I ended up developing 2 separate contracts, and instantiating a total of 3 in order to complete my capstone.

Before going further, it is important to note I started these contracts as a novice, and still am. They are chiefly for the purposes of learning, and to develop my own knowledge and skills in the cosmos. They are unaudited and should not be trusted further than you must.


My main project, Pet That Dog!, began as the cw-template counter contract. I then modified it to include a user-specific count, and a leaderboard. During the middle, and potentially most formative period of development for PTD!, the native token factory went live on Juno's mainnet, so I decided to add a native token that the user minted each time they pet the dog.

PTD! required a token_middleware contract, developed by Reece, in order to gatekeep minting of the native token, but I did not develop the middleware contract myself. In the process of deploying my own native token, I became a contributor to the repository here https://github.com/CosmosContracts/tokenfactory-contracts.

In all, Pet That Dog! required its own contract, a token_middleware contract, and a native token.

---


My second project came after talking with Benjamin R., who is also in the CW cohort. He described a contract to disburse funds to a set of addresses, and I decided to take it from there.

I trimmed the initial idea down to a simple, working MVP and came up with Disburser. 

Upon instantiation, the contract accepts a list of owners and their % ownership of the underlying fees, or application. This contract's address is then set as the x/feeshare withdraw_address for any fee-generating application. Each owner may then, using their supplied address, call the disburse function on the contract. 100% of the contract's wallet balance will be disbursed to the owners by their % ownership, for each Coin held.

The gas required to disburse funds is paid by the caller. This simple design should be enough to discourage anyone from inputting a duplicate owner during instantiation.

The contract has no migrate entrypoint, and is meant to be immutable. If the application's owner wishes to change the terms of disbursement, then a new contract is required. Therefore, the admin keys of the fee-generating application should be kept.

I decided to develop this contract because Benjamin R. and Juan Marchetto are helping me with the front-end UI and artwork for Pet That Dog!. Using the Disburser contract to share x/feeshare rewards seemed like a good idea. I could not find anything as simple as I had in mind, so I decided to develop the contract. I am pleased to say that even though I have not achieved 100% test coverage, everything seems to be working as intended on testnet.



---


## Testnet Values, and checksums

### Pet That Dog!
ptd_disburser_v0

Contract address:  juno1kz6d5uu8ygku3f5uj8r7tw8zr4zy0a07zpur6lpr4axf4k9a3uhqlahctj

Code ID: 1547 -> then migrated to 1549

sha256: (1549 blob) 74af9573dad2b6ae92f0c93ba1576c823ab77799cdebda73556c53388a042b01

 
### Disburser
disburser_ptd_v0

Contract address: juno1xdtpv263qkrxrz9cqdfm6ar6n7k78ettnfsm50lnd6j7ds2ngdcss68n72
https://testnet.mintscan.io/juno-testnet/account/juno1xdtpv263qkrxrz9cqdfm6ar6n7k78ettnfsm50lnd6j7ds2ngdcss68n72

Code ID: 1548

sha256: a23e2d257722a4b076fe36acc063fb04585bbcfae7d5d642bf9c877f0e51a4fb


### Token Middleware contract 

juno14wj40ylfds89msz9n7yajlcykre5zpxzka5a9e3mqvhu2dvymsvsa0cjp6

