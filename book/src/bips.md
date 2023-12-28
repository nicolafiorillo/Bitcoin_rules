## Bitcoin Improvement Proposals - BIPs

### BIP 34
Introduces a new block version number to the Bitcoin protocol and it has been implemented officially in 2012.

BIP34 mandates that the input script of this coinbase transaction must contain the block's height as a serialized integer.
The block height is serialized as a 4-byte integer as first element in the coinbase scriptsig.

Its primary purpose is to avoid having the same id for different coinbase transactions (in different blocks), including a serialized block height in it.
This alse ensures that the block's height is permanently recorded within the chain, making it easily accessible without having to parse the entire chain history.

### BIP 9
BIP 9 is a method of rolling out soft forks in a way that allows miners to signal that they are ready and willing to upgrade, while providing a fail-safe mechanism that automatically activates the soft fork after a certain period of time.
However, BIP 9 does not specify how the soft fork is to be implemented, only how it is to be signaled.

Current assignments: https://github.com/bitcoin/bips/blob/master/bip-0009/assignments.mediawiki
