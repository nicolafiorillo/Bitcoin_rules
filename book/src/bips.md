## Bitcoin Improvement Proposals - BIPs

### BIP 34
Introduces a new block version number to the Bitcoin protocol and it has been implemented officially in 2012.

BIP34 mandates that the input script of this coinbase transaction must contain the block's height as a serialized integer.
The block height is serialized as a 4-byte integer as first element in the coinbase scriptsig.

Its primary purpose is to avoid having the same id for different coinbase transactions (in different blocks), including a serialized block height in it.
This alse ensures that the block's height is permanently recorded within the chain, making it easily accessible without having to parse the entire chain history.

