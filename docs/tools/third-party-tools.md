---
title: "Works With: Third-Party Tools"
description: Community and third-party libraries and tools that work with Parity software, from JavaScript and Python clients to chain forking, XCM, and Ethereum tooling.
categories: Tooling
---

# Works With: Third-Party Tools

## Introduction

Parity's software exposes standard interfaces: Polkadot SDK nodes serve JSON-RPC and metadata, smoldot serves the same JSON-RPC interface as a light client, and the [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter.md) lets Ethereum tooling talk to a chain running `pallet-revive`. Many tools that Parity does not write build on those interfaces.

This page lists commonly used ones. Parity does not maintain them, so check each project's own documentation for versions and support. Most have a reference page in the [Polkadot Developer Docs](https://developers.polkadot.com/), maintained by the Polkadot Community Foundation.

## Client Libraries

These libraries connect to any Polkadot SDK-based chain through its JSON-RPC interface. They are alternatives or complements to [Subxt](/tools/subxt.md), Parity's Rust client.

| Tool | Language | Description |
|:-----|:---------|:------------|
| [Polkadot-API (PAPI)](https://developers.polkadot.com/reference/tools/papi/) | TypeScript | A modular set of libraries designed around light clients. It can connect through [smoldot](/tools/light-clients.md) or a WebSocket provider. |
| [Dedot](https://developers.polkadot.com/reference/tools/dedot/) | TypeScript | A lightweight, tree-shakable client with TypeScript types generated from chain metadata. |
| [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/) | JavaScript, TypeScript | A long-standing client library for querying chain data and submitting transactions. |
| [Python Substrate Interface](https://developers.polkadot.com/reference/tools/py-substrate-interface/) | Python | A library for connecting to nodes, querying storage, and submitting extrinsics from Python. |
| [Polkadart](https://developers.polkadot.com/reference/tools/polkadart/) | Dart | A type-safe Dart SDK for Polkadot SDK-based chains. |

## Testing and Development

These tools complement [Zombienet](/tools/zombienet.md) and the [Polkadot SDK testing guides](/polkadot-sdk/testing/run-a-parachain-network.md).

| Tool | Description |
|:-----|:------------|
| [Chopsticks](https://developers.polkadot.com/reference/tools/chopsticks/) | Forks a live Polkadot SDK-based chain locally, replays blocks, and simulates XCM between forked chains. Useful together with [try-runtime](/polkadot-sdk/runtime-maintenance/test-runtime-upgrades.md) when testing upgrades against real state. |
| [Moonwall](https://developers.polkadot.com/reference/tools/moonwall/) | An end-to-end testing framework that manages test environments and network configurations. |
| [Pop CLI](https://developers.polkadot.com/reference/tools/pop-cli/) | Scaffolds parachain and contract projects and launches local networks. |

## Cross-Chain Messaging

| Tool | Description |
|:-----|:------------|
| [ParaSpell XCM SDK](https://developers.polkadot.com/reference/tools/paraspell/) | Builds XCM transfers between chains from a high-level description, without writing XCM programs by hand. |
| [XCM tools overview](https://developers.polkadot.com/reference/tools/xcm-tools/) | A survey of libraries and utilities for building, sending, and testing XCM. |

To configure XCM in your own runtime and test it locally, see [Configure XCM](/polkadot-sdk/interoperability/configure-xcm.md) and [Test XCM](/polkadot-sdk/testing/test-xcm.md).

## Smart Contracts

Ethereum development environments and libraries work with a chain running `pallet-revive` through the [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter.md). Solidity compiled for the PVM backend goes through [resolc](/smart-contracts/resolc.md), which the [Hardhat Polkadot plugin](/smart-contracts/dev-environments/hardhat-polkadot.md) integrates with Hardhat.

| Tool | Description |
|:-----|:------------|
| [Hardhat](https://developers.polkadot.com/smart-contracts/dev-environments/hardhat/) | An Ethereum development environment for compiling, testing, and deploying contracts. |
| [Foundry](https://developers.polkadot.com/smart-contracts/dev-environments/foundry/) | A Rust-based Ethereum toolkit for building, testing, and deploying contracts. |
| [Remix IDE](https://developers.polkadot.com/smart-contracts/dev-environments/remix/) | A browser-based IDE for writing and deploying Solidity contracts. |
| [ethers.js](https://developers.polkadot.com/smart-contracts/libraries/ethers-js/), [viem](https://developers.polkadot.com/smart-contracts/libraries/viem/), [Wagmi](https://developers.polkadot.com/smart-contracts/libraries/wagmi/) | JavaScript and TypeScript libraries for interacting with contracts and accounts. |
| [Web3.py](https://developers.polkadot.com/smart-contracts/libraries/web3-py/) | A Python library for interacting with contracts and accounts. |
| [ink!](https://use.ink/) | A Rust language for smart contracts whose version 6 targets `pallet-revive`. Its repository states that it has not been actively maintained since January 2026. For Rust contracts with Parity tooling, see [Rust Contracts](/smart-contracts/rust-contracts.md). |

## Wallets, Indexers, and Oracles

Wallets, indexers, and oracles that integrate with Polkadot SDK-based chains are listed in the Polkadot Developer Docs under [Wallets](https://developers.polkadot.com/parachains/integrations/wallets/), [Indexers](https://developers.polkadot.com/parachains/integrations/indexers/), and [Oracles](https://developers.polkadot.com/parachains/integrations/oracles/). For wallets that support Polkadot Hub smart contracts, see [Wallets for Polkadot Hub](https://developers.polkadot.com/smart-contracts/integrations/wallets/).
