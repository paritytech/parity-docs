---
title: Send Transactions with SDKs
description: Learn how to construct, sign, and submit transactions to any Polkadot SDK-based chain using the Subxt Rust library and a local development node.
short_description: Build, sign, and submit transactions with Subxt.
categories: Chain Interactions, Tooling
page_badges:
  tutorial_badge: Intermediate
---

# Send Transactions with SDKs

## Introduction

Sending transactions on Polkadot SDK-based blockchains involves constructing an extrinsic (transaction), signing it with your account's private key, and submitting it to the network.

This guide uses [Subxt](/tools/subxt/), Parity's Rust library for Polkadot SDK-based chains, to send a balance transfer on a local development node. The example signs with the `//Alice` development account and sends funds to `//Bob`, so you do not need to fund an account first.

The same transaction flow works with third-party client libraries, for example [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/), [Dedot](https://developers.polkadot.com/reference/tools/dedot/), [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/), and [Python Substrate Interface](https://developers.polkadot.com/reference/tools/py-substrate-interface/).

## Prerequisites

- [Rust](https://rustup.rs/) toolchain (latest stable) and Cargo.
- A local development node listening on `{{ networks.local.ws_url }}`, for example the parachain template node described in [Run the Node Locally](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template/#run-the-node-locally). The template's `development` preset funds the well-known development accounts, such as Alice and Bob, as defined in its [genesis config presets](https://github.com/paritytech/polkadot-sdk-parachain-template/blob/v0.0.5/runtime/src/genesis_config_presets.rs).

## Set Up the Project

1. Create a new Rust project:

    ```bash
    cargo new subxt-send-tx-example && cd subxt-send-tx-example
    ```

2. Install the Subxt CLI:

    ```bash
    cargo install subxt-cli@{{ dependencies.crates.subxt_cli.version }}
    ```

3. Download the metadata from your local node:

    ```bash
    subxt metadata --url {{ networks.local.ws_url }} -o metadata.scale
    ```

4. Update `Cargo.toml` with the required dependencies:

    ```toml title="Cargo.toml"
    --8<-- "code/chain-interactions/send-transactions/with-sdks/subxt/Cargo.toml"
    ```

## Send a Balance Transfer

The following example constructs, signs, and submits a `Balances.transfer_keep_alive` transaction, then waits for it to be finalized.

Create a file at `src/bin/send_transfer.rs` and add the following code to it:

```rust title="src/bin/send_transfer.rs"
--8<-- "code/chain-interactions/send-transactions/with-sdks/subxt/src/bin/send_transfer.rs"
```

Key aspects of the code:

- **Development signer**: [`subxt_signer::sr25519::dev::alice()`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/sr25519/dev/index.html) returns the `//Alice` development keypair. To sign with your own account on another chain, load it with `Keypair::from_phrase` instead.
- **Transaction payload**: `runtime::transactions().balances().transfer_keep_alive()` builds a type-checked call from the generated interface.
- **Submission**: `sign_and_submit_then_watch_default` signs the transaction with default parameters, submits it, and returns a progress handle. `wait_for_finalized_success` waits for finalization and returns the transaction's events.

!!! warning
    Development accounts use publicly known keys. Use them only on local development chains. Never share your mnemonic phrase or private keys, and use secure key management in production.

Run the script:

```bash
cargo run --bin send_transfer
```

The script prints the sender and recipient addresses, the sender's free balance, and the `Balances.Transfer` event once the transaction is finalized.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge guide">Guide</span> **Query On-Chain State**

    ---

    Learn how to query storage and runtime data with Subxt.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-sdks/)

- <span class="badge guide">Guide</span> **Calculate Transaction Fees**

    ---

    Estimate fees before sending transactions.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/calculate-transaction-fees/)

</div>
