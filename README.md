<div align="center">
 <img src="https://raw.githubusercontent.com/nicolafiorillo/Bitcoin_rules/main/book/images/bitcoin_rules.webp" width="700" alt="Bitcoin_rules!" style="border-radius: 5%">
 <br>
 <strong>
   a Bitcoin (the protocol) node written in Rust mainly for educational purposes
 </strong>
</div>

## Status

[![Build/Test](https://github.com/nicolafiorillo/Bitcoin_rules/workflows/CI/badge.svg)](https://github.com/nicolafiorillo/Bitcoin_rules/actions)

## Motivation and other info

The `Bitcoin_rules!` project is a comprehensive endeavor aimed at exploring the intricacies of Bitcoin's protocol, staring from the very basics of the protocol and building up to a fully functional Bitcoin node, (almost) from scratch. `Bitcoin_rules!` goes beyond the surface-level understanding of Bitcoin: we delve into the technical aspects of building a full node, offering a deep dive into the inner workings of the Bitcoin network, protocol, and consensus.

Moreover, consider this as a contribute to the Bitcoin spread and adoption.

> **Do not use this code in production. Completeness, stability, and expecially security are not guaranteed.**

## Getting started

We are working on structures and algorithms and there is nothing to run yet. For the moment, clone the repo and build it with:

```shell
cargo build
```

### Run all tests

```shell
cargo test --lib --bins
```

### Run unit tests only (fast)

```shell
cargo test --lib --bins
```

### Run unit tests and integration tests (slow)

```shell
cargo test
```

### Run the node (cli app)

`DATABASE_URL` must be set as environment variable.

Prepare database: 

```
diesel setup
```
then run

```
cargo run --bin brn
```

## Roadmap
  The project is ramping up and the roadmap has not time references or milestones. Not yet.
<details>
  <summary>Click here to see the roadmap</summary>

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
- [X] Bitcoin scripting language
  - [ ] see [Opcodes](ROADMAP_OPS.md)
- [ ] Transaction validation
  - [X] Pay-to-Public-Key (P2PK)
  - [X] Pay-to-Public-Key-Hash (P2PKH)
  - [X] Multisig (OP_CHECKMULTISIG)
  - [X] Custom data (OP_RETURN)
  - [ ] Pay-to-Script-Hash (P2SH, BIP13)
  Fees
  - [ ] Fee estimation (from external source)
- [ ] Block structures and serialization
  - [X] Block header
  - [X] Target-bits-difficulty
  - [ ] Proof-of-work
- [ ] Block validation
  - [ ] Block reward
  - [X] Difficulty adjustment
- [X] Peer-to-peer network
  - [ ] Network messages serialization and deserialization
  - [ ] Peer-to-peer communication (_*in progress now*_)
    - [ ] Use stateful property-based testing for network communication validation
  - [ ] Peer discovery
  - [ ] Peer-to-peer synchronization
  - [ ] Gossip protocol
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
  - [ ] Vanity address
  - [ ] Generate paper wallet (png QRCode)
  - [ ] Generate/send new transaction
  - [ ] Balance
  - [ ] History
  - [ ] Fee estimation (from local chain data)
  - [ ] CoinJoin
  - [ ] Coin selection
  - [ ] Coin control
  - [ ] Hierarchical Deterministic (HD) key derivation
    - [ ] [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
    - [ ] [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)
    - [ ] Extended public key
    - [ ] Private key derivation functions (KDF)
    - [ ] Private key derivation from password
- [ ] Private Key generation
  - [X] Random private key generation
  - [ ] Private key generation from seed
- [ ] Passphrase-protected/encrypted private keys ([BIP38](https://github.com/bitcoin/bips/blob/master/bip-0038.mediawiki))
  - [ ] Encrypted private key
- [ ] Other
  - [ ] Multi-Party Computation (MPC)
  - [ ] Payjoin (BIP78)
  - [ ] Partially signed bitcoin transactions (BIP174, BIP370)
  - [ ] Stale-blocks

## Other (scattered) topics beyond the roadmap

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
- [ ] Lightning network
- [ ] Payment channels
- [ ] Schnorr signatures
- [ ] Taproot
- [ ] Sidechains
</details>

<details>
  <summary>Click here to see the roadmap of script operators implementation</summary>

- [X] 0x00 - `OP_0`
- [ ] 0x4C - `OP_PUSHDATA1`
- [ ] 0x4D - `OP_PUSHDATA2`
- [ ] 0x4E - `OP_PUSHDATA4`
- [X] 0x4F - `OP_1NEGATE`
- [X] 0x50 - `OP_RESERVED` (as reserved)
- [X] 0x51 - `OP_1`
- [X] 0x52 - `OP_2`
- [X] 0x53 - `OP_3`
- [X] 0x54 - `OP_4`
- [X] 0x55 - `OP_5`
- [X] 0x56 - `OP_6`
- [X] 0x57 - `OP_7`
- [X] 0x58 - `OP_8`
- [X] 0x59 - `OP_9`
- [X] 0x5A - `OP_10`
- [X] 0x5B - `OP_11`
- [X] 0x5C - `OP_12`
- [X] 0x5D - `OP_13`
- [X] 0x5E - `OP_14`
- [X] 0x5F - `OP_15`
- [X] 0x60 - `OP_16`
- [X] 0x61 - `OP_NOP`
- [X] 0x62 - `OP_VER` (as reserved)
- [X] 0x63 - `OP_IF`
- [X] 0x64 - `OP_NOTIF`
- [X] 0x65 - `OP_VERIF` (as reserved)
- [X] 0x66 - `OP_VERNOTIF` (as reserved)
- [X] 0x67 - `OP_ELSE`
- [X] 0x68 - `OP_ENDIF`
- [X] 0x69 - `OP_VERIFY`
- [X] 0x6A - `OP_RETURN`
- [X] 0x6B - `OP_TOALTSTACK`
- [X] 0x6C - `OP_FROMALTSTACK`
- [X] 0x6D - `OP_2DROP`
- [X] 0x6E - `OP_2DUP`
- [X] 0x6F - `OP_3DUP`
- [X] 0x70 - `OP_2OVER`
- [X] 0x71 - `OP_2ROT`
- [X] 0x72 - `OP_2SWAP`
- [X] 0x73 - `OP_IFDUP`
- [X] 0x74 - `OP_DEPTH`
- [X] 0x75 - `OP_DROP`
- [X] 0x76 - `OP_DUP`
- [X] 0x77 - `OP_NIP`
- [X] 0x78 - `OP_OVER`
- [X] 0x79 - `OP_PICK`
- [X] 0x7A - `OP_ROLL`
- [X] 0x7B - `OP_ROT`
- [X] 0x7C - `OP_SWAP`
- [X] 0x7D - `OP_TUCK`
- [X] 0x7E - `OP_CAT` (as deprecated)
- [X] 0x7F - `OP_SUBSTR` (as deprecated)
- [X] 0x80 - `OP_LEFT` (as deprecated)
- [X] 0x81 - `OP_RIGHT` (as deprecated)
- [X] 0x82 - `OP_SIZE`
- [X] 0x83 - `OP_INVERT` (as deprecated)
- [X] 0x84 - `OP_AND` (as deprecated)
- [X] 0x85 - `OP_OR` (as deprecated)
- [X] 0x86 - `OP_XOR` (as deprecated)
- [X] 0x87 - `OP_EQUAL`
- [X] 0x88 - `OP_EQUALVERIFY`
- [X] 0x89 - `OP_RESERVED1` (as reserved)
- [X] 0x8A - `OP_RESERVED2` (as reserved)
- [X] 0x8B - `OP_1ADD`
- [X] 0x8C - `OP_1SUB`
- [X] 0x8D - `OP_2MUL` (as deprecated)
- [X] 0x8E - `OP_2DIV` (as deprecated)
- [X] 0x8F - `OP_NEGATE`
- [X] 0x90 - `OP_ABS`
- [X] 0x91 - `OP_NOT`
- [X] 0x92 - `OP_0NOTEQUAL`
- [X] 0x93 - `OP_ADD`
- [X] 0x94 - `OP_SUB`
- [X] 0x95 - `OP_MUL` (as deprecated)
- [X] 0x96 - `OP_DIV` (as deprecated)
- [X] 0x97 - `OP_MOD` (as deprecated)
- [X] 0x98 - `OP_LSHIFT` (as deprecated)
- [X] 0x99 - `OP_RSHIFT` (as deprecated)
- [X] 0x9A - `OP_BOOLAND`
- [X] 0x9B - `OP_BOOLOR`
- [X] 0x9C - `OP_NUMEQUAL`
- [X] 0x9D - `OP_NUMEQUALVERIFY`
- [X] 0x9E - `OP_NUMNOTEQUAL`
- [X] 0x9F - `OP_LESSTHAN`
- [X] 0xA0 - `OP_GREATERTHAN`
- [X] 0xA1 - `OP_LESSTHANOREQUAL`
- [X] 0xA2 - `OP_GREATERTHANOREQUAL`
- [X] 0xA3 - `OP_MIN`
- [X] 0xA4 - `OP_MAX`
- [X] 0xA5 - `OP_WITHIN`
- [X] 0xA6 - `OP_RIPEMD160`
- [X] 0xA7 - `OP_SHA1`
- [X] 0xA8 - `OP_SHA256`
- [X] 0xA9 - `OP_HASH160`
- [X] 0xAA - `OP_HASH256`
- [ ] 0xAB - `OP_CODESEPARATOR`
- [X] 0xAC - `OP_CHECKSIG`
- [ ] 0xAD - `OP_CHECKSIGVERIFY`
- [X] 0xAE - `OP_CHECKMULTISIG`
- [ ] 0xAF - `OP_CHECKMULTISIGVERIFY`
- [X] 0xB0 - `OP_NOP1` (as ignored)
- [ ] 0xB1 - `OP_CHECKLOCKTIMEVERIFY`
- [ ] 0xB2 - `OP_CHECKSEQUENCEVERIFY`
- [X] 0xB3 - `OP_NOP4` (as ignored)
- [X] 0xB4 - `OP_NOP5` (as ignored)
- [X] 0xB5 - `OP_NOP6` (as ignored)
- [X] 0xB6 - `OP_NOP7` (as ignored)
- [X] 0xB7 - `OP_NOP8` (as ignored)
- [X] 0xB8 - `OP_NOP9` (as ignored)
- [X] 0xB9 - `OP_NOP10` (as ignored)
- [ ] 0xBA - `OP_CHECKSIGADD`
- [X] 0xFD - `OP_PUBKEY`
- [X] 0xFE - `OP_PUBKEYHASH`
- [X] 0xFF - `OP_INVALIDOPCODE`

</details>

## References
Where we list some useful resources for Bitcoin developers gathered during the development of `Bitcoin_rules!`.

<details>
  <summary>Click here to see the some useful resources</summary>

### History
- [The Complete Satoshi](https://satoshi.nakamotoinstitute.org/)
- [The Bitcoin Legacy Project](https://www.thebitcoinlegacyproject.org/)
- [The Incomplete History of Bitcoin Development](https://b10c.me/blog/004-the-incomplete-history-of-bitcoin-development/#)
- [Bitcoin 101: past, present and future ](https://www.musclesatz.com/articles/bitcoin-past-present-future)

### Documentations, references, and articles
- [Elliptic Curve Cryptography](docs/ecc/)
- [Bitcoin Core source code](https://github.com/bitcoin)
- [Bitcoin Wiki](https://en.bitcoin.it/wiki/Main_Page)
- [Bitcoin secp256k1](https://github.com/bitcoin-core/secp256k1)
- [Bitcoin Improvement Proposals (BIPs)](https://github.com/bitcoin/bips)
- [Script](https://en.bitcoin.it/wiki/Script)
- [Bitcoin Core architecture overview](https://jameso.be/dev++2018/#1) by [James O'Beirne](https://twitter.com/jamesob)
- [Bitcoin Developer Guides](https://developer.bitcoin.org/devguide/index.html)
- [Bitcoin Developer Reference](https://developer.bitcoin.org/reference/index.html)
- [Bitcoin Tutorials](https://www.herongyang.com/Bitcoin/)
- [CS120: Bitcoin for Developers I](https://learn.saylor.org/course/view.php?id=500)
- [Technical Bitcoin Resources](https://www.lopp.net/bitcoin-information/technical-resources.html) by [Jameson Loop](https://twitter.com/lopp)
- [Bitcoin Development Tools](https://www.lopp.net/bitcoin-information/developer-tools.html) by [Jameson Loop](https://twitter.com/lopp)
- [A developer-oriented series about Bitcoin](http://davidederosa.com/basic-blockchain-programming/) by [Davide De Rosa](https://twitter.com/keeshux)
- [Libbitcoin library](https://github.com/libbitcoin/libbitcoin-system/wiki)
- [Bitcoin Dev Kit](https://github.com/bitcoindevkit)
- [Bitcoinedge initiative](https://bitcoinedge.org/presentations) presentations.
- [Number Theory in Python](https://github.com/Robert-Campbell-256/Number-Theory-Python)
- [learn me a bitcoin](https://learnmeabitcoin.com/) by [Greg Walker](https://twitter.com/in3rsha)
- [Intel® Digital Random Number Generator (DRNG)](https://www.intel.com/content/dam/develop/external/us/en/documents/drng-software-implementation-guide-2-1-185467.pdf)
- [Elliptic Curve Cryptography: a gentle introduction](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
- [(Some of) the math behind Bech32 addresses](https://medium.com/@meshcollider/some-of-the-math-behind-bech32-addresses-cf03c7496285)
- [Bitcoins the hard way: Using the raw Bitcoin protocol](http://www.righto.com/2014/02/bitcoins-hard-way-using-raw-bitcoin.html)
- [MIT Bitcoin Club](https://www.youtube.com/@MITBitcoinClub/videos)
- [What are hash functions used for in bitcoin?](https://bitcoin.stackexchange.com/questions/120418/what-are-hash-functions-used-for-in-bitcoin)
- [The difficulty in the bitcoin protocol](https://leftasexercise.com/2018/06/04/the-difficulty-in-the-bitcoin-protocol/)
- [Ch12: Something on Bits, Target, Difficulty](https://medium.com/@ackhor/ch12-something-on-bits-target-difficulty-f863134061fb)
- [The Challenges of Optimizing Unspent Output Selection](https://blog.lopp.net/the-challenges-of-optimizing-unspent-output-selection/)
- [How does block synchronization work in Bitcoin Core today?](https://bitcoin.stackexchange.com/questions/121292/how-does-block-synchronization-work-in-bitcoin-core-today)
- [Bloom Filters](https://samwho.dev/bloom-filters/)
- [Testnet](https://bitcoinwiki.org/wiki/testnet)
- [Networking](https://learnmeabitcoin.com/technical/networking/)
- [P2SH](https://learnmeabitcoin.com/technical/script/p2sh/)
- [Protocol rules](https://en.bitcoin.it/wiki/Protocol_rules)

### Books
- [Mastering Bitcoin, 2nd ed.](https://github.com/bitcoinbook/bitcoinbook)
- [Programming Bitcoin](https://github.com/jimmysong/programmingbitcoin)

### BIPs
- [BIP 39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)

### Literature
- ["_Bitcoin was not forged in a vacuum_"](https://nakamotoinstitute.org/literature/)

### Courses
- [MIT, MAS.S62-Spring 2018, Cryptocurrency Engineering and Design](https://www.youtube.com/watch?v=l2iv2MiGaYI)
- [bitcoin-curriculum](https://github.com/chaincodelabs/bitcoin-curriculum)
- [Seminar for Bitcoin and Lightning protocol](https://chaincode.gitbook.io/seminars/)
- [Plan B Network](https://planb.network/)

### Tools
- [Bitcoin Script Debugger](https://github.com/bitcoin-core/btcdeb)
- [Hashing Online Tools](https://emn178.github.io/online-tools/index.html)
- [Bitnodes](https://bitnodes.io/)

### Interesting stuff
- [REWARD offered for hash collisions for SHA1, SHA256, RIPEMD160 and other](https://bitcointalk.org/index.php?topic=293382.0) by [Peter Todd](https://twitter.com/peterktodd)
- [Bitcoin's Academic Pedigree](https://queue.acm.org/detail.cfm?id=3136559) by [Arvind Narayanan](https://twitter.com/random_walker)
- [BitBox02: Diceware lookup table ](https://bitbox.swiss/bitbox02/BitBox_Diceware_LookupTable.pdf)
- [Recovering Bitcoin private keys using weak signatures from the blockchain](https://web.archive.org/web/20160308014317/http://www.nilsschneider.net/2013/01/28/recovering-bitcoin-private-keys.html)
- [Bitcoin private key database](https://isidoroghezzi.bitbucket.io/directory-js/?page=1&network=0)
- [Satoshi - Sirius emails 2009-2011](https://mmalmi.github.io/satoshi/)
- [Know Your Coin Privacy](https://kycp.org/)
- [Debugging Bitcoin Core](https://github.com/fjahr/debugging_bitcoin)
- [Bitcoin Traffic Sniffer and Analyzer](https://www.codeproject.com/Articles/895917/Bitcoin-Traffic-Sniffer-and-Analyzer)
- [TimechainStats](https://timechainstats.com/)
- [ECDSA: Revealing the private key, if nonce known (NIST256p)](https://asecuritysite.com/cracking/ecd2)
- [How to compile Bitcoin Core and run the unit and functional tests](https://jonatack.github.io/articles/how-to-compile-bitcoin-core-and-run-the-tests)
- [Using debuggers with Bitcoin Core](https://gist.github.com/LarryRuane/8c6e8de82f6e2b360ca54dd751388af6)

### People
- [Peter Todd](https://petertodd.org/)
- [Hal Finney](https://en.wikipedia.org/wiki/Hal_Finney_(computer_scientist))
- [Pieter Wuille](https://twitter.com/pwuille)
- [Jimmi Song](https://medium.com/@jimmysong)
- [Mike Hearn](https://plan99.net/~mike/index.html)
- [Jameson Lopp](https://github.com/jlopp)

### Communities
- [Bitcoin Forum](https://bitcointalk.org/index.php)
- [Bitcoin Stack Exchange](https://bitcoin.stackexchange.com/)
- [Delving Bitcoin](https://delvingbitcoin.org)
- [Bitcoin Optech](https://bitcoinops.org/)

### `Rust` 

- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
- [Rust Lifetimes: A Complete Guide to Ownership and Borrowing](https://earthly.dev/blog/rust-lifetimes-ownership-burrowing/)
- [Rusts Module System Explained](https://aloso.github.io/2021/03/28/module-system.html)
- [Exploring Binary](https://www.exploringbinary.com/)

### `PostgreSQL`

- [Data Types](https://www.postgresql.org/docs/current/datatype.html)
- [postgresql/Diesel Rust types](https://gist.github.com/steveh/7c7145409a5eed6b698ee8b609b6d1fc)
</details>

## Feedback

All kind of feedback are welcome! Please open an [issue](https://github.com/nicolafiorillo/Bitcoin_rules/issues) or [PR](https://github.com/nicolafiorillo/Bitcoin_rules/pulls).

## Support

If you want to support this project, you can donate to: 

- [BTCPayServer](https://priorato.btcpayserver.it/api/v1/invoices?storeId=6ZWNeeMiCdJcAPGVtBG31NMGK3dHjg1xweuMMyGKUsVA&price=1000&currency=SATS)
- [Paypal](https://paypal.me/nicolafiorillo)
