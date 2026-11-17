---
title: Polkadot Omni Node
description: Run parachain nodes with the polkadot-omni-node, a white-labeled binary that can run parachain nodes using a single pre-built solution.
categories: Polkadot SDK, Tooling
---

# Polkadot Omni Node

## Introduction

The [`polkadot-omni-node`](https://crates.io/crates/polkadot-omni-node/{{dependencies.crates.polkadot_omni_node.version}}) crate is a versatile, pre-built binary designed to simplify running parachains in the Polkadot ecosystem. Unlike traditional node binaries that are tightly coupled to specific runtime code, the `polkadot-omni-node` operates using an external Chain Specification file, allowing it to adapt dynamically to different parachains.

This approach enables it to act as a white-labeled node binary, capable of running most parachains that do not require custom node-level logic or extensions. You can use it to run your own parachain locally during development and to run nodes for existing networks, without maintaining a dedicated node codebase for each chain.

This guide covers installing the `polkadot-omni-node`, generating or obtaining a chain specification, and starting a parachain node.

## Install Polkadot Omni Node

Download the pre-built `polkadot-omni-node` binary from the [Polkadot SDK release](https://github.com/paritytech/polkadot-sdk/releases/tag/{{dependencies.repositories.polkadot_sdk.version}}) (recommended):

=== "macOS"

    ```bash
    curl -L -o polkadot-omni-node https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk.version}}/polkadot-omni-node-aarch64-apple-darwin
    chmod +x polkadot-omni-node
    sudo mv polkadot-omni-node /usr/local/bin/
    ```

=== "Ubuntu"

    ```bash
    curl -L -o polkadot-omni-node https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk.version}}/polkadot-omni-node
    chmod +x polkadot-omni-node
    sudo mv polkadot-omni-node /usr/local/bin/
    ```

Alternatively, you can install from source using `cargo`:

!!! note
    Building from source requires Rust and system dependencies. See [Install Dependencies: macOS](/polkadot-sdk/install-polkadot-sdk/#install-dependencies-macos) or [Install Dependencies: Linux](/polkadot-sdk/install-polkadot-sdk/#install-dependencies-linux) before proceeding.

```bash
cargo install --locked polkadot-omni-node@{{dependencies.crates.polkadot_omni_node.version}}
```

To confirm the installation, run:

```bash
polkadot-omni-node --version
```

You should see the installed version number printed to the terminal, confirming a successful installation.

## Obtain Chain Specifications

The `polkadot-omni-node` binary uses a chain specification file to configure and launch a parachain node. This file defines the parachain's genesis state, including its runtime, and its network settings. You can generate a chain specification for your own runtime or use the specification of an existing network.

### Generate Your Own

Use [chain-spec-builder](/tools/chain-spec-builder/) ([`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.staging_chain_spec_builder_version}}) on crates.io), also released as part of the Polkadot SDK, to create a chain specification from your compiled runtime. For example, with a runtime built from the parachain template:

```bash
chain-spec-builder create -t development \
--relay-chain paseo \
--para-id 1000 \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
named-preset development
```

This writes a `chain_spec.json` file. The `--relay-chain` flag records the identifier of the relay chain the parachain is meant to connect to; set it to match the relay chain you run against.

[Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template/) walks through installing chain-spec-builder, building the runtime, and generating the chain specification.

### Use a Live Network's Specification

To run a node for an existing parachain, use that network's chain specification. The [`paritytech/chainspecs`](https://github.com/paritytech/chainspecs) repository collects specifications for live networks, which you can also browse on the [Chainspec Collection](https://paritytech.github.io/chainspecs/) website:

1. Visit the [Chainspec Collection](https://paritytech.github.io/chainspecs/) website.
2. Find the parachain you want to run.
3. Click the chain spec to open it.
4. Copy the JSON content and save it locally as a `.json` file, for example, `chain_spec.json`.

## Run a Parachain Node

To see all available flags and configuration options, run:

```bash
polkadot-omni-node --help
```

### Run Your Own Chain in Development Mode

To run a chain spec you generated, start the node in development mode. It produces and finalizes blocks on its own, without a relay chain:

```bash
polkadot-omni-node --chain ./chain_spec.json --dev
```

The `--dev` option deletes all active data (keys, blockchain database, networking information) when the node stops, so each restart begins from a clean state. To run your parachain alongside a local relay chain, use [Zombienet](zombienet.md).

### Sync a Live Network

To run a full node for a live parachain, replace `./INSERT_PARACHAIN_CHAIN_SPEC.json` with the path to the saved chain spec file:

```bash
polkadot-omni-node --chain ./INSERT_PARACHAIN_CHAIN_SPEC.json --sync warp
```

- The `--chain` flag tells the `polkadot-omni-node` which parachain to run by pointing to its chain specification file.
- The `--sync warp` flag enables warp sync, allowing the node to quickly catch up to the latest finalized state. Historical blocks are fetched in the background as the node continues operating.

Once started, the node connects to peers and syncs with the network. The terminal logs show its progress.

## Interact with the Node

By default, `polkadot-omni-node` exposes a WebSocket endpoint at `{{ networks.local.ws_url }}`, which you can use to interact with the running node. You can connect using:

- Client libraries such as [Subxt](subxt.md), or the libraries listed in [Interact with a Chain](/tools/chain-interactions/#development-tools-and-sdks).
- A web interface that supports Polkadot SDK-based chains, for example, the community-maintained [Polkadot.js Apps](https://polkadot.js.org/apps/#/explorer).

Once connected, you can review blocks, call extrinsics, inspect storage, and interact with the runtime.

## Parachain Compatibility

The `polkadot-omni-node` is designed to work with most parachains out of the box; however, your parachain's runtime must meet specific requirements and follow certain conventions to be compatible. This section outlines what your runtime needs to implement and configure to work with the `polkadot-omni-node`:

- Your runtime must implement the required runtime APIs (see below).
- Your runtime must include and configure the required pallets.

The [`polkadot-sdk-parachain-template`](https://github.com/paritytech/polkadot-sdk-parachain-template/tree/{{dependencies.repositories.polkadot_sdk_parachain_template.version}}) provides a complete reference implementation that is fully compatible with the `polkadot-omni-node`. You can use it as a starting point or reference for ensuring your runtime meets all compatibility requirements.

### Required Runtime APIs

Your parachain runtime must implement the following runtime APIs for the `polkadot-omni-node` to function properly:

- **GetParachainInfo Runtime API**: The omni-node requires the [`GetParachainInfo`](https://paritytech.github.io/polkadot-sdk/master/cumulus_primitives_core/trait.GetParachainInfo.html) runtime API to identify and configure the parachain correctly. This API provides the parachain ID to the node.

    ```rust title="runtime/src/apis.rs"
    impl cumulus_primitives_core::GetParachainInfo<Block> for Runtime {
        fn parachain_id() -> cumulus_primitives_core::ParaId {
            // Return your parachain ID
            ParachainInfo::parachain_id()
        }
    }
    ```

- **Aura Runtime API**: For consensus, the `polkadot-omni-node` expects the [Aura runtime API](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_frame/runtime/apis/trait.AuraApi.html) to be implemented.

    ```rust title="runtime/src/apis.rs"
    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(SLOT_DURATION)
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }
    ```

### Required Pallets

Your runtime must include and properly configure the following pallets:

- **System Pallet**: The System pallet ([`frame-system`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_frame/prelude/frame_system/index.html)) is fundamental and must be configured with appropriate types.

    ```rust title="runtime/src/lib.rs"
    #[frame_support::runtime]
    impl frame_system::Config for Runtime {
        type Block = Block;
        type BlockNumber = BlockNumber;
        // ... other configurations
    }

    // Must be named "System" for omni-node compatibility
    pub type System = frame_system::Pallet<Runtime>;
    ```

- **ParachainSystem Pallet**: This pallet ([`cumulus-pallet-parachain-system`](https://paritytech.github.io/polkadot-sdk/master/cumulus_pallet_parachain_system/index.html)) enables parachain functionality and handles low-level details of being a parachain.

    ```rust title="runtime/src/lib.rs"
    impl cumulus_pallet_parachain_system::Config for Runtime {
        type RuntimeEvent = RuntimeEvent;
        type OnSystemEvent = ();
        // ... other configurations
    }

    // Must be named "ParachainSystem" for omni-node compatibility  
    pub type ParachainSystem = cumulus_pallet_parachain_system::Pallet<Runtime>;
    ```

- **Aura Pallet**: For block authoring consensus ([`pallet-aura`](https://paritytech.github.io/polkadot-sdk/master/pallet_aura/index.html)).

    ```rust title="runtime/src/lib.rs"
    impl pallet_aura::Config for Runtime {
        type AuthorityId = AuraId;
        type DisabledValidators = ();
        type MaxAuthorities = MaxAuthorities;
        type AllowMultipleBlocksPerSlot = ConstBool<false>;
    }

    pub type Aura = pallet_aura::Pallet<Runtime>;
    ```

- **ParachainInfo Pallet**: Provides parachain metadata ([`parachain-info`](https://paritytech.github.io/polkadot-sdk/master/staging_parachain_info/index.html)).

    ```rust title="runtime/src/lib.rs"
    impl parachain_info::Config for Runtime {}

    pub type ParachainInfo = parachain_info::Pallet<Runtime>;
    ```

If you're migrating an existing parachain to use the `polkadot-omni-node`, you may need to perform runtime upgrades to add the required runtime APIs and pallets. Follow the standard parachain [runtime upgrade](/polkadot-sdk/runtime-maintenance/runtime-upgrades/) procedures to apply these changes to a running chain.

## Command-Line Reference

This section summarizes the `polkadot-omni-node` command line at the `{{dependencies.repositories.polkadot_sdk.version}}` release. The options are defined in the Omni Node library's [`cli.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/polkadot-omni-node/lib/src/cli.rs), the Cumulus [`RunCmd`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/client/cli/src/lib.rs), and the shared Substrate node parameters in [`sc-cli`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/client/cli/src). The rustdoc for [`polkadot_omni_node_lib::cli::Cli`](https://paritytech.github.io/polkadot-sdk/master/polkadot_omni_node_lib/cli/struct.Cli.html) and the [Omni Node reference document](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html) describe the same options. Run `polkadot-omni-node --help` for the full list in your installed version.

### Command Structure

The node takes parachain arguments first and, optionally, relay chain arguments after a `--` separator:

```bash
polkadot-omni-node INSERT_PARACHAIN_ARGS -- INSERT_RELAY_CHAIN_ARGS
```

The arguments after `--` configure the relay chain node embedded in the parachain node. The embedded relay chain node uses its own defaults: P2P port `30334`, RPC port `9945`, Prometheus port `9616`, and a `polkadot` directory inside the parachain node's base path. If you do not pass `--chain` after `--`, the relay chain is taken from the `relay_chain` field of the parachain chain spec.

### Runtime Detection

The Omni Node does not compile in a runtime. When it starts, it reads the runtime metadata from the chain spec to decide how to run the chain:

- The block number type, `u32` or `u64`, is read from the `System` pallet's types. If the metadata cannot be read, `u32` is assumed.
- Consensus is always Aura. The Aura authority key type is read from the metadata. If it is not found, the node assumes `sr25519`, except for chain spec IDs that start with `asset-hub-polkadot` or `statemint`, which use `ed25519`.

The node logs the result at startup, for example `Omni Node strategy: BlockNumber=u32, Consensus=Aura(...)`. These checks are implemented in [`common/runtime.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/polkadot-omni-node/lib/src/common/runtime.rs), and the runtime requirements are listed in [Parachain Compatibility](#parachain-compatibility).

### Subcommands

Without a subcommand, `polkadot-omni-node` starts a node. The following subcommands are available:

| Subcommand | Description |
|---|---|
| `chain-spec-builder` | Generates and manages chain specs. Accepts the same subcommands and flags as the standalone [chain-spec-builder](chain-spec-builder.md) tool. |
| `key` | Key management utilities. |
| `export-genesis-head` | Exports the genesis head data of the parachain. `export-genesis-state` is accepted as an alias. |
| `export-genesis-wasm` | Exports the genesis runtime Wasm of the parachain. |
| `check-block` | Validates blocks. |
| `export-blocks` | Exports blocks. |
| `import-blocks` | Imports blocks. |
| `export-state` | Exports the state of a given block into a chain spec. |
| `revert` | Reverts the chain to a previous state. |
| `purge-chain` | Removes the whole chain database. |
| `benchmark` | Benchmarking subcommands. The `pallet` and `storage` benchmarks require a build with the `runtime-benchmarks` feature. |
| `build-spec` | Deprecated. Use `chain-spec-builder` instead. |

### Chain Selection

`--chain` takes the path to a chain spec JSON file. The Omni Node loads every chain spec from disk, so it has no built-in chain names such as `dev` or `local`. Generate a chain spec with [chain-spec-builder](chain-spec-builder.md), or use the spec of an existing network.

### Development Mode

The following flags start a standalone development node. In this mode, the node does not connect to a relay chain or a parachain network. It runs the runtime from the chain spec and seals blocks itself.

| Flag | Description |
|---|---|
| `--dev` | Starts a development node that produces a block every 3000 ms, unless `--dev-block-time` sets another interval. Also implies `--tmp` when no `--base-path` is given, so the node's data is deleted when it stops. |
| `--dev-block-time INSERT_MILLISECONDS` | Produces a block at the given interval in milliseconds. Cannot be combined with `--instant-seal`. |
| `--instant-seal` | Produces a block as soon as a transaction is received, instead of at fixed intervals. Cannot be combined with `--dev-block-time`. |

For example, to produce a block every second:

```bash
polkadot-omni-node --chain ./chain_spec.json --dev --dev-block-time 1000
```

### Collator Flags

Use the following flags to run the node as a collator that authors parachain blocks:

| Flag | Default | Description |
|---|---|---|
| `--collator` | `false` | Runs the node as a collator. Equivalent to `--validator`. |
| `--authoring INSERT_POLICY` | `lookahead` | Selects the collator implementation. `lookahead` builds a block once per imported relay chain block and on relay chain forks. `slot-based` builds blocks based on time, can use multiple cores, and is meant for chains that use elastic scaling. |
| `--relay-chain-rpc-urls INSERT_URLS` | None | Retrieves relay chain data from the given relay chain RPC endpoints instead of a full embedded relay chain node, which reduces resource use. URLs are tried in order of preference. |
| `--no-hardware-benchmarks` | `false` | Skips the hardware benchmarks that run at startup. |
| `--export-pov-to-path INSERT_DIRECTORY` | None | Exports every proof of validity (PoV) the collator builds to the given directory, for debugging. |
| `--experimental-max-pov-percentage INSERT_PERCENTAGE` | None | Experimental. Limits block size to a percentage of the relay chain's maximum PoV size. |
| `--collator-reserved-slots INSERT_COUNT` | `32` | Upper bound on collator reserved-peer slots. |
| `--no-dht-bootnode` | `false` | Does not advertise the node as a parachain bootnode on the relay chain DHT. |
| `--no-dht-bootnode-discovery` | `false` | Does not discover parachain bootnodes through the relay chain DHT. |

At the `{{dependencies.repositories.polkadot_sdk.version}}` release, the deprecated `--experimental-use-slot-based` flag is still accepted. Use `--authoring slot-based` instead.

[Run a Collator](/nodes/run-a-collator/) shows these flags in a complete collator setup.

### Common Node Flags

The Omni Node accepts the standard Polkadot SDK node flags from `sc-cli`. The following are the ones you are most likely to need:

| Flag | Default | Description |
|---|---|---|
| `-d`, `--base-path INSERT_PATH` | Platform-specific directory | Directory for the database, keystore, and node key. |
| `--tmp` | `false` | Uses a temporary base path that is deleted when the node stops. |
| `--name INSERT_NAME` | Random | The human-readable node name used on the network. |
| `--port INSERT_PORT` | `30333` | The P2P listening port. |
| `--rpc-port INSERT_PORT` | `9944` | The JSON-RPC listening port. |
| `--rpc-external` | `false` | Listens for RPC connections on all interfaces instead of only local ones. Not all RPC methods are safe to expose publicly, so put an RPC proxy in front of a public node. |
| `--rpc-cors INSERT_ORIGINS` | Local origins and `https://polkadot.js.org` | Comma-separated browser origins allowed to access the RPC server. `all` disables origin checks. In `--dev` mode, all origins are allowed by default. |
| `--bootnodes INSERT_MULTIADDRS` | None | Additional boot nodes to connect to. |
| `--sync INSERT_MODE` | `full` | Sync mode: `full`, `fast`, `fast-unsafe`, or `warp`. |
| `-l`, `--log INSERT_PATTERN` | `info` | Log filter, for example `--log info,parachain=debug`. |

### Statement Store Flags

The Omni Node can run a statement store, an off-chain store of signed statements whose permissions are read from chain state. Enable it with `--enable-statement-store`. The following flags tune it and only apply when it is enabled: `--statement-network-workers`, `--statement-rate-limit`, `--statement-store-max-total-statements`, `--statement-store-max-total-size`, and `--statement-store-purge-after-sec`. See `polkadot-omni-node --help` for their defaults.
