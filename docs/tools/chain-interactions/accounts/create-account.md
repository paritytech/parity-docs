---
title: Create an Account
description: Step-by-step guide to generating accounts for Polkadot SDK-based chains in Rust with sp-core, including a mnemonic and an SS58 address for a configurable network prefix.
short_description: Generate an account and SS58 address in Rust.
categories: Basics, Chain Interactions
page_badges:
  tutorial_badge: Beginner
---

# Create an Account

## Introduction

Creating accounts is a fundamental operation when building applications on Polkadot SDK-based chains. Accounts serve as the basis for identity, asset ownership, and transaction signing. Generating and managing accounts programmatically enables you to build wallets, automate operations, and create smoother user experiences.

Polkadot SDK-based chains use the SR25519 signature scheme by default, though ED25519 and ECDSA are also supported. Each account consists of a public key (address) and a private key (seed or mnemonic). Keep your private keys secure and never share them.

This tutorial generates an account in Rust with [`sp-core`](https://docs.rs/sp-core/latest/sp_core/), the Polkadot SDK crate that provides the core cryptographic primitives. To sign transactions with the generated mnemonic, use [`subxt-signer`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/), as shown in [Send Transactions with SDKs](/tools/chain-interactions/send-transactions/with-sdks.md).

Account generation is also available in third-party libraries, for example [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/) and [Python Substrate Interface](https://developers.polkadot.com/reference/tools/py-substrate-interface/).

## Prerequisites

Before starting, make sure you have:

- A basic understanding of public-key cryptography concepts.
- [Rust](https://rustup.rs/) toolchain (latest stable) and Cargo.

## Generate an Account in Rust

1. Create a new Rust project:

    ```bash
    cargo new account-creator-rust
    cd account-creator-rust
    ```

2. Add `sp-core` as a dependency:

    ```bash
    cargo add sp-core
    ```

3. Create your account generation code in `src/main.rs`:

    ```rust title="src/main.rs"
    --8<-- 'code/chain-interactions/accounts/create-account/create-account.rs'
    ```

    Key aspects of the code:

    - **Keypair generation**: [`sr25519::Pair::generate_with_phrase()`](https://docs.rs/sp-core/latest/sp_core/crypto/trait.Pair.html#method.generate_with_phrase) creates a new key pair with a mnemonic.
    - **Public key extraction**: The [`public()`](https://docs.rs/sp-core/latest/sp_core/crypto/trait.Pair.html#tymethod.public) method retrieves the public key from the pair.
    - **SS58 encoding**: [`to_ss58check_with_version()`](https://docs.rs/sp-core/latest/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check_with_version) encodes the public key as a human-readable address using the prefix in `SS58_PREFIX`. Set `SS58_PREFIX` to the prefix of your target chain. The example uses `42`, the generic Substrate prefix.

4. Build and run the project:

    ```bash
    cargo run
    ```

    You should see output similar to:

    --8<-- 'code/chain-interactions/accounts/create-account/create-account-rs.html'

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Send Transactions with SDKs__

    ---

    Learn how to send signed transactions using Subxt.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/with-sdks.md)

-   <span class="badge guide">Guide</span> __Calculate Transaction Fees__

    ---

    Learn how to estimate transaction fees before sending transactions from your accounts.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/calculate-transaction-fees.md)

-   <span class="badge guide">Guide</span> __Query Chain Data__

    ---

    Query account information, balances, and other chain state.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-sdks.md)

</div>
