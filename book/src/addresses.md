# Addresses

An address is a convenient way to communicate which script needs to go on the blockchain.

When a wallet sees an address, it recognizes the address type and produces the suitable script to manage it in the transaction.

## In `BTRC` code

The `address` function:
```rust,no_run,noplayground
{{#include ../../src/keys/key.rs:fn_address}}
```

## Address types

### Pay To Public Key - P2PK

Lock coins so that only the holder of the private key corresponding to the public key K may spend this coin.

This was used at the beginning of the chain, but it is not used anymore because:
- it reveals the public key on the chain
- use more space in transaction

### Pay To Public Key Hash - P2PKH
Lock coins so that a signature from a specific key is required. That means that only the holder of the private key corresponding to the public key hash160(K) may spend this coin.

Starts with:

| Example           | Network | version prefix |
|                   |         |                |
| `1...`            | mainnet | 0x00           |
| `m...` or `n...`  | testnet | 0x6F           |

Locking script:
  
    OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG

Unlocking script:
  
    <sig> <pubKey>

### Pay To Script Hash - P2SH
Lock coins with some conditions.

Starts with:

| Example | Network | version prefix |
|         |         |                |
| `3...`  | mainnet | 0x05           |
| `2...`  | testnet | 0xC4           |

Locking script:
  
    <TODO>

Unlocking script:
  
    <TODO>


## Other references

- [List of address prefixes](https://en.bitcoin.it/wiki/List_of_address_prefixes)

---

P2SH    3...
P2WPKH
P2WSH





## SegWit - bech32

Checksummed base32 encoding format used for native segwit addresses. It is defined in [BIP-173](
Mainnet P2WPKH: bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4
Testnet P2WPKH: tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx
Mainnet P2WSH: bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3
Testnet P2WSH: tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7

5...    private key
bc1...


