# `BTCR`, a Bitcoin node written in Rust

Opinionated bitcoin node written in Rust mainly for (my own) educational purposes (both Bitcoin and Rust).

_Do not use this for production stuff. Stability and expecially security are not guaranteed._

## Documentations and references

Here we list some useful documentations and references for Bitcoin developers gathered during the development of `BTCR`.

- [Bitcoin Core architecture overview](https://jameso.be/dev++2018/#1) by [@jamesob](https://twitter.com/jamesob).
- [Bitcoinedge initiative](https://bitcoinedge.org/presentations) presentations.
- [Elliptic Curve Cryptography](docs/ecc/)
- [Mastering Bitcoin, 2nd ed.](https://github.com/bitcoinbook/bitcoinbook)
- [Programming Bitcoin](https://github.com/jimmysong/programmingbitcoin)

## To do

- [ ] Add Github Action to run `cargo test`, `cargo clippy`, and `cargo fmt` on every commit.
- [ ] Show compilation status of the project and unit test results in README.md.
- [ ] Wrap in container.
<!-- - [ ] Add Github Action to run `cargo doc` and publish the documentation to Github Pages.
- [ ] Add Github Action to run `cargo audit` on every commit.
- [ ] Add Github Action to run `cargo bench` on every commit. -->

## bitcoins used for testing

| | |
|----------|----------|
|Network|`Testnet (0x6F)`|
|Address|`mh5KJHiC4DcMBVBgxnx18J2mBmk4n9o9Cr`|
|bitcoins|`0.00009326 BTC`|
|Transaction|[`ea27e55c870cbfb9f2fae55255754752bcdd718ea1f1a1fd6c16f7112fd69c2d`](https://live.blockcypher.com/btc-testnet/tx/ea27e55c870cbfb9f2fae55255754752bcdd718ea1f1a1fd6c16f7112fd69c2d/)|
|Block|[0000000000000006dace4cc7b840e5296a6fa248957b89e87c912d7f3bb396c1](https://live.blockcypher.com/btc-testnet/block/0000000000000006dace4cc7b840e5296a6fa248957b89e87c912d7f3bb396c1/)|

## Roadmap

- [X] Elliptic Curve Cryptography
- [X] Just enough cryptography and hash functions
- [X] Just enough serialization
- [ ] Transaction structures and serialization (_*work in progress now*_)
- [ ] Bitcoin scripting language
- [ ] Transaction validation
  - [ ] Pay-to-script-hash (P2SH)
- [ ] Block structures and serialization
  - [ ] Proof-of-work
- [ ] Peer-to-peer network
  - [ ] Peer discovery
  - [ ] Peer-to-peer communication
  - [ ] Peer-to-peer synchronization
- [ ] Payment protocol and verification
  - [ ] SPV
  - [ ] Merkle tree
- [ ] Bloom filters
- [ ] Segrated witness (Segwit)
- [ ] Wallet
  - [ ] Key management
  - [ ] Address generation
  - [ ] Transaction creation
  - [ ] Transaction signing
  - [ ] Transaction broadcasting

### other (scattered) topics beyond the roadmap

- [ ] Block chain
  - [ ] Block chain data structure
  - [ ] Block chain validation
  - [ ] Block chain synchronization
  - [ ] Block chain reorganization
  - [ ] Block chain pruning
  - [ ] Block chain checkpoints
  - [ ] Block chain forks
  - [ ] Block chain orphan blocks
- [ ] Mining
  - [ ] Mining pool
  - [ ] Mining pool reward
  - [ ] Mining pool payout
  - [ ] Mining pool difficulty
  - [ ] Mining pool block reward
  - [ ] Mining pool block reward distribution
- [ ] Block validation
  - [ ] Block reward
  - [ ] Difficulty adjustment 
- [ ] SPV
- [ ] Lightning network
- [ ] Payment channels
- [ ] Schnorr signatures
- [ ] Taproot
<!-- - [ ] MAST -->
<!-- - [ ] Confidential transactions -->
<!-- - [ ] Mimblewimble -->
- [ ] Sidechains
<!-- - [ ] Drivechain -->

## Feedback

Feedback are welcome! Please open an issue or PR.
