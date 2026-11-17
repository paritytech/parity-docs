---
title: Cryptography
description: Learn which signature schemes, key types, and hash functions the Polkadot SDK provides through sp_core and sp_runtime, and how to work with keys using subkey.
categories: Basics, Protocol Internals
---

# Cryptography

## Introduction

The Polkadot SDK uses cryptography to sign and verify transactions, to identify block authors and other session key holders, and to hash blocks and storage keys. The SDK is generic over the signature scheme: any scheme that implements the [`Pair`](https://paritytech.github.io/polkadot-sdk/master/sp_core/crypto/trait.Pair.html) trait from `sp_core` can be used.

This page describes the signature schemes and key types available in `sp_core`, how a runtime chooses which signatures it accepts, the hash functions used by the SDK, and how to work with keys from the command line with `subkey`. For background on the cryptography itself and why these schemes were chosen, see the [Cryptography](https://wiki.polkadot.com/learn/learn-cryptography/) page on the Polkadot Wiki.

## Signature Schemes

`sp_core` provides the following signature schemes. Each module defines a `Pair`, a `Public` key, and a `Signature` type:

- **[`sp_core::sr25519`](https://paritytech.github.io/polkadot-sdk/master/sp_core/sr25519/index.html)**: Schnorr signatures on the Ristretto group (Schnorrkel), using the same underlying curve as Ed25519. It supports both hard and soft key derivation. The parachain template's Aura keys use sr25519 (`sp_consensus_aura::sr25519::AuthorityId`).
- **[`sp_core::ed25519`](https://paritytech.github.io/polkadot-sdk/master/sp_core/ed25519/index.html)**: EdDSA signatures on Curve25519. It supports hard derivation only. GRANDPA keys use ed25519.
- **[`sp_core::ecdsa`](https://paritytech.github.io/polkadot-sdk/master/sp_core/ecdsa/index.html)**: ECDSA signatures on the secp256k1 curve, the same curve used by Bitcoin and Ethereum. It supports hard derivation only.

All three implement `Pair`, which provides methods such as `generate`, `from_phrase`, `from_seed`, and `from_string` to create a key pair, `public` to get the public key, `sign` to sign a message, and `verify` to check a signature. For example:

```rust
use sp_core::{sr25519, Pair};

// Create a key pair from a secret URI with a derivation path.
let pair = sr25519::Pair::from_string("//Alice", None).expect("valid secret URI");
let message = b"hello";
let signature = pair.sign(message);
assert!(sr25519::Pair::verify(&signature, message, &pair.public()));
```

`sp_core` also contains experimental `bls` and `bandersnatch` schemes, enabled with the `bls-experimental` and `bandersnatch-experimental` features.

## Signatures in the Runtime

A runtime chooses which signatures it accepts on transactions through its `Signature` type, which must implement [`sp_runtime::traits::Verify`](https://paritytech.github.io/polkadot-sdk/master/sp_runtime/traits/trait.Verify.html). The account ID type is then derived from the signer. The parachain template uses [`MultiSignature`](https://paritytech.github.io/polkadot-sdk/master/sp_runtime/enum.MultiSignature.html), which accepts signatures from any of the three schemes:

```rust
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
```

`MultiSignature` has `Ed25519`, `Sr25519`, `Ecdsa`, and `Eth` variants, and the matching [`MultiSigner`](https://paritytech.github.io/polkadot-sdk/master/sp_runtime/enum.MultiSigner.html) maps each public key to a 32-byte account ID. For sr25519 and ed25519, the account ID is the public key. For ECDSA, it is the Blake2 hash of the compressed public key, and the `Eth` variant derives the account ID from the Ethereum address instead. See [Accounts](/polkadot-sdk/reference/accounts.md) for how account IDs are encoded as SS58 addresses.

## Key Types

Nodes keep keys in a keystore, and each key is tagged with a four-byte [`KeyTypeId`](https://paritytech.github.io/polkadot-sdk/master/sp_core/crypto/struct.KeyTypeId.html) that says what the key is used for. The built-in identifiers in [`sp_core::crypto::key_types`](https://paritytech.github.io/polkadot-sdk/master/sp_core/crypto/key_types/index.html) include the following:

| Constant              | Identifier | Used for                       |
|-----------------------|------------|--------------------------------|
| `AURA`                | `aura`     | Aura block authoring           |
| `BABE`                | `babe`     | BABE block authoring           |
| `GRANDPA`             | `gran`     | GRANDPA finality voting        |
| `BEEFY`               | `beef`     | BEEFY                          |
| `IM_ONLINE`           | `imon`     | ImOnline heartbeats            |
| `AUTHORITY_DISCOVERY` | `audi`     | Authority discovery            |
| `ACCOUNT`             | `acco`     | Controlling a runtime account  |
| `STAKING`             | `stak`     | Staking                        |
| `STATEMENT`           | `stmt`     | Signing statements             |
| `DUMMY`               | `dumy`     | Tests                          |

A runtime declares which keys its session consists of with the [`impl_opaque_keys!`](https://paritytech.github.io/polkadot-sdk/master/sp_runtime/macro.impl_opaque_keys.html) macro. The parachain template has a single Aura key:

```rust
impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}
```

Pallets that define their own keys use [`sp_application_crypto`](https://paritytech.github.io/polkadot-sdk/master/sp_application_crypto/index.html) to wrap one of the `sp_core` schemes in an application-specific type bound to a `KeyTypeId`, so that keys for different purposes cannot be confused. For how collators generate and register their session keys, see [Run a Collator](/nodes/run-a-collator.md).

## Hash Functions

The SDK uses the Blake2 family as its default hash. The parachain template uses `BlakeTwo256` (Blake2b with a 256-bit output) as the block and header hashing algorithm, and the SS58 address checksum is also derived with Blake2b.

The [`sp_crypto_hashing`](https://paritytech.github.io/polkadot-sdk/master/sp_crypto_hashing/index.html) crate provides the hash functions used across the SDK, including `blake2_128`, `blake2_256`, `twox_64`, `twox_128`, `twox_256`, `sha2_256`, and `keccak_256`.

FRAME storage maps choose a hasher for their keys, such as [`Blake2_128Concat`](https://paritytech.github.io/polkadot-sdk/master/frame_support/struct.Blake2_128Concat.html) or [`Twox64Concat`](https://paritytech.github.io/polkadot-sdk/master/frame_support/struct.Twox64Concat.html). The FRAME documentation classifies `Blake2` hashers as cryptographically secure and `Twox` hashers as faster but not cryptographically secure. Use a cryptographically secure hasher when end users control the map keys. When they do not, or when the keys are cryptographically secure by definition (for example, an `AccountId`), a `Twox` hasher is sufficient.

## Work with Keys Using Subkey

[Subkey](https://paritytech.github.io/polkadot-sdk/master/subkey/index.html) is the Polkadot SDK command-line tool for generating and inspecting keys. Most commands accept a `--scheme` option (`sr25519`, `ed25519`, or `ecdsa`; the default is `sr25519`). Run `subkey --help` for the full list of commands and options.

- Generate a new key pair:

    ```bash
    subkey generate --scheme sr25519
    ```

- Inspect a secret phrase, seed, or SS58 address to see the public key and account ID:

    ```bash
    subkey inspect "INSERT_SECRET_PHRASE"
    ```

- Sign a message with a secret URI:

    ```bash
    echo -n "INSERT_MESSAGE" | subkey sign --suri "INSERT_SECRET_URI"
    ```

- Verify a signature against an address:

    ```bash
    echo -n "INSERT_MESSAGE" | subkey verify INSERT_SIGNATURE INSERT_ADDRESS
    ```

See [Validating Addresses](/polkadot-sdk/reference/accounts.md#validating-addresses) for example `subkey inspect` output.
