<div align="center">
 <img src="https://raw.githubusercontent.com/nicolafiorillo/Bitcoin_rules/main/images/bitcoin_rules.webp" width="800" alt="Bitcoin_rules!" style="border-radius: 5%">
 <br>
 <strong>
   a Bitcoin (the protocol) node written in Rust mainly for educational purposes
 </strong>
</div>

## Status

[![Build/Test](https://github.com/nicolafiorillo/Bitcoin_rules/workflows/CI/badge.svg)](https://github.com/nicolafiorillo/Bitcoin_rules/actions)

## Motivation and disclaimer

This is a humble contribute to the Bitcoin spread and adoption.

_Do not use this for production stuff. Completeness, stability, and expecially security are not guaranteed._

## Documentations, references, and articles

See [REFERENCES](REFERENCES.md).

## Features development roadmap

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
    - [ ] from wif
    - [X] to wif
  - [X] Variable-length integers (VarInt)
  - [X] Hash160 functions
- [X] Logging
- [X] Transaction structures and serialization
  - [X] Transaction input
  - [X] Transaction output
  - [X] Transaction serialization and deserialization
  - [X] Transaction fees
- [X] Bitcoin scripting language (_*in progress now*_)
  - [X] Pay-to-Public-Key (P2PK)
  - [ ] Implement all opcodes (_*in progress now*_)
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

## To do

- [X] Add Github Action to run `cargo test`, `cargo clippy`, and `cargo fmt` on every commit.
- [X] Show compilation status of the project and unit test results in README.md.
- [X] Signing commits.
- [X] Preparing a book for notes.
- [ ] Use a global Error enum.
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

## Building

At time `main()` is empty but you can run unit tests with:

  cargo test

## Contributors

- [Nicola Fiorillo](https://www.nicolafiorillo.com) (author and maintainer)

## Feedback

Feedback are welcome! Please open an issue or PR.

## Support

If you want to support this project, you can donate to: 

- [BTCPayServer](https://priorato.btcpayserver.it/api/v1/invoices?storeId=6ZWNeeMiCdJcAPGVtBG31NMGK3dHjg1xweuMMyGKUsVA&price=1000&currency=SATS)
- [Paypal](https://paypal.me/nicolafiorillo)

## Acknowledgments
- [Rust](https://www.rust-lang.org/) for being a great language.
- [Ludovico Einaudi](https://en.wikipedia.org/wiki/Ludovico_Einaudi) for his music during the development of this project.
- [Satoshi Nakamoto](https://www.metzdowd.com/pipermail/cryptography/2008-October/014810.html) for bringing us here.
