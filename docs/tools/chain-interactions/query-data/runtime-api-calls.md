---
title: Runtime API Calls
description: Learn how to call Polkadot SDK runtime APIs with the Subxt Rust library to execute logic in the Wasm runtime and retrieve computed results.
short_description: Execute runtime APIs directly for specialized queries and operations.
categories: Protocol Internals, Tooling
---

# Runtime API Calls

## Introduction

Polkadot SDK runtime APIs provide direct access to the blockchain's WebAssembly (Wasm) runtime, enabling specialized queries and computations that go beyond simple storage reads. Unlike storage queries that fetch static data, runtime APIs execute logic within the runtime to compute results dynamically.

Common runtime APIs include:

- **`AccountNonceApi`**: Retrieves the current transaction nonce for an account.
- **`Metadata`**: Queries available metadata versions and runtime information.
- **`TransactionPaymentApi`**: Estimates transaction fees before submission.

Runtimes expose additional APIs for the pallets they include. For example, a runtime with staking pallets exposes staking-related APIs.

## Call Runtime APIs with Subxt

This section uses [Subxt](/tools/subxt.md), Parity's Rust library for Polkadot SDK-based chains, to call runtime APIs on a local development node. The example queries an account nonce and the supported metadata versions.

The same runtime APIs can be called from third-party client libraries, for example [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/), [Dedot](https://developers.polkadot.com/reference/tools/dedot/), [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/), and [Python Substrate Interface](https://developers.polkadot.com/reference/tools/py-substrate-interface/).

### Prerequisites

- [Rust](https://rustup.rs/) toolchain (latest stable) and Cargo.
- A local development node listening on `{{ networks.local.ws_url }}`, for example the parachain template node described in [Run the Node Locally](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md#run-the-node-locally).

### Set Up the Project

1. Create a new Rust project:

    ```bash
    cargo new subxt-runtime-api-example && cd subxt-runtime-api-example
    ```

2. Install the Subxt CLI:

    ```bash
    cargo install subxt-cli@{{ dependencies.crates.subxt_cli.version }}
    ```

3. Download the metadata from your local node:

    ```bash
    subxt metadata --url {{ networks.local.ws_url }} -o metadata.scale
    ```

    To target a different chain, pass its WebSocket endpoint to `--url` and update the `NODE_URL` constant in the example to match.

4. Update `Cargo.toml` with the required dependencies:

    ```toml title="Cargo.toml"
    --8<-- "code/chain-interactions/query-data/runtime-api-calls/subxt/Cargo.toml"
    ```

### Call the APIs

The following example demonstrates calling two runtime APIs:

- **`AccountNonceApi.account_nonce`**: Gets the current nonce using the static interface generated from metadata.
- **`Metadata.metadata_versions`**: Retrieves supported metadata versions using the dynamic interface.

Create a file at `src/bin/runtime_apis.rs` and add the following code:

```rust title="src/bin/runtime_apis.rs"
--8<-- "code/chain-interactions/query-data/runtime-api-calls/subxt/src/bin/runtime_apis.rs"
```

By default, the example queries Alice's development account. Replace the `ADDRESS` value to query a different account. Then run the script:

```bash
cargo run --bin runtime_apis
```

The script prints the account nonce and the list of metadata versions that the runtime supports.

## Available Runtime APIs

The following runtime APIs are commonly available on Polkadot SDK-based chains:

| API | Description |
|-----|-------------|
| `AccountNonceApi` | Get current transaction nonce for an account |
| `TransactionPaymentApi` | Query transaction fees and weight information |
| `TransactionPaymentCallApi` | Estimate fees for a call without creating an extrinsic |
| `Metadata` | Query metadata versions and runtime metadata |
| `BlockBuilder` | Access block building functionality |
| `Core` | Core runtime version and execution |
| `TaggedTransactionQueue` | Validate transactions in the pool |
| `OffchainWorkerApi` | Offchain worker functionality |
| `SessionKeys` | Session key management |
| `GrandpaApi` | GRANDPA finality information |
| `BabeApi` | BABE consensus information |

!!! note
    Available runtime APIs vary by chain. Check your target chain's metadata to see which APIs are exposed.

## Where to Go Next

Now that you understand how to call runtime APIs to execute logic within the runtime, explore these related topics:

<div class="grid cards" markdown>

-   __Query with SDKs__

    ---

    Use the Subxt Rust library to query storage.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-sdks.md)

-   __Query On-Chain State with Polkadot REST API__

    ---

    Learn how to query on-chain state using the Polkadot REST API.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-rest.md)

-   __Send Transactions__

    ---

    Learn to construct and submit transactions.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/with-sdks.md)

</div>
