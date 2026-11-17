---
title: Query On-Chain State with Polkadot REST API
description: Learn how to query on-chain state from a Polkadot SDK-based chain using a self-run Polkadot REST API and curl, including balances, assets, and blocks.
short_description: Query chain data through standardized REST endpoints for simpler integration.
categories: Chain Interactions, Tooling
---

# Query On-Chain State with Polkadot REST API

## Introduction

[Polkadot REST API](/tools/polkadot-rest-api.md) is a REST service that Parity maintains for Polkadot SDK-based blockchains. It provides an HTTP interface to query account balances, asset information, block data, and other on-chain state without requiring WebSocket connections or SDK integrations. It replaces the deprecated Substrate API Sidecar and serves the same endpoints under the `/v1` prefix.

This guide demonstrates how to query on-chain storage with `curl` against a Polkadot REST API instance that you run yourself, connected to a local development node.

## Prerequisites

- [curl](https://curl.se/) or any HTTP client.
- A local development node listening on `{{ networks.local.ws_url }}`, for example the parachain template node described in [Run the Node Locally](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md#run-the-node-locally).
- For the asset queries, a runtime that includes the [Assets pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_assets/index.html) and at least one asset.

## Run Polkadot REST API Locally

Install and start Polkadot REST API. For more installation and configuration options, see the [Polkadot REST API](/tools/polkadot-rest-api.md) page.

=== "Cargo"

    ```bash
    # Install the binary
    cargo install --locked polkadot-rest-api@{{ dependencies.crates.polkadot_rest_api.version }}

    # Run with default settings (connects to ws://127.0.0.1:9944)
    polkadot-rest-api
    ```

    To connect to a node on a different endpoint, set the `SAS_SUBSTRATE_URL` environment variable:

    ```bash
    SAS_SUBSTRATE_URL=INSERT_NODE_WS_URL polkadot-rest-api
    ```

=== "Docker"

    ```bash
    docker run --rm -p 8080:8080 \
      -e SAS_SUBSTRATE_URL=INSERT_NODE_WS_URL \
      docker.io/paritytech/polkadot-rest-api:v{{ dependencies.crates.polkadot_rest_api.version }}
    ```

    Replace `INSERT_NODE_WS_URL` with a WebSocket endpoint of your node that is reachable from inside the container.

Once running, Polkadot REST API serves requests at `http://127.0.0.1:8080`, with all endpoints under the `/v1` prefix. All examples on this page use that address. The sample responses are illustrative, and the values you see depend on your chain.

## Query Account Balance

The `/v1/accounts/{accountId}/balance-info` endpoint returns an account's native token balance, including free, reserved, and frozen amounts.

**Request**

```bash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/balance-info"
```

**Response**

--8<-- "code/chain-interactions/query-data/query-rest/balance-info-response.html"

**Response fields**

| Field | Description |
|-------|-------------|
| `at.hash` | Block hash at which the query was executed |
| `at.height` | Block number at which the query was executed |
| `nonce` | Number of transactions sent from this account |
| `tokenSymbol` | Native token symbol of the chain |
| `free` | Balance available for transfers (in the smallest unit) |
| `reserved` | Balance locked for on-chain activities |
| `miscFrozen` | Legacy field. On runtimes that use `frozen`, contains a message stating that the field does not exist for this runtime |
| `feeFrozen` | Legacy field. On runtimes that use `frozen`, contains a message stating that the field does not exist for this runtime |
| `frozen` | Balance frozen and unavailable for transfers |
| `transferable` | Actual balance available to transfer |
| `locks` | Array of balance locks with their reasons |

**Query at a specific block**

You can query the balance at a specific block height or hash using the `at` query parameter:

```bash
# Query at block height
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/balance-info?at=INSERT_BLOCK_NUMBER"

# Query at block hash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/balance-info?at=INSERT_BLOCK_HASH"
```

## Query Asset Balances

The `/v1/accounts/{accountId}/asset-balances` endpoint returns an account's balances for assets managed by the Assets pallet. By default, assets with a zero balance are left out of the response. To include them, add the `showEmpty=true` query parameter.

**Request all asset balances**

```bash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/asset-balances"
```

**Request a specific asset balance**

To query a specific asset, provide the asset ID as a query parameter:

```bash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/asset-balances?assets[]=INSERT_ASSET_ID"
```

**Response**

--8<-- "code/chain-interactions/query-data/query-rest/asset-balances-response.html"

**Response fields**

| Field | Description |
|-------|-------------|
| `at.hash` | Block hash at which the query was executed |
| `at.height` | Block number at which the query was executed |
| `assetId` | Unique identifier for the asset |
| `balance` | Account's balance of this asset (in smallest unit) |
| `isFrozen` | Whether the account's asset balance is frozen |
| `isSufficient` | Whether this account's existence is being paid for by this asset balance (per-account flag, not the asset's global sufficiency setting) |

!!! note
    The `isSufficient` field in the asset balance response is a per-account flag. To check if an asset is configured as a sufficient asset (can pay for account existence), query the [Asset Details](#query-asset-details) endpoint and check the `isSufficient` field there.

**Query multiple assets**

You can query multiple assets in a single request:

```bash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/asset-balances?assets[]=INSERT_ASSET_ID_1&assets[]=INSERT_ASSET_ID_2"
```

## Query Asset Metadata

Use the pallet storage endpoint to query asset metadata like name, symbol, and decimals.

**Request**

```bash
curl -s "http://127.0.0.1:8080/v1/pallets/assets/storage/Metadata?keys[]=INSERT_ASSET_ID"
```

**Response**

--8<-- "code/chain-interactions/query-data/query-rest/asset-metadata-response.html"

**Response fields**

| Field | Description |
|-------|-------------|
| `at.hash` | Block hash at which the query was executed |
| `at.height` | Block number at which the query was executed |
| `pallet` | Name of the pallet containing the storage item |
| `palletIndex` | Numeric index of the pallet in the runtime |
| `storageItem` | Name of the storage item being queried |
| `keys` | Array of keys used to query the storage map |
| `value.deposit` | Deposit held for storing this metadata |
| `value.name` | Asset name (hex-encoded string) |
| `value.symbol` | Asset symbol (hex-encoded string) |
| `value.decimals` | Number of decimal places for display |
| `value.isFrozen` | Whether the asset metadata is frozen |

The `name` and `symbol` fields are returned as hex-encoded UTF-8 strings. In the sample response, `0x4578616d706c6520546f6b656e` decodes to "Example Token" and `0x455854` decodes to "EXT."

## Query Asset Details

Query the asset configuration, including owner, supply, and account count:

**Request**

```bash
curl -s "http://127.0.0.1:8080/v1/pallets/assets/storage/Asset?keys[]=INSERT_ASSET_ID"
```

**Response**

--8<-- "code/chain-interactions/query-data/query-rest/asset-details-response.html"

**Response fields**

| Field | Description |
|-------|-------------|
| `at.hash` | Block hash at which the query was executed |
| `at.height` | Block number at which the query was executed |
| `pallet` | Name of the pallet containing the storage item |
| `palletIndex` | Numeric index of the pallet in the runtime |
| `storageItem` | Name of the storage item being queried |
| `keys` | Array of keys used to query the storage map |
| `value.owner` | Account that owns the asset |
| `value.issuer` | Account authorized to mint new tokens |
| `value.admin` | Account with administrative privileges |
| `value.freezer` | Account that can freeze balances |
| `value.supply` | Total supply of the asset (in smallest unit) |
| `value.minBalance` | Minimum balance required to hold this asset |
| `value.isSufficient` | Whether this asset can pay for account existence (accounts holding only this asset don't need native tokens) |
| `value.accounts` | Number of accounts holding this asset |
| `value.sufficients` | Number of accounts whose existence is paid for by this asset |
| `value.status` | Asset status (Live, Frozen, or Destroying) |

## Query Foreign Asset Balances

For cross-chain assets (foreign assets) on chains that support them, use the `/v1/accounts/{accountId}/foreign-asset-balances` endpoint:

```bash
curl -s "http://127.0.0.1:8080/v1/accounts/INSERT_ADDRESS/foreign-asset-balances"
```

## Query Block Information

The `/v1/blocks/{blockId}` endpoint returns detailed block information, including extrinsics and events.

**Request latest block**

```bash
curl -s "http://127.0.0.1:8080/v1/blocks/head"
```

**Request specific block**

```bash
# By block number
curl -s "http://127.0.0.1:8080/v1/blocks/INSERT_BLOCK_NUMBER"

# By block hash
curl -s "http://127.0.0.1:8080/v1/blocks/INSERT_BLOCK_HASH"
```

**Response**

--8<-- "code/chain-interactions/query-data/query-rest/block-response.html"

## API Reference

For a complete list of endpoints and parameters, open `http://127.0.0.1:8080/docs` on your running instance, or see the published [Polkadot REST API documentation](https://paritytech.github.io/polkadot-rest-api/).

## Where to Go Next

Now that you understand how to query on-chain state with the REST API, explore these related topics:

<div class="grid cards" markdown>

-   __Query with SDKs__

    ---

    Use the Subxt Rust library for programmatic, type-safe access.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/query-sdks.md)

-   __Runtime API Calls__

    ---

    Learn how to execute runtime APIs for specialized queries.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/query-data/runtime-api-calls.md)

-   __Send Transactions__

    ---

    Learn to construct and submit transactions.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/with-sdks.md)

</div>
