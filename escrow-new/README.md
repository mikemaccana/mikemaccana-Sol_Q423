# Escrow Program

I followed [Dean's video](https://drive.google.com/file/d/1mr5iCSisJNnDmZryyHE7n_BXg6FViwzE/view) but rewrote the code.

- I used the 'multiple' template, which focuses on instructions rather than contexts. Each instruction is simply a module with a function called `handler()`

- Calling the Account Contraints struct 'Take' or 'Make' etc is unintuitive to me. A struct is not a verb. It does not do things. So I name my structs `TakeOfferAccountConstraints` or `MakeOfferAccountConstraints`

- Likewise `recieve` is `desired_amount` - lots of the explanations spoken by people in the videos made better names for the variables used.
