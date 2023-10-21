<div align="center">
 <img src="https://raw.githubusercontent.com/nicolafiorillo/Bitcoin_rules/main/images/bitcoin_rules.webp" width="800" alt="Bitcoin_rules!" style="border-radius: 5%">
 <br>
 <strong>
   a Bitcoin (the protocol) node written in Rust mainly for educational purposes
 </strong>
</div>

## Status

[![Build/Test](https://github.com/nicolafiorillo/Bitcoin_rules/workflows/CI/badge.svg)](https://github.com/nicolafiorillo/Bitcoin_rules/actions)

## Motivation and other info

This is a contribute to the Bitcoin spread and adoption. _Do not use this for production stuff. Completeness, stability, and expecially security are not guaranteed._

See [references](REFERENCES.md) for documentations, references, and articles used building this project.

See also [roadmap](ROADMAP.md).

## To do

- [X] Add Github Action to run `cargo test`, `cargo clippy`, and `cargo fmt` on every commit.
- [X] Show compilation status of the project in README.md.
- [ ] Show unit test results in README.md.
- [X] Signing commits.
- [X] Preparing a book for notes.
- [ ] Use a global Error enum.
- [ ] Wrap in container.
- [ ] Verify array bounds, especially in deserialization.
- [ ] Minimize `clone()` usage.

## sats used for testing

Testnet (`0x6F`)

|Address|Value|Transaction|Status|
|-|-|-|-|
|`mh5kjhic4dcmbvbgxnx18j2mbmk4n9o9cr`|`0.00009326 tBTC`|[`ea27e55c870cbfb9f2fae55255754752bcdd718ea1f1a1fd6c16f7112fd69c2d`](https://live.blockcypher.com/btc-testnet/tx/ea27e55c870cbfb9f2fae55255754752bcdd718ea1f1a1fd6c16f7112fd69c2d/)|lost in action|
|`n3adrwnBHvrQ26omXe4bhBFg5VPBP1ZiDK`|`0.00005930 tBTC`|[`2d5830f2a3dd491a9e0fa1d842c4b22debaf3133158fb7f992041f3dd2eaf2fb`](https://live.blockcypher.com/btc-testnet/tx/2d5830f2a3dd491a9e0fa1d842c4b22debaf3133158fb7f992041f3dd2eaf2fb/)|lost in action|
|`mnfoJ71rZhhdCJs372B5CiD5DrYWQyR1UZ`|`0.00006484 tBTC`|[`2ad00c8e79a0c62c613d51e4669a14a4a94302e487be38ce1316a2ecc705c646`](https://live.blockcypher.com/btc-testnet/tx/2ad00c8e79a0c62c613d51e4669a14a4a94302e487be38ce1316a2ecc705c646/)|lost in action|
|`mnfoJ71rZhhdCJs372B5CiD5DrYWQyR1UZ`|`0.00006484 tBTC`|[`2ad00c8e79a0c62c613d51e4669a14a4a94302e487be38ce1316a2ecc705c646`](https://live.blockcypher.com/btc-testnet/tx/2ad00c8e79a0c62c613d51e4669a14a4a94302e487be38ce1316a2ecc705c646/)|unspent|

## Building

At time `main()` is almost empty but tests can be executed with:

```
cargo test
```

## Feedback

Feedback are welcome! Please open an issue or PR.

## Support

If you want to support this project, you can donate to: 

- [BTCPayServer](https://priorato.btcpayserver.it/api/v1/invoices?storeId=6ZWNeeMiCdJcAPGVtBG31NMGK3dHjg1xweuMMyGKUsVA&price=1000&currency=SATS)
- [Paypal](https://paypal.me/nicolafiorillo)

## Acknowledgments
- [Rust](https://www.rust-lang.org/) for being a great language.
- [Satoshi Nakamoto](https://www.metzdowd.com/pipermail/cryptography/2008-October/014810.html) for bringing us here.
