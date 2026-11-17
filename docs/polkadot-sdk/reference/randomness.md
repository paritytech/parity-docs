---
title: Randomness
description: Learn how runtimes built with the Polkadot SDK get on-chain randomness through the Randomness trait, which sources pallet-babe provides, and which sources are not secure.
categories: Basics, Protocol Internals
---

# Randomness

## Introduction

A blockchain runtime is deterministic: every node must compute the same result from the same inputs, so a runtime cannot read real-world entropy. On-chain randomness is instead derived from data that all nodes agree on, such as verifiable random function (VRF) outputs included in blocks or past block hashes. Such values are hard to predict far in advance but become predictable, and sometimes biasable, close to the time they are used.

The Polkadot SDK exposes on-chain randomness to pallets through the [`Randomness`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/trait.Randomness.html) trait. This page describes the trait, the implementations the SDK provides, and the limits you should design around. For the research behind the relay chain's randomness, see the [Randomness](https://wiki.polkadot.com/learn/learn-cryptography/#randomness) section of the Polkadot Wiki.

## The `Randomness` Trait

The `frame_support::traits::Randomness<Output, BlockNumber>` trait has two functions:

- **`random(subject: &[u8]) -> (Output, BlockNumber)`**: Returns the most recently determined random seed for the given `subject`, together with the block number since when the seed was determinable by chain observers. The `subject` is a context identifier that gives each caller an independent result, for example `random(&b"my context"[..])`.
- **`random_seed() -> (Output, BlockNumber)`**: Returns the basic random seed, equivalent to calling `random` with an empty subject. Prefer `random` with a subject.

The returned block number matters. The trait documentation states that the seed should only be used to distinguish commitments made before that block. If commitments were made afterwards, stop accepting new commitments and call `random` again on later blocks until the returned block number is later than the latest commitment.

A pallet that needs randomness declares an associated type bound by the trait, and the runtime chooses the implementation:

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type MyRandomness: frame_support::traits::Randomness<Self::Hash, BlockNumberFor<Self>>;
}
```

## Randomness from BABE

Chains that use BABE for block production can use the randomness implementations in [`pallet_babe::randomness`](https://paritytech.github.io/polkadot-sdk/master/pallet_babe/randomness/struct.RandomnessFromOneEpochAgo.html). They are derived from the VRF outputs that block authors include in their blocks, accumulated per epoch:

- **[`RandomnessFromTwoEpochsAgo`](https://paritytech.github.io/polkadot-sdk/master/pallet_babe/randomness/struct.RandomnessFromTwoEpochsAgo.html)**: For consensus protocols that depend on finality and act on commitments made during the epoch before the previous epoch. Input commitments should come from at least three epochs ago.
- **[`RandomnessFromOneEpochAgo`](https://paritytech.github.io/polkadot-sdk/master/pallet_babe/randomness/struct.RandomnessFromOneEpochAgo.html)**: For on-chain code that does not depend on finality and acts on commitments made during the previous epoch. Input commitments should come from at least two epochs ago.
- **[`ParentBlockRandomness`](https://paritytech.github.io/polkadot-sdk/master/pallet_babe/randomness/struct.ParentBlockRandomness.html)**: Produced with each block, but derived from `RandomnessFromTwoEpochsAgo` and weaker than both other sources, because a block author can withhold a block whose randomness is unfavorable. When used from within the runtime, it is generated from the parent block. It returns an `Option<Hash>`.

`CurrentBlockRandomness` is deprecated. It does not provide fresh randomness for the current block; use `ParentBlockRandomness` instead.

For every BABE source, adversaries that control enough block production slots near epoch boundaries can bias the result, for example by choosing what to build or skipping slots. Treat these sources as suitable for low-influence decisions and design your pallet so that a biased value cannot cause significant loss.

## Randomness on Parachains

A parachain built from the parachain template uses Aura, not BABE, so `pallet_babe` randomness is not available in its runtime. The relay chain publishes its BABE randomness under well-known storage keys, such as [`ONE_EPOCH_AGO_RANDOMNESS`](https://paritytech.github.io/polkadot-sdk/master/polkadot_primitives/v9/well_known_keys/constant.ONE_EPOCH_AGO_RANDOMNESS.html) and `TWO_EPOCHS_AGO_RANDOMNESS`. A parachain runtime can ask the collator to include relay chain storage keys in the relay chain state proof it passes to the runtime by implementing the [`KeyToIncludeInRelayProof`](https://paritytech.github.io/polkadot-sdk/master/cumulus_primitives_core/trait.KeyToIncludeInRelayProof.html) runtime API. The same bias limits apply to relay chain randomness read this way.

## Insecure Randomness for Testing

The [`pallet_insecure_randomness_collective_flip`](https://paritytech.github.io/polkadot-sdk/master/pallet_insecure_randomness_collective_flip/index.html) pallet implements `Randomness` using the block hashes of the previous 81 blocks. It is useful for tests and local development chains that have no VRF-based source.

!!! warning
    Do not use `pallet_insecure_randomness_collective_flip` in production. Its documentation states that the values it produces do not fulfill the cryptographic requirements for random numbers. The values are only low-influence, which defends against relatively weak adversaries.

## Additional Resources

- [Randomness](https://wiki.polkadot.com/learn/learn-cryptography/#randomness) on the Polkadot Wiki
- [`frame_support::traits::Randomness`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/trait.Randomness.html) in the Polkadot SDK Rust docs
- [Blind Assignment of Blockchain Extension (BABE)](/get-started/glossary.md#blind-assignment-of-blockchain-extension-babe) in the glossary
