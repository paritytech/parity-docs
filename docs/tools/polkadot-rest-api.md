---
title: Polkadot REST API
description: Run Polkadot REST API, the Rust successor to Substrate API Sidecar, to query Polkadot SDK-based chains and submit transactions over HTTP.
categories: Tooling
---

# Polkadot REST API

## Introduction

[Polkadot REST API](https://github.com/paritytech/polkadot-rest-api) is a REST service for interacting with Polkadot SDK-based nodes. You run it next to a node, and it exposes HTTP endpoints for blocks, accounts, pallet storage, transactions, and other chain data, so your application can read chain state without a WebSocket connection or an SDK.

Polkadot REST API is a Rust rewrite of [Substrate API Sidecar](https://github.com/paritytech/substrate-api-sidecar) built on [Subxt](/tools/subxt.md). The Sidecar README marks `substrate-api-sidecar` as deprecated in favor of this project and describes it as having 1:1 API compatibility, with endpoints served under the `/v1/` prefix. If you already run Sidecar, see [Migrating from Sidecar](#migrating-from-sidecar).

This page covers installing Polkadot REST API, configuring it, running it against a local development node, and sending your first requests.

## Prerequisites

- A Polkadot SDK-based node with its RPC endpoint reachable from where you run the service. For local development, start a node listening on `{{ networks.local.ws_url }}`, for example with [Polkadot Omni Node](/tools/omninode.md) or [Zombienet](/tools/zombienet.md).
- One of the following, depending on how you install:
    - A Rust toolchain with `cargo`, to install from crates.io or build from source. See [Install Dependencies: macOS](/polkadot-sdk/install-polkadot-sdk.md#install-dependencies-macos) or [Install Dependencies: Linux](/polkadot-sdk/install-polkadot-sdk.md#install-dependencies-linux).
    - [Docker](https://www.docker.com/), to run the published container image.
- [curl](https://curl.se/) or any HTTP client to send requests.

## Install Polkadot REST API

The project publishes the [`polkadot-rest-api`](https://crates.io/crates/polkadot-rest-api) crate on crates.io and the [`paritytech/polkadot-rest-api`](https://hub.docker.com/r/paritytech/polkadot-rest-api) image on Docker Hub. It does not attach prebuilt binaries to its [GitHub releases](https://github.com/paritytech/polkadot-rest-api/releases), so install it with Cargo, build it from source, or run the container image.

=== "Cargo"

    Install the binary from crates.io:

    ```bash
    cargo install --locked polkadot-rest-api@{{ dependencies.crates.polkadot_rest_api.version }}
    ```

    Confirm the installation:

    ```bash
    polkadot-rest-api --version
    ```

=== "From source"

    Clone the repository and build the release binary:

    ```bash
    git clone https://github.com/paritytech/polkadot-rest-api.git
    cd polkadot-rest-api
    cargo build --release --bin polkadot-rest-api
    ```

    The binary is written to `target/release/polkadot-rest-api`. You can also build and start it in one step with `cargo run --release --bin polkadot-rest-api`.

=== "Docker"

    Pull the image:

    ```bash
    docker pull docker.io/paritytech/polkadot-rest-api:v{{ dependencies.crates.polkadot_rest_api.version }}
    ```

    The image sets `SAS_EXPRESS_PORT=8080` and `SAS_EXPRESS_BIND_HOST=0.0.0.0` and exposes port `8080`. See the upstream [`Dockerfile`](https://github.com/paritytech/polkadot-rest-api/blob/main/Dockerfile) for details.

## Configuration

Polkadot REST API reads its configuration from environment variables with the `SAS_` prefix, the same prefix Sidecar uses. The most common variables are:

| Variable | Default | Description |
| --- | --- | --- |
| `SAS_SUBSTRATE_URL` | `ws://127.0.0.1:9944` | RPC endpoint of the node to connect to. Accepts `ws://`, `wss://`, `http://`, and `https://` URLs. |
| `SAS_SUBSTRATE_MULTI_CHAIN_URL` | Empty | JSON array of additional chain connections, each with a `url` and a `type` of `relay`, `assethub`, or `parachain`. Used to give an Asset Hub deployment access to its relay chain. |
| `SAS_EXPRESS_BIND_HOST` | `127.0.0.1` | IP address the HTTP server binds to. Must be an IP address, not a hostname. |
| `SAS_EXPRESS_PORT` | `8080` | Port the HTTP server listens on. |
| `SAS_EXPRESS_REQUEST_LIMIT` | `512000` | Maximum request body size, in bytes. |
| `SAS_LOG_LEVEL` | `info` | Log level. Accepts `trace`, `debug`, `info`, `http`, `warn`, and `error`, or a full `tracing_subscriber` filter directive such as `info,subxt_rpcs=debug`. |
| `SAS_METRICS_ENABLED` | `false` | When `true`, exposes Prometheus metrics at `/metrics`. |

For the full list, including logging, reconnection, and metrics settings, see the [configuration source](https://github.com/paritytech/polkadot-rest-api/blob/main/crates/config/src/lib.rs) and the [configuration changes](https://github.com/paritytech/polkadot-rest-api/blob/main/docs/guides/MIGRATION.md#configuration-changes) section of the migration guide.

You can also keep the variables in a file. At startup, the service loads `.env` from the working directory if it exists. To load a different file, pass `--env-file`:

```bash
polkadot-rest-api --env-file .env.local
```

The repository includes an [`.env.sample`](https://github.com/paritytech/polkadot-rest-api/blob/main/.env.sample) file you can use as a template.

## Run Against a Local Node

With a local development node listening on `{{ networks.local.ws_url }}`, start the service:

=== "Binary"

    ```bash
    SAS_SUBSTRATE_URL={{ networks.local.ws_url }} polkadot-rest-api
    ```

    Because `{{ networks.local.ws_url }}` is the default node URL, running `polkadot-rest-api` without the variable connects to the same node.

=== "Docker"

    ```bash
    docker run --rm -it --read-only -p 8080:8080 \
      -e SAS_SUBSTRATE_URL=INSERT_NODE_WS_URL \
      docker.io/paritytech/polkadot-rest-api:v{{ dependencies.crates.polkadot_rest_api.version }}
    ```

    Replace `INSERT_NODE_WS_URL` with a WebSocket endpoint of your node that is reachable from inside the container. Inside a container, `127.0.0.1` refers to the container itself, not to your host.

The service listens on `http://127.0.0.1:8080` by default. All endpoints are served under the `/v1` prefix.

To connect to another chain, set `SAS_SUBSTRATE_URL` to that chain's RPC endpoint. For public RPC endpoints of Polkadot networks, see the [Polkadot developer portal](https://developers.polkadot.com/).

## Send Requests

With the service running, you can call its endpoints with `curl`, a browser, or any HTTP client.

To check the node the service is connected to, call `/v1/node/version`:

```bash
curl -s http://127.0.0.1:8080/v1/node/version
```

The response is a JSON object with the node's `clientVersion`, `clientImplName`, and `chain`.

To fetch the latest block, call `/v1/blocks/head`:

```bash
curl -s http://127.0.0.1:8080/v1/blocks/head
```

To check the version of Polkadot REST API itself, call `/v1/version`. To see which pallets the connected chain supports, its chain type, and its SS58 prefix, call `/v1/capabilities`.

Most read endpoints accept an `at` query parameter with a block hash or block number, so you can query historical state:

```bash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/balance-info?at=INSERT_BLOCK_NUMBER"
```

For task-oriented examples covering balances, assets, pallet storage, and blocks, see [Query On-Chain State with Polkadot REST API](/tools/chain-interactions/query-data/query-rest.md).

### API Reference

Each running instance serves its own interactive API documentation at `http://127.0.0.1:8080/docs` and an OpenAPI 3.0 specification at `http://127.0.0.1:8080/api-docs/openapi.json`. These paths are not under `/v1`. The same documentation is published at [paritytech.github.io/polkadot-rest-api](https://paritytech.github.io/polkadot-rest-api/).

## Migrating from Sidecar

The upstream [migration guide](https://github.com/paritytech/polkadot-rest-api/blob/main/docs/guides/MIGRATION.md) documents every difference between Sidecar v20.14.0 and Polkadot REST API. The changes most likely to affect an existing integration are:

- **Endpoint prefix**: All endpoints move under `/v1`. For example, `GET /blocks/head` becomes `GET /v1/blocks/head`, and `POST /transaction` becomes `POST /v1/transaction`. Update client URLs and health checks by prepending `/v1` to existing paths.
- **Install and start**: Replace the `@substrate/api-sidecar` npm package with the `polkadot-rest-api` crate, and start the service with `polkadot-rest-api` instead of `substrate-api-sidecar`. No Node.js runtime is needed.
- **Docker image**: Replace `parity/substrate-api-sidecar` with `paritytech/polkadot-rest-api`. The image already binds to `0.0.0.0`, so you can remove `SAS_EXPRESS_BIND_HOST=0.0.0.0` from your environment.
- **Configuration**: Most `SAS_` variables keep their names and defaults. The custom type variables (`SAS_SUBSTRATE_TYPES_BUNDLE`, `SAS_SUBSTRATE_TYPES_CHAIN`, `SAS_SUBSTRATE_TYPES_SPEC`, and `SAS_SUBSTRATE_TYPES`), `SAS_SUBSTRATE_CACHE_CAPACITY`, `SAS_EXPRESS_INJECTED_CONTROLLERS`, and `SAS_LOG_FILTER_RPC` are not supported. `SAS_EXPRESS_MAX_BODY` is replaced by `SAS_EXPRESS_REQUEST_LIMIT`.
- **Metrics**: Prometheus metrics are served on the main port at `/metrics` instead of on a separate port `9100`. Update your scrape configuration.
- **Asset balances**: The `asset-balances`, `pool-asset-balances`, and `foreign-asset-balances` account endpoints no longer return assets with a zero balance by default. Add `showEmpty=true` to restore the previous behavior.
- **Relay chain block format**: Sidecar's `useRcBlockFormat=object` is replaced by `format=object` combined with `useRcBlock=true`.
- **Pallet metadata types**: The `args` field in pallet metadata responses returns the type names from the runtime metadata instead of fully expanded type definitions.
- **Removed endpoints**: The experimental trace endpoints, `POST /contracts/ink/{address}/query`, and most parachain endpoints under `/paras` are not implemented. `GET /v1/paras/{paraId}/head-inclusions` is available.

The migration guide also lists behavior fixes, such as endpoints that now honor the `at` parameter for historical queries, and endpoints that are new in Polkadot REST API.

## Where to Go Next

<div class="grid cards" markdown>

-   __Query On-Chain State with Polkadot REST API__

    ---

    Query balances, assets, pallet storage, and blocks with `curl`.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-rest.md)

-   __Subxt__

    ---

    Use the Rust library that Polkadot REST API is built on for type-safe, programmatic chain access.

    [:octicons-arrow-right-24: Get Started](/tools/subxt.md)

</div>
