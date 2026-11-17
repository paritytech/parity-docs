---
title: Subxt Rust API
description: Subxt is a Rust library for type-safe interaction with Polkadot SDK blockchains, enabling transactions, state queries, runtime API access, and more.
categories: Tooling
---

# Subxt Rust API

## Introduction

[Subxt](https://github.com/paritytech/subxt) is a Rust library for interacting with Polkadot SDK-based chains. It provides a type-safe interface for submitting transactions, querying on-chain state, and performing other chain interactions. Subxt generates Rust types from a chain's metadata, so the compiler checks your calls against the runtime you target.

The examples on this page connect to a local development node at `{{ networks.local.ws_url }}` and use the `//Alice` and `//Bob` development accounts.

## Prerequisites

Before using Subxt, ensure you have the following:

- Rust and Cargo installed on your system. You can install them using [Rustup](https://rustup.rs/).
- A local development node listening on `{{ networks.local.ws_url }}`, for example one started with [Polkadot Omni Node](omninode.md) or [Zombienet](zombienet.md).
- A Rust project. If you don't have one, create it with:
    ```bash
    cargo new my_project && cd my_project
    ```

## Installation

To use Subxt in your project, you must install the necessary dependencies. Each plays a specific role in enabling interaction with the blockchain:

1. **Install the Subxt CLI**: [`subxt-cli`](https://crates.io/crates/subxt-cli) is a command-line tool that provides utilities for working with Polkadot SDK metadata. In the context of Subxt, it is essential to download chain metadata, which is required to generate type-safe Rust interfaces for interacting with the blockchain. Install it using the following:

    ```bash
    cargo install subxt-cli@{{dependencies.crates.subxt_cli.version}}
    ```

2. **Add core dependencies**: These dependencies are essential for interacting with the blockchain.

    - **[subxt](https://crates.io/crates/subxt)**: The main library for communicating with Polkadot SDK nodes. It handles RPC requests, encoding/decoding, and type generation.

        ```bash
        cargo add subxt@{{dependencies.crates.subxt.version}}
        ```

    - **[subxt-signer](https://crates.io/crates/subxt-signer)**: Provides cryptographic functionality for signing transactions. Without this, you can only read data but cannot submit transactions.

        ```bash
        cargo add subxt-signer@{{dependencies.crates.subxt_signer.version}}
        ```

    - **[tokio](https://crates.io/crates/tokio)**: An asynchronous runtime for Rust. Since blockchain operations are async, Tokio enables the efficient handling of network requests. The `rt` feature enables Tokio's runtime, including the current-thread single-threaded scheduler, which is necessary for async execution. The `macros` feature provides procedural macros like `#[tokio::main]` to simplify runtime setup.

        ```bash
        cargo add tokio@{{dependencies.crates.tokio.version}} --features rt,macros
        ```

    After adding the dependencies, your `Cargo.toml` should look like this:

    ```toml
    --8<-- 'code/reference/tools/subxt/Cargo.toml'
    ```

## Get Started

This section walks through the fundamental operations of Subxt, from downloading metadata to querying state and submitting transactions.

### Download Chain Metadata

Before interacting with a blockchain, you need to retrieve its metadata. This metadata defines storage structures, extrinsics, and other runtime details. Use the `subxt-cli` tool to download the metadata from your local node:

```bash
subxt metadata --url {{ networks.local.ws_url }} > polkadot_metadata.scale
```

To target a different chain, replace the URL with that chain's RPC endpoint.

### Generate Type-Safe Interfaces

Use the `#[subxt::subxt]` macro to generate a type-safe Rust interface from the downloaded metadata:

```rust
--8<-- 'code/reference/tools/subxt/subxt.rs:6:8'
```

Once Subxt interfaces are generated, you can interact with your node in the following ways. The links point to the related Subxt documentation:

- **[Transactions](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/transactions/index.html)**: Builds and submits transactions, monitors their inclusion in blocks, and retrieves associated events.
- **[Storage](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/storage/index.html)**: Enables querying of node storage data.
- **[Events](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/events/index.html)**: Retrieves events emitted from recent blocks.
- **[Constants](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/constants/index.html)**: Accesses constant values stored in nodes that remain unchanged across a specific runtime version.
- **[Extrinsics](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/extrinsics/index.html)**: Loads recent blocks or subscribes to new/finalized blocks, allowing examination of extrinsics, events, and storage at those blocks.
- **[Runtime APIs](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/runtime_apis/index.html)**: Makes calls into pallet runtime APIs to fetch data.
- **[Custom values](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/custom_values/index.html)**: Accesses "custom values" contained within metadata.
- **[Raw RPC calls](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/index.html#re-exports)**: Facilitates raw RPC requests to compatible nodes.

### Initialize the Subxt Client

To interact with a node using Subxt, create an asynchronous main function and initialize the client. The example connects to the local node; change `NODE_URL` to target another chain:

```rust
--8<-- 'code/reference/tools/subxt/subxt.rs::19'
    // Your code here...
--8<-- 'code/reference/tools/subxt/subxt.rs:74:75'
```

### Read Chain Data

Subxt provides multiple ways to access on-chain data:

- **Constants**: Constants are predefined values in the runtime that remain unchanged unless modified by a runtime upgrade.

    For example, to retrieve the existential deposit, use:
    
    ```rust
    --8<-- 'code/reference/tools/subxt/subxt.rs:21:25'
    ```

- **State**: State refers to the current chain data, which updates with each block.

    To fetch account information, replace `INSERT_ADDRESS` with the address you want to fetch data from and use:

    ```rust
    --8<-- 'code/reference/tools/subxt/subxt.rs:29:42'
    ```

### Submit Transactions

To submit a transaction, construct an extrinsic, sign it, and send it to the chain. The following example transfers funds from the `//Alice` development account to `//Bob` using the [`subxt_signer::sr25519::dev`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/sr25519/dev/index.html) helpers. Replace `INSERT_AMOUNT` with the amount to transfer:

```rust
--8<-- 'code/reference/tools/subxt/subxt.rs:46:72'
```

The development accounts' secret keys are public, so use them only on local development chains. For other chains, load a keypair from your own secret with `Keypair::from_phrase` or `Keypair::from_uri`.

## Light Client Support

Subxt can connect through an embedded [smoldot](light-clients.md) light client instead of an RPC node, using its `lightclient` module. Enable it with the `light-client` Cargo feature (not on by default). See the [`light_client.rs`](https://github.com/paritytech/subxt/blob/master/subxt/examples/light_client.rs) example in the Subxt repository.

## Where to Go Next

- [Subxt documentation](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/introduction/index.html): Reference material and advanced features.
- [Subxt examples](https://github.com/paritytech/subxt/tree/master/subxt/examples): Runnable examples for transactions, storage, blocks, runtime APIs, and light clients.
