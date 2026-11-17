---
title: Bulletin Chain Data Storage
description: Pallet-level reference for the Bulletin Chain runtime, covering the TransactionStorage and DataRenewal pallets, authorizer origins, IPFS retrieval, and size limits.
categories: Apps, Reference
---

# Data Storage

## Introduction

This page is the pallet-level reference for the Bulletin Chain runtime: the extrinsics, storage items, and events of the `TransactionStorage` and `DataRenewal` pallets, how authorizations are granted, how data is retrieved over IPFS, and the size limits that apply to uploads. For the conceptual model (content addressing, permissionless reads, authorization-gated writes, and time-bound retention), start with the [Bulletin Chain overview](/apps/reference/infrastructure/bulletin-chain/index.md).

The runtime and pallets are developed in the [`paritytech/polkadot-bulletin-chain`](https://github.com/paritytech/polkadot-bulletin-chain) repository, which is planned to move to the [fellowship runtimes repository](https://github.com/polkadot-fellows/runtimes).

## Authorization

The Bulletin Chain has no token balances and charges no fees. Storage access is controlled through authorizations, which prevent spam and ensure fair usage. An authorization is a permission entry in the chain's storage, not a token transfer. It specifies:

- The account allowed to store data.
- The number of transactions and bytes the account can use.
- An expiration block after which the authorization becomes invalid, set a fixed number of blocks after the authorization is granted. The number of blocks is part of the runtime configuration.

Each storage transaction decrements the remaining transaction count and byte allowance. Once the allocation is used up or the expiration block is reached, the account needs a new authorization. See [Authorization](/apps/reference/infrastructure/bulletin-chain/authorization.md) for the full model.

### Who Grants Authorization

The `authorize_account` and `authorize_preimage` extrinsics require the _authorizer_ origin. The authorizer origin accepts Root, sibling parachains that are allowed to manage authorizations, and accounts that Root registers with `add_authorizer`. Each registered account has a budget that limits how much authorization it can grant.

On a local development chain, the `//Alice` account has sudo access and can grant authorizations. To get an authorization on TestNet, see [Get TestNet Tokens](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/).

### Account Authorization

Account authorization grants a specific account the ability to store any data on-chain, up to the allocated transaction and byte limits. This is the most common authorization type for active users.

- Granted with the `authorize_account` extrinsic (authorizer origin).
- Refreshed with `refresh_account_authorization`.
- Cleaned up after expiry with `remove_expired_account_authorization`.

### Preimage Authorization

Preimage authorization grants storage access to anyone who can provide the preimage of a specific hash. This is useful for sponsored uploads, where a third party authorizes the storage of specific, known content without needing to know which account will submit it.

- Granted with the `authorize_preimage` extrinsic (authorizer origin).
- Refreshed with `refresh_preimage_authorization`.
- Cleaned up after expiry with `remove_expired_preimage_authorization`.

### Capacity Planning

When requesting authorization, estimate how many transactions and bytes you need. A single file of up to 2 MiB requires one transaction. For chunked uploads (files over 2 MiB), each chunk requires a separate transaction, plus one additional transaction for the manifest. With the Bulletin SDKs' default chunk size of 1 MiB, a 10 MiB file requires 11 transactions. See [Chunked Uploads](/apps/reference/infrastructure/bulletin-chain/chunked-upload.md).

## Transaction Storage Pallet

The `TransactionStorage` pallet is the core module for storing data on the Bulletin Chain. It provides the following extrinsics:

| Extrinsic | Description | Origin |
|:--:|:--:|:--:|
| `store(data)` | Store data on-chain. Emits `Stored` with the transaction index, content hash, and CID | Authorized account, Root, or unsigned with a matching preimage authorization |
| `store_with_cid_config(cid, data)` | Store data with an explicit CID configuration (codec and hashing algorithm) | Authorized account, Root, or unsigned with a matching preimage authorization |
| `authorize_account(who, transactions, bytes)` | Authorize an account to store up to `bytes` bytes in `transactions` transactions. Calling it again before the authorization expires adds to the existing allowance | Authorizer |
| `authorize_preimage(content_hash, max_size)` | Authorize anyone to store data with the given content hash, up to `max_size` bytes | Authorizer |
| `refresh_account_authorization(who)` | Extend the expiration of an existing account authorization. The allowance is unchanged | Authorizer |
| `refresh_preimage_authorization(content_hash)` | Extend the expiration of an existing preimage authorization. The allowance is unchanged | Authorizer |
| `remove_expired_account_authorization(who)` | Remove an expired account authorization from storage | Any account |
| `remove_expired_preimage_authorization(content_hash)` | Remove an expired preimage authorization from storage | Any account |
| `add_authorizer(who, budget)` | Allow an account to grant authorizations, within the given budget | Root |
| `remove_authorizer(who)` | Remove an account from the allowed authorizers | Root |
| `remove_exhausted_authorizer(who)` | Remove an authorizer whose budget is used up or whose validity period has ended | Any account |
| `apply_block_inherents(proof)` | Submit the block's storage proof | None (inherent) |

The pallet exposes the following storage items:

| Storage Item | Type | Description |
|:--:|:--:|:--:|
| `Authorizations` | `Map<AuthorizationScope, Authorization>` | Active authorizations, keyed by account (`Account`) or content hash (`Preimage`) |
| `AllowedAuthorizers` | `Map<AccountId, AuthorizerBudget>` | Accounts allowed to grant authorizations, with their remaining budget |
| `Transactions` | `Map<BlockNumber, Vec<TransactionInfo>>` | Metadata of the transactions stored in each block |
| `TransactionByContentHash` | `Map<Hash, (BlockNumber, u32)>` | Most recent block number and transaction index for each content hash. Updated on every renewal |
| `BlockTransactions` | `Vec<TransactionInfo>` | Transactions stored in the current block |
| `ProofChecked` | `bool` | Whether the storage proof was checked in the current block |
| `ByteFee` | `Balance` | The fee charged per byte of stored data |
| `EntryFee` | `Balance` | The base fee charged per storage transaction |
| `RetentionPeriod` | `BlockNumber` | The number of blocks for which stored data is retained |

The pallet emits the following events:

| Event | Description |
|:--:|:--:|
| `Stored` | Emitted when data is stored. Contains the transaction index, content hash, and CID |
| `ProofChecked` | Emitted when a storage proof is successfully verified |
| `AccountAuthorized` | Emitted when an account is granted storage authorization |
| `AccountAuthorizationRefreshed` | Emitted when an account authorization is refreshed |
| `PreimageAuthorized` | Emitted when a content hash is granted storage authorization |
| `PreimageAuthorizationRefreshed` | Emitted when a preimage authorization is refreshed |
| `ExpiredAccountAuthorizationRemoved` | Emitted when an expired account authorization is removed |
| `ExpiredPreimageAuthorizationRemoved` | Emitted when an expired preimage authorization is removed |
| `AuthorizerAdded` | Emitted when an account is added to the allowed authorizers |
| `AuthorizerRemoved` | Emitted when Root removes an account from the allowed authorizers |
| `ExhaustedAuthorizerRemoved` | Emitted when an authorizer is removed because its budget is used up or its validity period has ended |

## Data Renewal Pallet

Stored data is removed when its retention period ends. The `DataRenewal` pallet extends the retention of data stored through the `TransactionStorage` pallet. Renewal calls identify the data in one of two ways:

- **`Position { block, index }`**: The block number and transaction index of the most recent store or renewal of the data.
- **`ContentHash(hash)`**: The content hash of the data. The chain maps each content hash to its most recent position, so this form keeps working after the data is renewed.

The pallet provides the following extrinsics:

| Extrinsic | Description | Origin |
|:--:|:--:|:--:|
| `renew(entry)` | Schedule a one-time renewal. It takes effect when the retention period ends, and the registration is then removed. The renewal is paid from your authorization when you register it | Authorized account |
| `force_renew(entry)` | Renew the data immediately | Authorized account, Root, or unsigned with a matching preimage authorization |
| `enable_auto_renew(content_hash)` | Renew the data at the end of every retention period. The first renewal is paid when you register; later renewals are charged to your authorization, and the registration is dropped if your authorization cannot cover one | Authorized account |
| `disable_auto_renew(content_hash)` | Cancel recurring renewal. The owner can disable it only after the prepaid first renewal has taken effect | Authorized account that owns the registration, or Root |
| `process_pending_renewals()` | Process the renewals due in the current block | None (inherent) |

Each content hash can have only one registered renewal at a time, across all accounts: `renew` and `enable_auto_renew` fail with `RenewalAlreadyEnabled` while a registration exists.

Renewed bytes are tracked separately from stored bytes. A renewal is rejected when the bytes already renewed under the authorization, plus the size of the data, would exceed the authorization's byte allowance (`InvalidTransaction::Custom(5)`), or when it would exceed the chain-wide cap on renewed storage, `MaxPermanentStorageSize` (`InvalidTransaction::Custom(6)`). To check whether a renewal would be accepted, call the `BulletinTransactionStorageApi.can_renew(account, entry)` runtime API.

The pallet exposes the following storage items:

| Storage Item | Type | Description |
|:--:|:--:|:--:|
| `Renewals` | `Map<Hash, RenewalData>` | Renewal registrations, keyed by content hash |
| `PendingRenewals` | `Vec<(Hash, TransactionInfo, RenewalData)>` | Renewals to process in the current block |
| `PermanentStorageUsed` | `u64` | Total bytes of renewed data on chain, across all accounts |

The pallet emits the following events:

| Event | Description |
|:--:|:--:|
| `Renewed` | Emitted when data is renewed immediately with `force_renew`. Contains the new transaction index and the content hash |
| `RenewalEnabled` | Emitted when a renewal is registered. `recurring` is `true` for `enable_auto_renew` |
| `AutoRenewalDisabled` | Emitted when recurring renewal is disabled |
| `DataRenewed` | Emitted when a registered renewal takes effect. Contains the new transaction index, the content hash, and the account |
| `RenewalFailed` | Emitted when a registered renewal cannot be paid from the account's authorization. The registration is dropped and the data expires |
| `PermanentStorageUsedUpdated` | Emitted when the total of renewed storage changes |
| `PermanentStorageNearCap` | Emitted when the total of renewed storage crosses 80% of `MaxPermanentStorageSize` |

For the renewal lifecycle and its costs, see [Renewal](/apps/reference/infrastructure/bulletin-chain/renewal.md).

## IPFS Integration and Data Retrieval

The Bulletin Chain follows a "write-to-chain, read-from-network" architecture. Data is stored through on-chain transactions and retrieved directly from collator nodes using the CID. Bulletin Chain nodes are themselves participants in the [InterPlanetary File System (IPFS)](https://ipfs.tech/) network:

1. **CID generation**: The chain computes an IPFS-compatible [Content Identifier (CID)](https://docs.ipfs.tech/concepts/content-addressing/) from your data using a cryptographic hash. This CID uniquely identifies the content regardless of where it is stored.
2. **Bitswap serving**: Bulletin Chain collator nodes implement the IPFS [Bitswap](https://docs.ipfs.tech/concepts/bitswap/) wire protocol, the standard way IPFS nodes exchange data blocks. IPFS clients can request data directly from collator nodes using the CID. For the node flags that enable this, see [Storage and IPFS Flags](/apps/reference/infrastructure/bulletin-chain/run-a-node.md#storage-and-ipfs-flags).

The following retrieval methods are available:

| Method | Status | Description |
|:--:|:--:|:--:|
| Direct P2P ([Helia](https://helia.io/)) | Available | Connect to collator nodes through [libp2p](https://libp2p.io/) and fetch data using the CID. The recommended decentralized approach for production applications |
| IPFS HTTP gateway | Available where deployed | A gateway that fetches from Bulletin Chain collator nodes serves data at `<gateway>/ipfs/<CID>`, which can also be used programmatically with `fetch()` |
| Smoldot light client | Planned | Fully decentralized retrieval through the `bitswap_block` RPC, enabling trustless data access without connecting to a full node |

Inside a Host, the Product SDK reads data by CID through the Host instead; see [Store Data on Chain](/apps/build/store-data-on-chain.md).

!!! warning
    Generic public IPFS gateways like `ipfs.io` or `cloudflare-ipfs.com` are deprecated for Bulletin Chain retrieval. They do not reliably serve Bulletin Chain data and introduce external single points of failure.

## Data Lifecycle

The typical lifecycle of data on the Bulletin Chain follows these steps:

1. **Authorization**: Obtain an authorization from an account or origin that holds the authorizer role. See [Who Grants Authorization](#who-grants-authorization).
2. **Storage**: Submit a `store` or `store_with_cid_config` transaction with your data payload.
3. **Retrieval**: Fetch data from collator nodes by CID through Direct P2P (Helia), an IPFS HTTP gateway, or a Host.
4. **Renewal**: Before the retention period ends, register a renewal with the `DataRenewal` pallet: `renew` for a one-time renewal or `enable_auto_renew` for recurring renewal. Use `force_renew` to renew immediately.

## Size Limits

| Parameter | Value |
|:---:|:--:|
| Max transaction size | 2 MiB |
| Default chunk size (Bulletin SDKs) | 1 MiB |
| Max file size per chunking operation (Bulletin SDKs) | 64 MiB |
| Retention period | Set by the runtime's `RetentionPeriod` storage item |

For files larger than the max transaction size, data must be split into chunks using the [DAG-PB (UnixFS)](https://ipld.io/specs/codecs/dag-pb/) standard. Each chunk is stored as a separate transaction. A final manifest transaction is uploaded last, containing the CIDs of all chunks. The manifest's CID becomes the root CID for the complete file. IPFS clients can then retrieve and reassemble the file automatically using the manifest.

The Bulletin SDKs chunk at most 64 MiB in a single operation. To store a larger file, split it into segments of at most 64 MiB and chunk each segment separately.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Store Data on Chain__

    ---

    The Product-side how-to: setting up the storage client, storing and retrieving data, larger files, and renewal.

    [:octicons-arrow-right-24: Get Started](/apps/build/store-data-on-chain.md)

-   <span class="badge tutorial">Tutorial</span> __Store and Retrieve Data on the Bulletin Chain__

    ---

    Authorize an account, store an image, retrieve it, and manage renewals with PAPI.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/store-data/bulletin-chain.md)

-   <span class="badge guide">Guide</span> __Run a Bulletin Chain Node__

    ---

    Run a local Bulletin Chain dev node or network to store and retrieve data against.

    [:octicons-arrow-right-24: Get Started](/apps/reference/infrastructure/bulletin-chain/run-a-node.md)

</div>
