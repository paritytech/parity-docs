---
title: Query On-Chain State with SDKs
description: Learn how to query account information, balances, and asset data from any Polkadot SDK-based chain using the Subxt Rust library.
short_description: Programmatically read account information and other chain state with Subxt.
categories: Chain Interactions, Tooling
page_badges:
  tutorial_badge: Intermediate
---

# Query On-Chain State with SDKs

## Introduction

Polkadot SDK-based blockchains store data in a key-value database that external applications can query. This on-chain state includes account information, balances, asset information, and any other data the runtime manages.

This guide uses [Subxt](/tools/subxt.md), Parity's Rust library for Polkadot SDK-based chains, to query a local development node for account information and asset data. Subxt generates type-safe interfaces from chain metadata, so storage queries are checked at compile time. To query a chain without generated code, use Subxt's dynamic API instead; see [Static and Dynamic APIs](/tools/subxt.md#static-and-dynamic-apis).

The same storage queries work with third-party client libraries, for example [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/), [Dedot](https://developers.polkadot.com/reference/tools/dedot/), [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/), and [Python Substrate Interface](https://developers.polkadot.com/reference/tools/py-substrate-interface/).

## Prerequisites

- [Rust](https://rustup.rs/) toolchain (latest stable) and Cargo.
- A local development node listening on `{{ networks.local.ws_url }}`, for example the parachain template node described in [Run the Node Locally](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md#run-the-node-locally).

## Set Up the Project

1. Create a new Rust project:

    ```bash
    cargo new subxt-query-example && cd subxt-query-example
    ```

2. Install the Subxt CLI:

    ```bash
    cargo install subxt-cli@{{ dependencies.crates.subxt_cli.version }}
    ```

3. Download the metadata from your local node:

    ```bash
    subxt metadata --url {{ networks.local.ws_url }} -o metadata.scale
    ```

    To query a different chain, pass its WebSocket endpoint to `--url` and update the `NODE_URL` constant in the examples to match.

4. Update `Cargo.toml` with the required dependencies:

    ```toml title="Cargo.toml"
    --8<-- "code/chain-interactions/query-data/query-sdks/subxt/Cargo.toml"
    ```

## Query Account Information

The following example queries the `System.Account` storage item to retrieve an account's nonce, reference counters, and native token balances. By default, it queries Alice's development account.

Create a file at `src/bin/query_balance.rs` and add the following code to it:

```rust title="src/bin/query_balance.rs"
--8<-- "code/chain-interactions/query-data/query-sdks/subxt/src/bin/query_balance.rs"
```

Replace the `ADDRESS` value with any SS58 address you want to query. Then run the script:

```bash
cargo run --bin query_balance
```

The script prints the account information and balance details described in the next section. To read every account instead of a single one, iterate over the storage map as described in [Iterate over Storage](/tools/subxt.md#iterate-over-storage).

### Account Data Fields

When you query `System.Account`, the result contains the following fields:

- **Nonce**: The number of transactions sent from this account, used to prevent replay attacks.
- **Consumers**: The number of modules depending on this account's existence.
- **Providers**: The number of modules providing for this account's existence.
- **Sufficients**: The number of modules that allow this account to exist on its own.
- **Free balance**: The transferable balance available for transactions.
- **Reserved balance**: Balance that is locked for specific purposes, such as deposits held by pallets.
- **Frozen balance**: Balance that cannot be used for transfers but may be used for other operations.

The total balance is the sum of the free and reserved balances. Balances are returned in the chain's smallest unit.

## Query Asset Information

The following example queries the `Assets` pallet to retrieve metadata, details, and an account's balance for a single asset. It requires a runtime that includes the [Assets pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_assets/index.html) and an asset that already exists on your chain. To add the pallet to your own runtime, see [Add Existing Pallets to the Runtime](/polkadot-sdk/customize-runtime/add-existing-pallets.md).

Create a file at `src/bin/query_asset.rs` and add the following code to it:

```rust title="src/bin/query_asset.rs"
--8<-- "code/chain-interactions/query-data/query-sdks/subxt/src/bin/query_asset.rs"
```

Replace `INSERT_ASSET_ID` with the ID of an asset on your chain and `INSERT_ADDRESS` with the account address you want to query. Then run the script:

```bash
cargo run --bin query_asset
```

The script prints the asset's name, symbol, and decimals, followed by its owner, supply, number of holders, and minimum balance, and the queried account's balance of the asset.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge guide">Guide</span> **Runtime API Calls**

    ---

    Execute runtime APIs for specialized queries.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/runtime-api-calls.md)

- <span class="badge guide">Guide</span> **Send Transactions**

    ---

    Learn how to construct and submit transactions.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/with-sdks.md)

</div>
