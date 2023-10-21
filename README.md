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
