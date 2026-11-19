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

## Static and Dynamic APIs

Subxt offers two ways to build queries, calls, and constant lookups:

- **Static**: The `#[subxt::subxt]` macro generates typed addresses and payloads, such as `polkadot::storage().system().account()`, from metadata at compile time. The compiler checks names, argument types, and return types, and values decode into generated Rust types.
- **Dynamic**: The [`subxt::dynamic`](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/dynamic/index.html) module builds the same addresses and payloads from pallet and item names at runtime. Use it when you do not have metadata at compile time, for example in a tool that works with any chain.

Dynamic addresses take the key and value types as generic parameters. You can use [`subxt::dynamic::Value`](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/dynamic/index.html) as a catch-all type, or any type that implements `DecodeAsType` to decode only the fields you need. The following example reads a constant and a storage entry, then submits a transfer, all without generated code:

```rust
use subxt::dynamic::{self, Value};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("{{ networks.local.ws_url }}").await?;
    let at_block = api.at_current_block().await?;

    // Constant: Balances.ExistentialDeposit, decoded as a u128.
    let deposit_addr = dynamic::constant::<u128>("Balances", "ExistentialDeposit");
    let deposit = at_block.constants().entry(deposit_addr)?;
    println!("Existential deposit: {deposit}");

    // Storage: System.Account, keyed by a 32-byte account ID.
    let account_addr = dynamic::storage::<([u8; 32],), Value>("System", "Account");
    let alice_id = dev::alice().public_key().0;
    let account = at_block
        .storage()
        .fetch(account_addr, (alice_id,))
        .await?
        .decode()?;
    println!("Alice's account: {account}");

    // Transaction: Balances.transfer_keep_alive from Alice to Bob.
    let tx = dynamic::transaction(
        "Balances",
        "transfer_keep_alive",
        (dev::bob().public_key().to_address::<()>(), 1_000_000_000_000u128),
    );
    at_block
        .transactions()
        .sign_and_submit_then_watch_default(&tx, &dev::alice())
        .await?
        .wait_for_finalized_success()
        .await?;

    Ok(())
}
```

The `subxt::dynamic` module also provides `runtime_api_call`, `view_function_call`, and `custom_value` for runtime APIs, view functions, and custom values. A dynamic call fails at runtime, not at compile time, if the pallet, item, or argument types do not match the chain's metadata. For a longer example that also calls a runtime API and decodes into custom structs, see [`dynamic.rs`](https://github.com/paritytech/subxt/blob/v{{ dependencies.crates.subxt.version }}/subxt/examples/dynamic.rs) in the Subxt repository.

## Configure Code Generation

You can generate the static interface with the `#[subxt::subxt]` macro or with the `subxt codegen` command. Both read the same metadata and accept the same options.

### Macro Attributes

The [`#[subxt::subxt]`](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/attr.subxt.html) macro needs exactly one metadata source:

- **`runtime_metadata_path = "..."`**: A SCALE-encoded metadata file, such as the one produced by `subxt metadata`. This is the recommended source.
- **`runtime_metadata_insecure_url = "..."`**: A node URL to download metadata from at compile time. It requires the `runtime-metadata-insecure-url` feature of the `subxt-macro` crate. The Subxt documentation advises against it outside CI, because builds depend on a reachable node and the metadata can change without notice.
- **`runtime_path = "..."`**: A runtime Wasm file. It requires the `runtime-wasm-path` feature of `subxt`.

The following attributes change the generated code:

- **`derive_for_all_types = "..."`**: Adds derives to every generated type, for example `"Eq, PartialEq"`. Substituted types must also implement these traits.
- **`derive_for_type(path = "...", derive = "...")`**: Adds derives to one generated type. Add `recursive` to apply them to the types it contains.
- **`attributes_for_all_types` and `attributes_for_type(path = "...", attributes = "...")`**: Add attributes to all generated types or to one type.
- **`substitute_type(path = "...", with = "...")`**: Replaces a generated type with your own type wherever it appears. The substitute must implement `EncodeAsType` and `DecodeAsType`, or be wrapped in `subxt::utils::Static`.
- **`crate = "..."`**: Sets the path to the `subxt` crate in the generated code, which is useful when you re-export Subxt from a library.
- **`generate_docs`**: Includes metadata documentation in the generated code.
- **`runtime_types_only`**: Generates only the runtime types, without the Subxt interface.
- **`no_default_derives` and `no_default_substitutions`**: Turn off the default derives or the default type substitutions.

Every `path` you pass must name a type in the metadata. Otherwise, the macro fails with an error such as ``Type `PalletId` does not exist at path `frame_support::PalletId` ``, so adjust the paths to the chain you target.

For example, to add derives to generated types and substitute the `Perbill` type with your own `crate::Perbill` type:

```rust
#[subxt::subxt(
    runtime_metadata_path = "./polkadot_metadata.scale",
    derive_for_all_types = "Eq, PartialEq",
    derive_for_type(path = "frame_support::PalletId", derive = "Ord, PartialOrd"),
    substitute_type(path = "sp_arithmetic::per_things::Perbill", with = "crate::Perbill"),
)]
pub mod polkadot {}
```

### Subxt CLI

`subxt-cli` provides the same metadata and code generation as commands. The commands accept `--url` to read from a node or `--file` to read a metadata file. If you pass neither, they connect to `ws://localhost:9944`. They reject unencrypted `ws://` and `http://` URLs unless the host is local or you pass `--allow-insecure`.

- **`subxt metadata`**: Downloads metadata. Use `--pallets` or `--runtime-apis` with comma-separated names to keep only part of it, `--format` to choose `bytes` (default), `hex`, or `json`, and `--output-file` to write to a file instead of standard output.

    ```bash
    subxt metadata --url {{ networks.local.ws_url }} --pallets System,Balances --output-file polkadot_metadata_small.scale
    ```

- **`subxt codegen`**: Prints the generated Rust interface instead of generating it in a macro, so you can commit and review it. It accepts `--derive`, `--derive-for-type`, `--attributes-for-type`, `--substitute-type`, `--crate`, `--no-docs`, `--runtime-types-only`, `--no-default-derives`, and `--no-default-substitutions`, which mirror the macro attributes.

    ```bash
    subxt codegen --file polkadot_metadata.scale | rustfmt --edition=2021 --emit=stdout > src/polkadot_interface.rs
    ```

Run `subxt metadata --help` or `subxt codegen --help` for all options.

## Sign Transactions with subxt-signer

The [`subxt-signer`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/index.html) crate provides keypairs that implement Subxt's `Signer` trait, so you can pass them to methods such as `sign_and_submit_then_watch_default`. Its default features enable the `sr25519` and `ecdsa` modules and the `subxt` integration.

Each of the [`sr25519::Keypair`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/sr25519/struct.Keypair.html) and [`ecdsa::Keypair`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/ecdsa/struct.Keypair.html) types can be created in the following ways:

- **Development accounts**: The `dev` module of each scheme returns the well-known development keypairs, `alice()`, `bob()`, `charlie()`, `dave()`, `eve()`, and `ferdie()`. They derive from the public development phrase, so use them only on local chains. Development chains usually fund only the sr25519 accounts.
- **Secret URI**: `Keypair::from_uri` takes a [`SecretUri`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/struct.SecretUri.html) of the form `phrase//hard/soft///password`. A URI without a phrase, such as `//Alice`, derives from the development phrase. A `0x`-prefixed hex string in place of the phrase is used as the seed.
- **Mnemonic**: `Keypair::from_phrase` takes a BIP-39 mnemonic, re-exported as `subxt_signer::bip39::Mnemonic`, and an optional password.
- **Secret key**: `Keypair::from_secret_key` takes 32 raw secret key bytes.

The following example creates keypairs in each of these ways. It reads the mnemonic from an environment variable so the secret does not appear in your source code:

```rust
use std::str::FromStr;
use subxt_signer::{bip39::Mnemonic, ecdsa, sr25519, SecretUri};

fn keypairs() -> Result<(), Box<dyn std::error::Error>> {
    // Development account.
    let alice = sr25519::dev::alice();

    // Secret URI: the development phrase with the hard derivation path //Alice.
    let uri = SecretUri::from_str("//Alice")?;
    let alice_from_uri = sr25519::Keypair::from_uri(&uri)?;
    assert_eq!(alice.public_key().0, alice_from_uri.public_key().0);

    // Mnemonic phrase, read from the MNEMONIC environment variable.
    let phrase = Mnemonic::parse(std::env::var("MNEMONIC")?)?;
    let account = sr25519::Keypair::from_phrase(&phrase, None)?;

    // An ecdsa keypair from the same secret URI.
    let alice_ecdsa = ecdsa::Keypair::from_uri(&uri)?;

    Ok(())
}
```

An ecdsa key produces a different account ID from an sr25519 key built from the same secret, because the account ID is a hash of the ecdsa public key. To load accounts exported from Polkadot.js as JSON, enable the `polkadot-js-compat` feature. Ethereum-style keys are available in the `eth` module behind the `unstable-eth` feature.

## Subscribe to Blocks and Events

`OnlineClient` provides three block subscriptions:

- **`stream_blocks`**: Finalized blocks. Use this one in most cases.
- **`stream_best_blocks`**: Blocks on the current best fork.
- **`stream_all_blocks`**: All imported blocks, including forks.

Blocks from `stream_best_blocks` and `stream_all_blocks` can be pruned at any time, so working with them later can fail. Each streamed block has a `number`, a `hash`, and an `at` method that returns a client for that block, with the same storage, constants, events, and extrinsics APIs as `at_current_block`.

The following example follows finalized blocks and prints every balance transfer event, using the `polkadot` module generated earlier on this page:

```rust
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "./polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("{{ networks.local.ws_url }}").await?;

    let mut blocks = api.stream_blocks().await?;
    while let Some(block) = blocks.next().await {
        let block = block?;
        println!("Block #{} ({})", block.number(), block.hash());

        let at_block = block.at().await?;
        let events = at_block.events().fetch().await?;

        // Decode only the events of one type, using the generated type.
        for transfer in events.find::<polkadot::balances::events::Transfer>() {
            println!("  Transfer: {:?}", transfer?);
        }

        // Or walk every event and read its pallet and name.
        for event in events.iter() {
            let event = event?;
            println!("  {}.{}", event.pallet_name(), event.event_name());
        }
    }

    Ok(())
}
```

`Events` also provides `find_first` and `has`, and each event exposes `decode_as` and `decode_fields_unchecked_as` to decode into other types, such as `subxt::dynamic::Value`. To list the extrinsics in a block and the events each one emitted, call `at_block.extrinsics().fetch()`, as in the [`blocks.rs`](https://github.com/paritytech/subxt/blob/v{{ dependencies.crates.subxt.version }}/subxt/examples/blocks.rs) example.

## Iterate over Storage

A storage map can be iterated instead of queried one key at a time. Call `iter` on a storage entry with a prefix of its keys:

- **`()`**: Iterates over every entry in the map.
- **`(first_key,)`**: Iterates over the entries under `first_key`, for maps with more than one key.

You can pass up to one fewer key than the map has. The following example lists every account in `System.Account` on the local node:

```rust
let at_block = api.at_current_block().await?;
let accounts = at_block
    .storage()
    .entry(polkadot::storage().system().account())?;

let mut entries = accounts.iter(()).await?;
while let Some(entry) = entries.next().await {
    let entry = entry?;

    // The static address lets you decode the key parts and value into typed Rust values.
    let (account_id,) = entry.key()?.decode()?;
    let info = entry.value().decode()?;
    println!("{account_id}: free balance {}", info.data.free);
}
```

Each item gives you the key and the value. With a dynamic address, decode them into `subxt::dynamic::Value` or another type that implements `DecodeAsType`. For more, see the [`storage_entries.rs`](https://github.com/paritytech/subxt/blob/v{{ dependencies.crates.subxt.version }}/subxt/examples/storage_entries.rs) example.

## Light Client Support

Subxt can connect through an embedded [smoldot](light-clients.md) light client instead of an RPC node, using its `lightclient` module. Enable it with the `light-client` Cargo feature (not on by default). See the [`light_client.rs`](https://github.com/paritytech/subxt/blob/v{{ dependencies.crates.subxt.version }}/subxt/examples/light_client.rs) example in the Subxt repository. To connect to a local network, see [Connect to a Local Network](/tools/light-clients.md#connect-to-a-local-network).

## Where to Go Next

- [Subxt documentation](https://docs.rs/subxt/{{ dependencies.crates.subxt.version }}/subxt/introduction/index.html): Reference material and advanced features.
- [Subxt examples](https://github.com/paritytech/subxt/tree/v{{ dependencies.crates.subxt.version }}/subxt/examples): Runnable examples for transactions, storage, blocks, runtime APIs, and light clients.
