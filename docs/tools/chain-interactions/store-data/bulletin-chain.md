---
title: Store and Retrieve Data on the Bulletin Chain
description: Learn how to store an image on a Bulletin Chain node, obtain its CID, retrieve it over IPFS, and renew it before it expires.
categories: Chain Interactions
page_badges:
  tutorial_badge: Intermediate
---

# Store and Retrieve Data on the Bulletin Chain

## Introduction

The [Bulletin Chain](/apps/reference/infrastructure/bulletin-chain/index.md) is a Polkadot SDK runtime, developed by Parity in the [`paritytech/polkadot-bulletin-chain`](https://github.com/paritytech/polkadot-bulletin-chain) repository, that provides data storage with IPFS-compatible content addressing through its `TransactionStorage` and `DataRenewal` pallets. You can use it to store static sites, images, media assets, application data, or any other files that benefit from availability and censorship-resistance.

In this tutorial, you walk through a common developer scenario: storing an image on the Bulletin Chain and obtaining its CID (Content Identifier) so it can be referenced from a dApp, NFT metadata, or any IPFS-compatible system. The same steps apply to any file type, including documents, JSON configs, and HTML pages.

The endpoint is configuration. The steps work against a Bulletin Chain node you run locally or against any deployment you have access to. For TestNet access, see [Get TestNet Tokens](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/) on developers.polkadot.com.

## Prerequisites

- [Node.js](https://nodejs.org/) v18 or higher.
- The WebSocket endpoint of a Bulletin Chain node.
- An account authorized to store data on that chain, as described in the next section.
- An image or file to store, at most 2 MiB per transaction. For larger files, you can chunk your data. For details, see [Size Limits](/apps/reference/infrastructure/bulletin-chain/data-storage.md#size-limits).

## Get Authorization

The Bulletin Chain has no token balances. Instead, an account needs an [authorization](/apps/reference/infrastructure/bulletin-chain/authorization.md) before it can store data. An authorization grants your account a specific number of transactions and bytes that you can use for storage, and it expires at a set block.

The `authorize_account(who, transactions, bytes)` extrinsic of the `TransactionStorage` pallet grants authorizations. It requires the authorizer origin: Root, or an account or parachain allowed to grant authorizations. You cannot self-authorize. For details, see [Who Grants Authorization](/apps/reference/infrastructure/bulletin-chain/data-storage.md#who-grants-authorization).

- **Local development chain**: The `//Alice` account has sudo access and can grant authorizations. Submit `authorize_account` for your account through the `Sudo` pallet with any client that supports Polkadot SDK chains.
- **TestNet**: See [Get TestNet Tokens](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/) on developers.polkadot.com.

!!! note
    Once an authorization expires, unused allowance is not refunded, and you need a new authorization.

## Store Your Image

Now that your account is authorized, you can store your image on-chain. This tutorial uses [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/), a third-party TypeScript library, to submit extrinsics from the `TransactionStorage` pallet. You can also submit the same extrinsics with other clients that support Polkadot SDK chains, for example Polkadot.js Apps.

### Set Up the Project

1. Create and initialize a new project:

    ```bash
    mkdir bulletin-store-example && cd bulletin-store-example && \
    npm init -y && npm pkg set type=module
    ```

2. Install the required dependencies:

    ```bash
    npm install polkadot-api@{{dependencies.javascript_packages.polkadot_api.version}} @polkadot-labs/hdkd@{{dependencies.javascript_packages.hdkd.version}} @polkadot-labs/hdkd-helpers@{{dependencies.javascript_packages.hdkd_helpers.version}} multiformats
    ```

3. Fetch the Bulletin Chain metadata and generate typed descriptors:

    ```bash
    npx papi add bulletin -w INSERT_BULLETIN_WS_ENDPOINT
    ```

    Replace `INSERT_BULLETIN_WS_ENDPOINT` with the WebSocket endpoint of your Bulletin Chain node, for example `{{ networks.local.ws_url }}` for a node running locally on the default port. This command downloads the chain metadata and generates typed descriptors for all pallet interactions.

### Store an Image

Create a file named `store-data.ts` with the following content:

```typescript title="store-data.ts"
--8<-- "code/chain-interactions/store-data/bulletin-chain/store-data.ts"
```

Replace the following placeholders:

- `INSERT_BULLETIN_WS_ENDPOINT`: The same WebSocket endpoint you used to generate the descriptors.
- `INSERT_IPFS_GATEWAY_URL`: The base URL of an IPFS HTTP gateway that serves data from your Bulletin Chain deployment, if one exists. The script only prints this URL.
- `INSERT_IMAGE_PATH`: The path to your image, for example `./logo.png`. Ensure the file is at most 2 MiB.

The script signs with the `//Alice` development account. To use a different authorized account, derive the signer from that account's mnemonic instead of `DEV_PHRASE`.

Run the script:

```bash
npx tsx store-data.ts
```

You should see output similar to:

--8<-- 'code/chain-interactions/store-data/bulletin-chain/store-data-ts.html'

Save the CID and the content hash. You need them to retrieve and renew your data.

## Verify Your Stored Image

The script prints the block in which your transaction was included and the data from the `TransactionStorage.Stored` event: the index of the transaction in the block, the content hash, and the CID. To verify the stored image independently, look up that block with a block explorer or any client connected to your node and check for the `TransactionStorage.Stored` event.

## Retrieve Your Data

The Bulletin Chain follows a "write-to-chain, read-from-network" architecture: you retrieve data from collator nodes using the CID. For a full overview of retrieval methods, see [IPFS Integration and Data Retrieval](/apps/reference/infrastructure/bulletin-chain/data-storage.md#ipfs-integration-and-data-retrieval).

=== "Direct P2P (Helia)"

    The recommended decentralized approach is to connect directly to Bulletin Chain collator nodes using [Helia](https://helia.io/) (a lean IPFS implementation) and retrieve data by CID over [libp2p](https://libp2p.io/).

    This method requires the multiaddresses of the collator nodes you connect to. See the [Bulletin Chain repository](https://github.com/paritytech/polkadot-bulletin-chain) for Helia configuration examples.

=== "IPFS Gateway"

    Where an IPFS HTTP gateway that fetches from Bulletin Chain collator nodes is deployed, it serves data at `<gateway>/ipfs/<CID>`. You can open that URL in a browser or fetch it programmatically:

    ```typescript
    const gateway = 'INSERT_IPFS_GATEWAY_URL';
    const cid = 'INSERT_CID';
    const response = await fetch(`${gateway}/ipfs/${cid}`);
    const data = await response.arrayBuffer();
    console.log(`Retrieved ${data.byteLength} bytes`);
    ```

    Replace `INSERT_IPFS_GATEWAY_URL` with the gateway's base URL and `INSERT_CID` with the CID string you received when storing your data (for example, `bafk2bzacea6wlxy...`).

!!! warning "Provisional"
    Retrieval through the Smoldot light client with the `bitswap_block` RPC is planned but not yet available.

!!! note
    Stored data is only available within the retention period set by the runtime's `RetentionPeriod` parameter. After that, the data is pruned from the chain and is no longer retrievable unless it has been renewed.

## Renew Your Data

If you need your image to remain available beyond the retention period, renew it before it expires. Register a renewal with the `DataRenewal` pallet, identifying your data by the content hash that the store script printed:

```typescript
console.log('Registering renewal...');
const result = await api.tx.DataRenewal.renew({
  entry: { type: 'ContentHash', value: 'INSERT_CONTENT_HASH' },
}).createAndSubmit(signer);
console.log(`Renewal registered in block: ${result.block.hash}`);
```

You should see output similar to:

--8<-- 'code/chain-interactions/store-data/bulletin-chain/renew-data-ts.html'

The `renew` call registers a one-time renewal. When the retention period of your data ends, the chain stores the data again, so it stays available for another full retention period. The renewal is paid from your authorization when you register it. Each content hash can have only one registered renewal at a time, so registering again before this one takes effect fails with `RenewalAlreadyEnabled`.

The `DataRenewal` pallet also provides the following alternatives:

- **`force_renew`**: Renews the data immediately. It takes the same `entry` argument as `renew`.
- **`enable_auto_renew`**: Renews the data at the end of every retention period. It takes the data's content hash. Each renewal after the first is charged to your authorization, and recurring renewal stops if your authorization cannot cover it.

!!! warning
    You can also identify the data by its position, using the block number and the `Index` value that the store script printed:

    ```typescript
    entry: {
      type: 'Position',
      value: { block: INSERT_BLOCK_NUMBER, index: INSERT_INDEX },
    },
    ```

    A position identifies one stored copy of the data and stays valid until that copy's retention period ends. Each renewal stores a new copy at a new block number and index. When a `renew` takes effect, the original copy expires at the same time, and calls that use its position fail with `RenewedNotFound`. After `force_renew`, the original copy keeps its position until its own period ends, so renewing that position again stores another copy. The content hash always resolves to the latest copy.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge reference">Reference</span> __Bulletin Chain Reference__

    ---

    Explore the Bulletin Chain runtime, authorization model, and renewal lifecycle.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/bulletin-chain/index.md)

-   <span class="badge reference">Reference</span> __Data Storage Reference__

    ---

    Explore the full technical reference for the Bulletin Chain, including all extrinsics, storage items, and events.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/bulletin-chain/data-storage.md)

</div>
