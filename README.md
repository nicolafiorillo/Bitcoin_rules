# `BTCR`, a Bitcoin node written in Rust

[![Build/Test](https://github.com/nicolafiorillo/btcr/workflows/CI/badge.svg)](https://github.com/nicolafiorillo/btcr/actions)

Opinionated bitcoin node written in Rust mainly for (my own) educational purposes (both Bitcoin and Rust).

This is my contribute to the Bitcoin spread and adoption.

_Do not use this for production stuff. Stability and expecially security are not guaranteed._

_Moreover, my Rust is still not idiomatic enough._

## Documentations, references, and articles

Here we list some useful documentations, references, and articles for Bitcoin developers gathered during the development of `BTCR`.

- [Elliptic Curve Cryptography](docs/ecc/)
- [Bitcoin Core source code](https://github.com/bitcoin)
- [Bitcoin Wiki](https://en.bitcoin.it/wiki/Main_Page)
- [Bitcoin secp256k1](https://github.com/bitcoin-core/secp256k1)
- [Bitcoin Improvement Proposals (BIPs)](https://github.com/bitcoin/bips)
- [Bitcoin Core architecture overview](https://jameso.be/dev++2018/#1) by [James O'Beirne](https://twitter.com/jamesob)
- [Bitcoin Developer Guides](https://developer.bitcoin.org/devguide/index.html)
- [Bitcoin Tutorials](https://www.herongyang.com/Bitcoin/)
- [CS120: Bitcoin for Developers I](https://learn.saylor.org/course/view.php?id=500)
- [Technical Bitcoin Resources](https://www.lopp.net/bitcoin-information/technical-resources.html) by [Jameson Loop](https://twitter.com/lopp)
- [Bitcoin Development Tools](https://www.lopp.net/bitcoin-information/developer-tools.html) by [Jameson Loop](https://twitter.com/lopp)
- [A developer-oriented series about Bitcoin](http://davidederosa.com/basic-blockchain-programming/) by [Davide De Rosa](https://twitter.com/keeshux)

- [Mastering Bitcoin, 2nd ed.](https://github.com/bitcoinbook/bitcoinbook)
- [Programming Bitcoin](https://github.com/jimmysong/programmingbitcoin)
- [Libbitcoin library](https://github.com/libbitcoin/libbitcoin-system/wiki)
- [Bitcoin Dev Kit](https://github.com/bitcoindevkit)

- [Bitcoinedge initiative](https://bitcoinedge.org/presentations) presentations.
- [Number Theory in Python](https://github.com/Robert-Campbell-256/Number-Theory-Python)

### Articles

- [Elliptic Curve Cryptography: a gentle introduction](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
- [(Some of) the math behind Bech32 addresses](https://medium.com/@meshcollider/some-of-the-math-behind-bech32-addresses-cf03c7496285)

## To do

- [X] Add Github Action to run `cargo test`, `cargo clippy`, and `cargo fmt` on every commit.
- [X] Show compilation status of the project and unit test results in README.md.
- [X] Signing commits.
- [X] Preparing a book for notes.
- [ ] Wrap in container.
- [ ] Substitute `rug` with custom implementation of big integers.
- [ ] Verify array bounds, especially in deserialization.
- [ ] Minimize `clone()` usage.

## sats used for testing

| | |
|----------|----------|
|Network|`Testnet (0x6F)`|
|Address|`mh5KJHiC4DcMBVBgxnx18J2mBmk4n9o9Cr`|
|bitcoins|`0.00009326 BTC`|
|Transaction|[`ea27e55c870cbfb9f2fae55255754752bcdd718ea1f1a1fd6c16f7112fd69c2d`](https://live.blockcypher.com/btc-testnet/tx/ea27e55c870cbfb9f2fae55255754752bcdd718ea1f1a1fd6c16f7112fd69c2d/)|
|Block|[0000000000000006dace4cc7b840e5296a6fa248957b89e87c912d7f3bb396c1](https://live.blockcypher.com/btc-testnet/block/0000000000000006dace4cc7b840e5296a6fa248957b89e87c912d7f3bb396c1/)|

## Development Roadmap

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
  - [X] Base58 decoding
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
  - [ ] Pay-to-Public-Key (P2PK) (_*in progress now*_)
  - [ ] Pay-to-Public-Key-Hash (P2PKH)
  - [ ] Pay-to-Script-Hash (P2SH)
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
- [ ] Segregated witness (Segwit)
- [ ] Seed phrase ([BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki))
- [ ] bech32 address format ([BIP173](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki))
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
  - [ ] [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
  - [ ] [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)
  - [ ] Extended public key
  - [ ] Private key derivation functions (KDF)
  - [ ] Private key derivation from password
- [ ] Private Key generation
  - [ ] Random private key generation
  - [ ] Private key generation from seed
- [ ] Passphrase-protected/encrypted private keys ([BIP38](https://github.com/bitcoin/bips/blob/master/bip-0038.mediawiki))
  - [ ] Encrypted private key

### other (scattered) topics beyond the roadmap
- [ ] Bitcoin scripting language
  - [ ] Pay-to-Multisig (P2MS)
  - [ ] Pay-to-Witness-Public-Key-Hash (P2WPKH)
  - [ ] Pay-to-Witness-Script-Hash (P2WSH)
  - [ ] Pay-to-Taproot (P2TR)
  - [ ] Pay-to-Tapscript (P2TS)
  - [ ] Pay-to-Tapscript-Hash (P2TSH)
  - [ ] Pay-to-Taproot-Script-Hash (P2TRSH)
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
- [ ] Sidechains

## Feedback

Feedback are welcome! Please open an issue or PR.

## Donate

If you want to support: 

- [BTCPayServer](https://priorato.btcpayserver.it/api/v1/invoices?storeId=6ZWNeeMiCdJcAPGVtBG31NMGK3dHjg1xweuMMyGKUsVA&price=1000&currency=SATS)
- [Paypal](https://paypal.me/nicolafiorillo)

