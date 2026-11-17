---
title: Run the Ethereum JSON-RPC Adapter
description: Build and run eth-rpc, the Ethereum JSON-RPC adapter for pallet-revive, against a local node, and configure its storage, RPC server, and logging options.
short_description: Build, run, and configure the eth-rpc adapter.
categories: Smart Contracts, Tooling
---

# Run the Ethereum JSON-RPC Adapter

## Introduction

The Ethereum JSON-RPC adapter, `eth-rpc`, is a server that lets Ethereum wallets, libraries, and development tools work with a chain whose runtime includes `pallet-revive`. It accepts standard Ethereum JSON-RPC requests, translates them into Polkadot SDK RPC calls and runtime API calls to a node, and returns Ethereum-compatible responses.

The adapter is the `eth-rpc` binary of the [`pallet-revive-eth-rpc`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/rpc) crate in the Polkadot SDK. This page shows how to build it, run it next to a local node, and configure it. For the Ethereum methods it serves, see [JSON-RPC APIs](/smart-contracts/for-eth-devs/json-rpc-apis.md).

## Prerequisites

Before you start, make sure you have:

- [Rust and the Polkadot SDK dependencies](/polkadot-sdk/install-polkadot-sdk.md) installed, if you build the adapter from source.
- A node whose runtime includes `pallet-revive`. For local development, the [Local Development Node](/smart-contracts/dev-environments/local-dev-node.md) guide shows how to build the Revive dev node. To add `pallet-revive` to your own runtime, see [Add pallet-revive to Your Runtime](/polkadot-sdk/customize-runtime/add-pallet-revive.md).

## How the Adapter Works

`eth-rpc` runs as a separate process and connects to a node over WebSocket, by default at `{{ networks.local.ws_url }}`. On startup, it reads the `ChainId` constant of `pallet-revive` from the node, which it returns for `eth_chainId`.

While it runs, the adapter subscribes to new best and finalized blocks and stores Ethereum receipt data in a SQLite database, which it uses to answer queries such as `eth_getTransactionReceipt` and `eth_getLogs`. Depending on the [pruning mode](#configure-storage-and-pruning), it also syncs historical blocks backward from the chain head.

It serves JSON-RPC over HTTP and WebSocket on port `8545` by default, so a local adapter is reachable at `{{ networks.local.eth_rpc_url }}`.

## Get the Adapter

You can build `eth-rpc` from the Polkadot SDK source or use the `parity/eth-rpc` Docker image.

### Build from Source

Clone the Polkadot SDK at the release you want and build the `eth-rpc` binary:

```bash
git clone --branch {{dependencies.repositories.polkadot_sdk.version}} --depth 1 https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk
cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release
```

The binary is written to `target/release/eth-rpc`. Check that it runs:

```bash
./target/release/eth-rpc --help
```

!!! note
    The adapter's client is generated at build time from the metadata of the Revive dev runtime in the same source tree. Building the adapter from the same Polkadot SDK release as the runtime you connect it to keeps the two in step.

### Use the Docker Image

The [`parity/eth-rpc`](https://hub.docker.com/r/parity/eth-rpc/tags) image contains the `eth-rpc` binary as its entry point. Tags follow the Polkadot SDK release naming, such as `{{dependencies.repositories.polkadot_sdk.docker_image_version}}`. Without arguments, the container prints the help text. The image exposes port `8545` for JSON-RPC and port `9616` for Prometheus metrics.

```bash
docker run --rm --network=host parity/eth-rpc:{{dependencies.repositories.polkadot_sdk.docker_image_version}} --dev
```

The image is built from the [`eth-rpc` Dockerfile](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/rpc/dockerfiles/eth-rpc/Dockerfile) in the Polkadot SDK repository.

## Run the Adapter Against a Local Node

1. Start a local node with `pallet-revive` in its runtime. For example, from the `polkadot-sdk` directory, build and start the Revive dev node:

    ```bash
    cargo build -p revive-dev-node --bin revive-dev-node --release
    ./target/release/revive-dev-node --dev
    ```

2. In a second terminal, start the adapter in development mode:

    ```bash
    ./target/release/eth-rpc --dev
    ```

    The adapter connects to the node at `{{ networks.local.ws_url }}` and serves JSON-RPC at `{{ networks.local.eth_rpc_url }}`.

3. In a third terminal, check that the adapter responds by requesting the chain ID:

    ```bash
    curl -s -X POST {{ networks.local.eth_rpc_url }} \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
    ```

    The result is the `ChainId` that the node's runtime configures for `pallet-revive`, in hexadecimal.

The `--dev` flag changes the adapter's behavior in the following ways:

- **Temporary storage**: Without `--base-path`, the database is created in a temporary directory that is deleted on exit. The adapter logs a warning about this when it runs in archive mode. Pass `--base-path` to keep the database between runs.
- **CORS**: All origins are allowed, unless you set `--rpc-cors`.
- **Development accounts**: `eth_accounts` returns five development accounts (Alith, Baltathar, Charleth, Dorothy, and Ethan). Outside development mode, the adapter manages no accounts.

To see more detail, raise the log level for the `eth-rpc` target:

```bash
RUST_LOG="info,eth-rpc=debug" ./target/release/eth-rpc --dev
```

To connect to a node on another address, pass its WebSocket URL:

```bash
./target/release/eth-rpc --node-rpc-url ws://INSERT_NODE_HOST:INSERT_NODE_PORT
```

## Configure Storage and Pruning

The `--eth-pruning` flag selects how the adapter stores receipt data:

- **`archive` (default)**: Stores data in a persistent on-disk database and syncs all historical blocks. This mode requires the node to be an archive node.
- **`<N>`**: Keeps only the latest `N` blocks in an in-memory database. `N` must be at least `1`. The adapter does not sync historical blocks in this mode.

In archive mode, the adapter creates an `eth-rpc.db` SQLite database in the directory set by `--base-path` (short form `-d`). Without `--base-path` and outside `--dev`, it uses a platform default directory:

| Platform | Default Database Directory |
|:--------:|:--------------------------:|
| Linux | `~/.local/share/eth-rpc/` |
| macOS | `~/Library/Application Support/eth-rpc/` |
| Windows | `%APPDATA%\eth-rpc\` |

For example, to keep a local database in `./eth-rpc-data`:

```bash
./target/release/eth-rpc --dev --base-path ./eth-rpc-data
```

To keep only the latest 256 blocks in memory:

```bash
./target/release/eth-rpc --eth-pruning 256
```

## Command-Line Options

`eth-rpc` defines a few options of its own and reuses the shared RPC, logging, and Prometheus options of the Polkadot SDK node CLI. The options below come from the adapter's [CLI source](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/rpc/src/cli.rs) and the [`sc-cli` parameter definitions](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/client/cli/src/params). Run `eth-rpc --help` for the complete list.

### Adapter Options

| Option | Description | Default |
|:------:|:-----------:|:-------:|
| `--node-rpc-url` | WebSocket URL of the node to connect to | `ws://127.0.0.1:9944` |
| `--eth-pruning` | Receipt storage mode: `archive` or a number of blocks to keep in memory | `archive` |
| `--allow-unprotected-txs` | Accept transactions without a chain ID, which the adapter rejects by default | Disabled |
| `--dev` | Development mode, as described in [Run the Adapter Against a Local Node](#run-the-adapter-against-a-local-node) | Disabled |
| `-d`, `--base-path` | Directory for the `eth-rpc.db` database | Platform default, or a temporary directory with `--dev` |

### RPC Server Options

| Option | Description | Default |
|:------:|:-----------:|:-------:|
| `--rpc-port` | Port for the JSON-RPC server | `8545` |
| `--rpc-external` | Listen on all network interfaces instead of only on localhost | Disabled |
| `--unsafe-rpc-external` | Same as `--rpc-external`, without the warning | Disabled |
| `--rpc-cors` | Comma-separated list of allowed browser origins, or `all` to disable origin checks | Localhost and `https://polkadot.js.org`; all origins with `--dev` |
| `--rpc-max-connections` | Maximum number of RPC server connections | `100` |
| `--rpc-max-request-size` | Maximum request payload size in megabytes | `15` |
| `--rpc-max-response-size` | Maximum response payload size in megabytes | `15` |
| `--rpc-max-subscriptions-per-connection` | Maximum concurrent subscriptions per connection | `1024` |
| `--rpc-message-buffer-capacity-per-connection` | Number of messages the server buffers per connection | `64` |
| `--rpc-rate-limit` | Maximum calls per minute for each connection | Disabled |
| `--rpc-rate-limit-whitelisted-ips` | IP ranges in CIDR notation that are exempt from rate limiting | None |
| `--rpc-rate-limit-trust-proxy-headers` | Trust `X-Real-IP`, `X-Forwarded-For`, and `Forwarded` headers for rate limiting | Disabled |
| `--rpc-disable-batch-requests` | Reject batch requests | Disabled |
| `--rpc-max-batch-request-len` | Maximum number of calls in a batch request | None |

The adapter also applies `--rpc-max-request-size` and `--rpc-max-response-size` to its own connection to the node.

### Logging and Metrics Options

| Option | Description | Default |
|:------:|:-----------:|:-------:|
| `-l`, `--log` | Log filter, such as `info,eth-rpc=debug` | `info` for all targets |
| `--detailed-log-output` | Include the log target, level, and thread name in log lines | Disabled |
| `--disable-log-color` | Disable colored log output | Disabled |
| `--prometheus-port` | Port for the Prometheus metrics endpoint | `9616` |
| `--prometheus-external` | Expose the metrics endpoint on all interfaces | Disabled |
| `--no-prometheus` | Disable the metrics endpoint | Disabled |

### Removed Options

[PR #11153](https://github.com/paritytech/polkadot-sdk/pull/11153) removed several storage options. The adapter exits with an error that references the PR if you pass any of them:

| Removed Option | Replacement |
|:--------------:|:-----------:|
| `--database-url` | `--base-path` for an on-disk database, or `--eth-pruning=<N>` for an in-memory database |
| `--cache-size` | `--eth-pruning=<N>` |
| `--index-last-n-blocks` | `--eth-pruning=archive` |
| `--earliest-receipt-block` | None; the adapter discovers the first EVM block |

## Production Considerations

The adapter's source documents the following points that matter outside local development:

- **Archive node**: The default `archive` pruning mode syncs all historical blocks and requires the node to be an archive node. Against a pruned node, consider `--eth-pruning=<N>`, which keeps only recent blocks.
- **Public exposure**: By default, the adapter listens only on localhost. The `--rpc-external` option warns that not all RPC methods are safe to expose publicly and recommends placing an RPC proxy server in front of the endpoint to filter methods.
- **Persistent storage**: Set `--base-path` to a persistent directory so the receipt database survives restarts.
- **Unprotected transactions**: Leave `--allow-unprotected-txs` disabled unless you need to accept transactions signed without a chain ID ([EIP-155](https://eips.ethereum.org/EIPS/eip-155) replay protection).

For Docker and systemd setups next to a parachain RPC node, see [Ethereum RPC Compatibility](/nodes/run-a-node/parachain-rpc.md#ethereum-rpc-compatibility).

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge reference">Reference</span> __JSON-RPC APIs__

    ---

    Browse the Ethereum JSON-RPC methods the adapter supports, with request examples.

    [:octicons-arrow-right-24: Reference](/smart-contracts/for-eth-devs/json-rpc-apis.md)

-   <span class="badge guide">Guide</span> __Add pallet-revive to Your Runtime__

    ---

    Add `pallet-revive` and the runtime APIs the adapter needs to your own Polkadot SDK runtime.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/customize-runtime/add-pallet-revive.md)

</div>
