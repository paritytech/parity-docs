---
title: Calculate Transaction Fees
description: Learn how transaction fees are calculated on Polkadot SDK-based chains and how to estimate the fee for a transaction with the Subxt Rust library.
short_description: Calculate transaction fees to ensure sufficient balance and optimize costs.
categories: Chain Interactions, Tooling
page_badges:
  tutorial_badge: Beginner
---

# Calculate Transaction Fees

## Introduction

Transaction fees are the cost of executing operations on Polkadot SDK-based chains. Estimating these fees before you submit a transaction helps you manage account balances and build better user experiences in your applications.

This tutorial explains the fee model that FRAME runtimes use and shows how to estimate a transaction's fee with [Subxt](/tools/subxt.md), Parity's Rust library for Polkadot SDK-based chains.

## Fee Model

Runtimes that use the [Transaction Payment pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_transaction_payment/index.html) calculate fees from the following components:

- **Base fee**: The minimum amount a user pays for a transaction, declared as a base weight in the runtime and converted to a fee.
- **Length fee**: A fee proportional to the encoded length of the transaction.
- **Weight fee**: A fee proportional to the amount of weight (computational resources) the transaction consumes.
- **Tip**: An optional amount that increases the priority of the transaction in the transaction queue.

The pallet combines these components as follows:

```text
inclusion_fee = base_fee + length_fee + [targeted_fee_adjustment * weight_fee]
final_fee = inclusion_fee + tip
```

The `targeted_fee_adjustment` multiplier tunes the fee based on network congestion. Each runtime configures how weight maps to fees and how the multiplier is updated, so the same call can cost different amounts on different chains.

The runtime exposes fee estimates through the `TransactionPaymentApi` runtime API. Client libraries call this API to estimate the fee of a signed transaction before submitting it. For more about runtime APIs, see [Runtime API Calls](/tools/chain-interactions/query-data/runtime-api-calls.md).

## Estimate Fees with Subxt

### Prerequisites

- [Rust](https://rustup.rs/) toolchain (latest stable) and Cargo.
- A local development node listening on `{{ networks.local.ws_url }}`, for example the parachain template node described in [Run the Node Locally](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md#run-the-node-locally).

### Set Up the Project

1. Create a new Rust project:

    ```bash
    cargo new subxt-fee-estimate-example && cd subxt-fee-estimate-example
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
    --8<-- "code/chain-interactions/send-transactions/calculate-transaction-fees/subxt/Cargo.toml"
    ```

### Estimate the Fee

The following example builds a balance transfer from the `//Alice` development account to `//Bob`, signs it without submitting it, and asks the node for a fee estimate.

Create a file at `src/bin/estimate_fee.rs` and add the following code to it:

```rust title="src/bin/estimate_fee.rs"
--8<-- "code/chain-interactions/send-transactions/calculate-transaction-fees/subxt/src/bin/estimate_fee.rs"
```

Key aspects of the code:

- **Transaction creation**: `runtime::transactions().balances().transfer_keep_alive()` builds the balance transfer call from the generated interface.
- **`create_signed`**: Signs the transaction with the given transaction parameters and returns it without submitting it.
- **[`partial_fee_estimate`](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/transactions/struct.SubmittableTransaction.html#method.partial_fee_estimate)**: Returns an estimate of what the transaction is expected to cost, excluding any tip, based on the block at which you submit it. The actual amount can vary from block to block.

Run the script:

```bash
cargo run --bin estimate_fee
```

The script prints the estimated fee in the chain's smallest unit.

## Other Tools

Fee estimation is also available in third-party client libraries, for example [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/) and [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/). Interfaces such as Polkadot.js Apps also display the estimated fee before you sign a transaction.

## Where to Go Next

Now that you can calculate transaction fees, explore related guides to send transactions and manage fees in your applications.

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Pay Fees with Different Tokens__

    ---

    Learn how to send transactions while paying fees using alternative tokens instead of the native chain token.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/pay-fees-with-different-tokens.md)

-   <span class="badge guide">Guide</span> __Send Transactions with SDKs__

    ---

    Learn how to send signed transactions using Subxt.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/with-sdks.md)

-   <span class="badge guide">Guide</span> __Query Chain Data__

    ---

    Explore different methods for querying blockchain data using REST APIs, SDKs, and runtime API calls.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-sdks.md)

</div>
