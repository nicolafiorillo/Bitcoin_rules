# `BTCR`, a Bitcoin node written in Rust

[![Build/Test](https://github.com/nicolafiorillo/btcr/workflows/CI/badge.svg)](https://github.com/nicolafiorillo/btcr/actions)

Opinionated bitcoin node written in Rust mainly for (my own) educational purposes (both Bitcoin and Rust).

This is my contribute to the Bitcoin spread and adoption.

_Do not use this for production stuff. Stability and expecially security are not guaranteed._
_Moreover, my Rust is still not idiomatic enough._

## Documentations and references

Here we list some useful documentations and references for Bitcoin developers gathered during the development of `BTCR`.

- [Elliptic Curve Cryptography](docs/ecc/)
- [Bitcoin Core source code](https://github.com/bitcoin)
- [Bitcoin Wiki](https://en.bitcoin.it/wiki/Main_Page)
- [Bitcoin Improvement Proposals (BIPs)](https://github.com/bitcoin/bips)
- [Bitcoin Core architecture overview](https://jameso.be/dev++2018/#1) by [@jamesob](https://twitter.com/jamesob).

- [Mastering Bitcoin, 2nd ed.](https://github.com/bitcoinbook/bitcoinbook)
- [Programming Bitcoin](https://github.com/jimmysong/programmingbitcoin)
- [Libbitcoin library](https://github.com/libbitcoin/libbitcoin-system/wiki)

- [Bitcoinedge initiative](https://bitcoinedge.org/presentations) presentations.

## To do

- [X] Add Github Action to run `cargo test`, `cargo clippy`, and `cargo fmt` on every commit.
- [X] Show compilation status of the project and unit test results in README.md.
- [ ] Generate documentation with rustdoc.
- [ ] Wrap in container.
- [ ] Substitute `rug` with custom implementation of big integers.
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
  - [X] Finite fields implementation
  - [X] Elliptic curves implementation
  - [X] Elliptic curves over finite fields
  - [X] Bitcoin elliptic curve
- [X] Just enough private/public key cryptography and hash functions
  - [X] Hash256 functions
  - [X] Create signatures
    - [X] Deterministic k generation
  - [X] Signing and verification
- [X] Just enough serialization
  - [X] Standard for Efficient Cryptography (SEC) for public key
    - [X] Compressed and uncompressed
  - [X] Distinguished Encoding Rules (DER) for signatures serialization 
  - [X] Base58 encoding
  - [X] Wallet Import Format (WIF) format for private key serialization
  - [X] Variable-length integers (VarInt)
  - [X] Hash160 functions
- [X] Logging
- [X] Transaction structures and serialization
  - [X] Transaction input
  - [X] Transaction output
  - [X] Transaction serialization and deserialization
  - [X] Transaction fees
- [ ] Bitcoin scripting language (_*in progress now*_)
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
- [ ] Seed phrase ([BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki))
- [ ] Configuration
- [ ] User interfaces
  - [ ] REPL
    - [ ] Define commands
    - [ ] Command parsing and execution
    - [ ] Command help
    - [ ] Command history
    - [ ] Command completion
  - [ ] API
  - [ ] Messages (via queues)
- [ ] Wallet
  - [ ] Key management
  - [ ] Generate paper wallet (png QRCode)
- [ ] Hierarchical Deterministic (HD) key derivation
  - [ ] Private key derivation functions (KDF)
  - [ ] Private key derivation from password
- [ ] Private Key generation
  - [ ] Random private key generation
  - [ ] Private key generation from seed
- [ ] Passphrase-protected private keys ([BIP38](https://github.com/bitcoin/bips/blob/master/bip-0038.mediawiki))


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
