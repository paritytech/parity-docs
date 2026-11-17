---
title: Clients and Tools
description: Parity's client libraries and developer tools for Polkadot SDK-based chains, including Subxt, the Polkadot REST API, smoldot, Zombienet, and Omni Node.
categories: Tooling
---

# Clients and Tools

## Introduction

Parity builds client libraries and developer tools that work with any chain built on the Polkadot SDK. Use them to connect to a chain, read its state, submit transactions, and run local networks for development and testing.

## Interact with a Chain

Task-based guides for reading chain state, submitting transactions, storing data, and managing accounts are in [Interact with a Chain](chain-interactions/index.md).

## Client Libraries

| Tool | Use it to |
|:-----|:----------|
| [Subxt](subxt.md) | Connect to a chain from Rust, query storage, call runtime APIs, and submit transactions. |
| [Polkadot REST API](polkadot-rest-api.md) | Run a REST service in front of a node and query chain data over HTTP. Successor to the deprecated Substrate API Sidecar. |
| [Light clients](light-clients.md) | Connect to a chain without trusting a third-party RPC node, using smoldot and Substrate Connect. |

## Node and Network Tools

| Tool | Use it to |
|:-----|:----------|
| [Zombienet](zombienet.md) | Spin up local relay chain and parachain networks for development and testing, from the CLI or from Rust with `zombienet-sdk`. |
| [Chain Spec Builder](chain-spec-builder.md) | Generate, inspect, verify, and convert chain specifications from a compiled runtime Wasm blob. |
| [Polkadot Omni Node](omninode.md) | Run a parachain from its runtime and chain specification, without building a custom node. |
| [Utilities](utilities.md) | Convert and hash values, re-encode addresses, and derive ERC-20 precompile addresses in the browser. |

## Third-Party Tools

Many community and third-party tools also work with Polkadot SDK-based chains, including Polkadot-API, Dedot, polkadot.js, Chopsticks, and ParaSpell. They are documented in the [Polkadot Developer Docs](https://developers.polkadot.com/reference/tools/chopsticks/).
