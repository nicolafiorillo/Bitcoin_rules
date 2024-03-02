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

> _There's no better way to learn how something works than by trying to reproduce it._
>
> Paul Graham, "[How to Do Great Work](https://paulgraham.com/greatwork.html)"

A Bitcoin node is the backbone of the Bitcoin network, functioning as a participant that validates transactions, maintains a copy of the blockchain, and ensure the integrity of the blockchain by enforcing consensus rules.

The `Bitcoin_rules!` project is a comprehensive endeavor aimed at exploring the intricacies of Bitcoin's protocol, staring from the very basics of the protocol and building up to a fully functional Bitcoin node, (almost) from scratch. `Bitcoin_rules!` goes beyond the surface-level understanding of Bitcoin: we delve into the technical aspects of building a full node, offering a deep dive into the inner workings of the Bitcoin network, protocol, and consensus.

Moreover, consider this as a contribute to the Bitcoin spread and adoption.

See [references](REFERENCES.md) for documentations, references, and articles used building `Bitcoin_rules!`. See also [roadmap](ROADMAP.md) for the list of features and milestones.

> **Do not use this code in production stuff. Completeness, stability, and expecially security are not guaranteed.**

## Getting started

At time `main()` is almost empty. We are working on structures and algorithms.
For the moment, clone the repo and build it with:

```shell
cargo build
```

Run the tests:

```shell
cargo test --lib --bins
```

And run console (cli) app:

```
cargo run
```

## Feedback

Feedback are welcome! Please open an [issue](https://github.com/nicolafiorillo/Bitcoin_rules/issues) or [PR](https://github.com/nicolafiorillo/Bitcoin_rules/pulls).

## Support

If you want to support this project, you can donate to: 

- [BTCPayServer](https://priorato.btcpayserver.it/api/v1/invoices?storeId=6ZWNeeMiCdJcAPGVtBG31NMGK3dHjg1xweuMMyGKUsVA&price=1000&currency=SATS)
- [Paypal](https://paypal.me/nicolafiorillo)

## Acknowledgments
- [Rust](https://www.rust-lang.org/) for being a great language.
- [Satoshi Nakamoto](https://www.metzdowd.com/pipermail/cryptography/2008-October/014810.html) for bringing us here.
