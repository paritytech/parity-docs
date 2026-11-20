---
title: Run a Bulletin Chain Node
description: Fetch polkadot-omni-node, build a Bulletin Chain spec, and run a local dev node or Zombienet network with IPFS serving enabled, then connect tools to it.
categories: Apps, Reference
---

# Run a Bulletin Chain Node

## Introduction

The Bulletin Chain has no node binary of its own. Its runtimes, `bulletin-westend-runtime` and `bulletin-paseo-runtime`, live in the [`paritytech/polkadot-bulletin-chain`](https://github.com/paritytech/polkadot-bulletin-chain) repository, and any node runs them with the Polkadot SDK's `polkadot-omni-node`. Parity publishes this software; anyone can run a Bulletin Chain node or network from it.

This page runs a single Bulletin Chain node locally in development mode, describes the storage and IPFS flags a Bulletin node uses, points to the Zombienet configuration for a local relay chain and parachain network, and shows how to connect tools to the node. For what the chain stores and how authorization works, see the [Bulletin Chain overview](/apps/reference/infrastructure/bulletin-chain/index.md) and [Data Storage](/apps/reference/infrastructure/bulletin-chain/data-storage.md).

!!! warning "Unaudited reference code"
    The repository describes the Bulletin Chain as a reference implementation that has not been fully audited and is not recommended for production use without independent review. See its [Security section](https://github.com/paritytech/polkadot-bulletin-chain#security) before you run it for real use.

## Prerequisites

Before you start, make sure you have the following:

- A Rust toolchain and the build dependencies from [Install Polkadot SDK Dependencies](/polkadot-sdk/install-polkadot-sdk.md). The chain spec step compiles the runtime.
- [`just`](https://github.com/casey/just), which runs every recipe in the repository. Install it with `cargo install just --locked`.

On macOS, a missing C++ header (`algorithm` file not found) or `libclang.dylib` error during the build is covered in the repository's [Troubleshooting section](https://github.com/paritytech/polkadot-bulletin-chain#troubleshooting).

## Get the Node and a Chain Spec

1. Clone the repository:

    ```bash
    git clone https://github.com/paritytech/polkadot-bulletin-chain.git
    cd polkadot-bulletin-chain
    ```

2. Fetch the node binaries:

    ```bash
    just binaries-polkadot
    ```

    The recipe fetches `polkadot-omni-node`, plus the `polkadot` relay chain binary and its workers, into a versioned folder under `./.polkadot-binaries/`, such as `polkadot-node/INSERT_VERSION/darwin-aarch64/`, and prints that folder's path. Later commands use the printed path. The version comes from `POLKADOT_NODE_VERSION` in the repository's `.github/env` file. For a release tag, the script downloads the prebuilt binary for Linux x86_64 or macOS arm64 and verifies its `.sha256` file; for a 40-character commit hash, it clones the Polkadot SDK and builds the binaries. To use another version for one session, set the variable on the command line, for example `POLKADOT_NODE_VERSION=INSERT_RELEASE_TAG just binaries-polkadot`.

3. Build a chain spec for the Westend-flavored runtime:

    ```bash
    just chain-spec westend
    ```

    The recipe builds `bulletin-westend-runtime` and writes the chain spec to `zombienet/bulletin-westend-spec.json`. Run `just chain-spec paseo` for the Paseo-flavored runtime. If the spec already exists, the script skips the build; set `FORCE_REBUILD_SPEC=1` to regenerate it.

## Run a Dev Node

Start a single node that produces and finalizes its own blocks, with no relay chain:

```bash
OMNI_NODE="$(just binaries-polkadot)/polkadot-omni-node"
"$OMNI_NODE" --chain ./zombienet/bulletin-westend-spec.json --dev --ipfs-server
```

The node behaves as follows:

- It serves RPC and WebSocket connections at `{{ networks.local.ws_url }}`.
- `--dev` deletes the node's database when the node stops.
- `//Alice` holds the sudo key, so `//Alice` can grant storage authorizations.
- `--ipfs-server` lets IPFS peers fetch stored data from the node over Bitswap.

A fresh chain stores nothing until an account is authorized. With Polkadot.js Apps connected to `{{ networks.local.ws_url }}`, use **Developer** > **Sudo** to call `transactionStorage.authorizeAccount` with the account, a transaction count, and a byte allowance, or `transactionStorage.authorizePreimage` to sponsor one known blob by its content hash. The authorized account then submits `transactionStorage.store`. See [Authorization](/apps/reference/infrastructure/bulletin-chain/authorization.md) for what an authorization records.

## Storage and IPFS Flags

The flags below come from `polkadot-omni-node`. Run `polkadot-omni-node --help` for the full list and the defaults of the version you fetched.

| Flag | Purpose |
|:-----|:--------|
| `--ipfs-server` | Join the IPFS network and serve stored transactions over the Bitswap protocol. |
| `--ipfs-bootnodes` | IPFS bootstrap nodes, as multiaddresses with peer IDs. Requires `--ipfs-server`. |
| `--enable-hop` | Enable the [Hand-Off Protocol (HOP)](/apps/reference/infrastructure/hop/index.md) data pool on the node. |
| `--hop-max-pool-size`, `--hop-max-user-size` | The HOP pool size and the per-user cap, in MiB. |
| `--hop-retention-secs`, `--hop-check-interval`, `--hop-promotion-buffer-secs` | How long HOP keeps entries, how often it cleans up expired entries, and how long before expiry it starts promoting entries on chain, in seconds. |
| `--hop-disable-rate-limit` | Turn off per-account HOP submit rate limiting, intended for tests and dev nodes. |
| `--hop-data-dir` | Directory for HOP data. Defaults to `hop` inside the chain data directory. |

The dev node command above enables only `--ipfs-server`. The repository's Zombienet configuration runs its collators with `--ipfs-server`, the HOP flags, `--pool-kbytes 65536`, and `--authoring slot-based`. The slot-based collator is required there because the runtime enforces `RelayParentOffset`.

## Run a Local Network

To run the Bulletin Chain as a parachain with a local relay chain, use the Zombienet configurations in the repository's `zombienet/` folder:

- `bulletin-westend-local.toml`: A local relay chain with three validators and two collators running the Westend-flavored Bulletin runtime as parachain `1010`.
- `bulletin-paseo-local.toml`: The same layout for the Paseo-flavored Bulletin runtime as parachain `1501`. Only its first collator enables HOP.

Both files start the relay chain from the `westend-local` chain spec.

To start the Westend-flavored network:

1. Fetch the Zombienet binary, and build the chain spec if you haven't already:

    ```bash
    just binaries-zombienet
    just chain-spec westend
    ```

2. From the repository root, spawn the network. The configuration resolves `chain_spec_path` relative to the current directory and reads the binary paths from two environment variables:

    ```bash
    BIN_DIR="$(just binaries-polkadot)"
    POLKADOT_BINARY_PATH="$BIN_DIR/polkadot" \
    POLKADOT_PARACHAIN_BINARY_PATH="$BIN_DIR/polkadot-omni-node" \
    "$(just binaries-zombienet)/zombienet" -p native spawn ./zombienet/bulletin-westend-local.toml
    ```

    Set `POLKADOT_PARACHAIN_BINARY_PATH` to `polkadot-omni-node`, as above. The comment at the top of each configuration file names a `polkadot-parachain` binary that `just binaries-polkadot` doesn't fetch.

The relay chain validators serve RPC on ports `9942`, `9943`, and `9944`, and the two collators on `10000` and `12346`. Connect Bulletin tools to a collator, for example `ws://127.0.0.1:10000`, not to port `9944`, which is a relay chain validator in this network. The parachain produces its first block after it onboards, which takes a few minutes after the relay chain starts.

For Zombienet itself, see [Run a Parachain Network](/polkadot-sdk/testing/run-a-parachain-network.md).

For a full round trip without manual setup, the repository's examples start a local network and a Kubo IPFS node, authorize an account, store data, and read it back over IPFS:

```bash
cd examples
just run-authorize-and-store bulletin-westend-runtime ws kubo-native
```

The `kubo-native` mode downloads a Kubo binary; the default `kubo-docker` mode runs Kubo in Docker instead. See the [examples README](https://github.com/paritytech/polkadot-bulletin-chain/blob/main/examples/README.md) for the other recipes, including a Smoldot light client variant.

## Connect Tools to the Node

A local node exposes the same interfaces as any Bulletin Chain node, so the following tools connect to it:

- **Polkadot.js Apps**: Connect to `{{ networks.local.ws_url }}` to submit authorizations as sudo and `transactionStorage.store` extrinsics.
- **Bulletin SDK**: [`@parity/bulletin-sdk`](https://www.npmjs.com/package/@parity/bulletin-sdk) builds on `polkadot-api`. Create the `polkadot-api` client from a WebSocket provider for `{{ networks.local.ws_url }}` and pass it to `AsyncBulletinClient`, as the [SDK README](https://github.com/paritytech/polkadot-bulletin-chain/blob/main/sdk/typescript/README.md) shows. The Rust SDK lives in the repository's `sdk/rust/` folder.
- **An IPFS gateway**: The node speaks Bitswap over libp2p, not HTTP. To fetch a CID over HTTP, run an IPFS node such as Kubo peered with the Bulletin node and request `/ipfs/INSERT_CID` from its gateway. The examples' `ipfs-kubo-start` recipe in `examples/justfile` shows a working Kubo configuration, with peering set through `Peering.Peers` and the gateway on port `8283`.

A Product does not connect to a node directly: the Host provides chain access. To store data from a Product, see [Store Data on Chain](/apps/build/store-data-on-chain.md).

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Data Storage**

    ---

    The `TransactionStorage` and `DataRenewal` pallets, IPFS retrieval, and size limits.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/bulletin-chain/data-storage.md)

- <span class="badge learn">Learn</span> **Authorization**

    ---

    What an authorization records, what a write costs, and when an authorization expires.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/bulletin-chain/authorization.md)

- <span class="badge learn">Learn</span> **Chunked Uploads**

    ---

    How large files are split into chunks with a DAG-PB manifest.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/bulletin-chain/chunked-upload.md)

- <span class="badge learn">Learn</span> **Renewal**

    ---

    How long stored data is retained and how renewal extends it.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/bulletin-chain/renewal.md)

</div>
