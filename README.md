# Tamagotchi

## Food

Copy of SNIP-20 reference implementation.

## Market

Contract for buying FOOD tokens.

## Pet

Contract for instancing and feeding pet.

# CosmWasm Test task

## Intro
Some of us still remember the times back when we had our first digital pets - Tamagochi. As kids it was very challenging to keep your pet fed, clean and happy.

## Challenge

Fast forward 20 years. Lets implement a basic Tamagochi logic.

- Create a SNIP-20 token contract - called `FOOD`.

- Create a CosmWasm `Market` contract that gives the ability to the user to purchase SNIP-20 tokens to feed to their Pet.

- Create a CosmWasm `Pet` contract which implements a digital pet (Tamagochi like) interface, allowing one to feed the creature with SNIP-20/21/22/23 tokens.


## Requirements / restrictions

- `FOOD` tokens can only be minted by the Market contract.
- `FOOD` tokens can be purchased only with the native SCRT token in ratio `SCRT/FOOD` - `1:100`.
- Feeding the `Pet` (sending the `Pet` contract a `FOOD` token) should result in burning `FOOD` tokens.
- `Pet` should implement a countdown clock, which gets pushed `4` hours ahead every time the pet gets fed.
- If you miss feeding the `Pet` for more than 4 hours then the `Pet` will starve.
- If a `Pet` starves to death then it cannot be fed any longer so `FOOD` tokens shall be returned to the sender rather then burned.


## References
- [scrt.network](https://scrt.network)
- https://github.com/SecretFoundation/SNIPs/blob/master/SNIP-20.md
- cosmwasm.com
- github.com/hackbg/fadroma
