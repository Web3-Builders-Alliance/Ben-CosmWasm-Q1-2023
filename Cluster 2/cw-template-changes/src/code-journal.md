# cw-Template Contract

* What are the concepts (borrowing, ownership, vectors etc)

The cw-template contract originally allows for any user to increment the count variable by 1 each time the Increment
message is called.

The contract allows for the owner of the contract, who was the address to instantiate the contract, to reset the count.

A single struct, State, encompasses all state objects.

* What is the organization?

Typical orgranization, although tests are still in the contract.rs file.

* What is the contract doing? What is the mechanism?

Increments a global count variable by 1 each time the Increment message is called.

* Proposed changes:

I am attempting to add a user_count variable to state, which is a Map of user's Addr and the number of times that they
have incremented the count variable.

It seems to me that in this scenario we now have a common action being performed -- Increment -- which increments the
count of a variable by 1, each time it is called.

However, two different count variables now exist, a global count variable and a user_count variable, which is a Map of
user's Addr and the number of times they have called Increment.

I wonder if this should be abstracted to a generic type, or not, given that Increment now does two things, or if the
increment function should be split into two functions, one for each count variable.

### Proposed changes to the contract

Here are a few changes that I would like to implement in the contract.

* Change the count variable to be a Uint128 so that the count can never be negative
* Change the owner variable to be a String type, and validate that string as an address rather than using the cw wrapper
  called Addr, which seems to not be safe to use?
* Remove the Reset message and its associated functions so the count can never be reset, even by the contract
  owner/admin
* Add a new state Map item called USER_COUNT that is a map of a user's address and the number of times that user has
  called the increment function
