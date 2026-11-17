---
title: Chain Interactions Overview
description: Query data, send transactions, store data, and manage accounts on any Polkadot SDK-based chain with Parity's client libraries, Subxt first.
categories: Chain Interactions
---

# Chain Interactions

## Introduction

This section shows how to interact with any Polkadot SDK-based chain using Parity's client libraries. [Subxt](/tools/subxt.md) is the main path throughout: a Rust library that generates type-safe interfaces from chain metadata. The [Polkadot REST API](/tools/polkadot-rest-api.md) provides a REST alternative for reading chain data.

The guides run against a local development node by default, so you can follow them without access to a live network. Point the same code at any other Polkadot SDK-based chain by changing the endpoint.

You'll learn how to:

- Query on-chain state and call runtime APIs.
- Send transactions, estimate their fees, and pay fees in non-native assets.
- Estimate the fees of cross-chain messages sent with XCM.
- Store and retrieve data on the Bulletin Chain.
- Create accounts programmatically.

## Core Interaction Patterns

### Query On-Chain Data

Accessing blockchain state is fundamental to building responsive applications. Polkadot SDK-based chains offer several ways to query on-chain data, each suited for different use cases.

<!-- INDEX TABLE START
dir: query-data
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

### Send Transactions

Transactions are the primary mechanism for modifying blockchain state. These guides cover transaction construction, signing, submission, and fees.

<!-- INDEX TABLE START
dir: send-transactions
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

### Send Cross-Chain Transactions

Polkadot SDK-based chains communicate through Cross-Consensus Messaging (XCM), which lets chains exchange messages and transfer assets.

<!-- INDEX TABLE START
dir: send-transactions/interoperability
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

### Store Data

The Bulletin Chain runtime provides content-addressed storage with IPFS-compatible CIDs.

<!-- INDEX TABLE START
dir: store-data
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

### Manage Accounts

Accounts are the basis of user identity and transaction signing in blockchain applications.

<!-- INDEX TABLE START
dir: accounts
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

### Manage Tokens

Guides for registering, converting, and transferring assets on live networks are available on [developers.polkadot.com](https://developers.polkadot.com/chain-interactions/token-operations/register-local-asset/).

## Parity Client Libraries

| Tool | Description |
|------|-------------|
| [Subxt](/tools/subxt.md) | Rust library for type-safe interaction with Polkadot SDK-based chains: storage queries, transactions, runtime APIs, and events. |
| [`subxt-signer`](https://docs.rs/subxt-signer/{{ dependencies.crates.subxt_signer.version }}/subxt_signer/) | Rust crate for deriving keys and signing transactions, including the development accounts used in local examples. |
| [Polkadot REST API](/tools/polkadot-rest-api.md) | REST service that exposes chain data from a Polkadot SDK-based node over HTTP. |
| [Light clients](/tools/light-clients.md) | smoldot and Substrate Connect, for connecting to chains without trusting a full node. |

The same patterns work with third-party client libraries, for example [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/), [Dedot](https://developers.polkadot.com/reference/tools/dedot/), [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/), and [Python Substrate Interface](https://developers.polkadot.com/reference/tools/py-substrate-interface/).
