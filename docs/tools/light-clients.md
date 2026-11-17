---
title: Light Clients
description: Connect to Polkadot SDK-based chains without trusting a third-party RPC node, using the smoldot light client and Substrate Connect.
categories: Polkadot SDK, Tooling
---

# Light Clients

## Introduction

Light clients enable secure and efficient blockchain interaction without running a Full Node. They provide a trust-minimized alternative to JSON-RPC by verifying data through cryptographic proofs rather than blindly trusting remote nodes.

This guide covers:

- What light clients are and how they work.
- Their advantages compared to full nodes and JSON-RPC.
- The smoldot light client and Substrate Connect.
- Libraries that work with them.

Light clients are particularly valuable for resource-constrained environments and applications requiring secure, decentralized blockchain access without the overhead of maintaining full nodes.

!!!note "Light node or light client?"
    The terms _light node_ and _light client_ are interchangeable. Both refer to a blockchain client that syncs without downloading the entire blockchain state. All nodes in a blockchain network are fundamentally clients, engaging in peer-to-peer communication.

## Light Clients Workflow

Unlike JSON-RPC interfaces, where an application must maintain a list of providers or rely on a single node, light clients are not limited to or dependent on a single node. They use cryptographic proofs to verify the blockchain's state, ensuring it is up-to-date and accurate. By verifying only block headers, light clients avoid syncing the entire state, making them ideal for resource-constrained environments.

```mermaid
flowchart LR
DAPP([dApp])-- Query Account Info -->LC([Light Client])
LC -- Request --> FN(((Full Node)))
LC -- Response --> DAPP
FN -- Response (validated via Merkle proof) --> LC
```

In the diagram above, the decentralized application queries on-chain account information through the light client. The light client runs as part of the application and requires minimal memory and computational resources. It uses Merkle proofs to verify the state retrieved from a full node in a trust-minimized manner. Polkadot-compatible light clients use warp syncing, which downloads only block headers.

Light clients can quickly verify the blockchain's state, including GRANDPA finality justifications.

!!!note "What does it mean to be trust-minimized?"
    _Trust-minimized_ means that the light client does not need to fully trust the full node from which it retrieves the state. This is achieved through the use of Merkle proofs, which allow the light client to verify the correctness of the state by checking the Merkle tree root.

## JSON-RPC and Light Client Comparison

Another common method of communication between a user interface (UI) and a node is through the JSON-RPC protocol. Generally, the UI retrieves information from the node, fetches network or Pallet data, and interacts with the blockchain. This is typically done in one of two ways:

- **User-controlled nodes**: The UI connects to a node client installed on the user's machine.
    - These nodes are secure, but installation and maintenance can be inconvenient.
- **Publicly accessible nodes**: The UI connects to a third-party-owned publicly accessible node client.
    - These nodes are convenient but centralized and less secure. Applications must maintain a list of backup nodes in case the primary node becomes unavailable.

While light clients still communicate with full nodes, they offer significant advantages for applications requiring a secure alternative to running a full node:

| Full Node                                                                                       | Light Client                                                   |
| :---------------------------------------------------------------------------------------------: | :------------------------------------------------------------: |
| Fully verifies all blocks of the chain                                                          | Verifies only the authenticity of blocks                       |
| Stores previous block data and the chain's storage in a database                                | Does not require a database                                    |
| Installation, maintenance, and execution are resource-intensive and require technical expertise | No installation is typically included as part of the application |

## smoldot

[smoldot](https://github.com/smol-dot/smoldot) is the light client implementation for Polkadot SDK-based chains. It connects to a chain from its chain specification, warp syncs, and serves the JSON-RPC interface to your application locally. smoldot is available in the following forms:

- **JavaScript package**: The [`smoldot`](https://www.npmjs.com/package/smoldot) npm package runs in the browser and in Node.js or Deno. See the [JavaScript API documentation](https://paritytech.github.io/smoldot/doc-javascript/).
- **Rust library**: The [`smoldot-light`](https://crates.io/crates/smoldot-light) crate embeds a light client in a Rust application. See the [`smoldot-light` documentation](https://docs.rs/smoldot-light).
- **Primitives**: The [`smoldot`](https://crates.io/crates/smoldot) crate provides lower-level Rust primitives that the other components build on.

For applications in other languages, the smoldot README suggests either wrapping `smoldot-light` in a small Rust library with bindings, or embedding the JavaScript package.

## Substrate Connect

[Substrate Connect](https://github.com/paritytech/substrate-connect) is a set of npm packages that run smoldot in the browser so a web application can connect to a chain without an RPC server:

- **[`@substrate/connect`](https://www.npmjs.com/package/@substrate/connect)**: Connects an application to a chain through smoldot.
- **[`@substrate/connect-known-chains`](https://www.npmjs.com/package/@substrate/connect-known-chains)**: Provides chain specifications for well-known chains.
- **[`@substrate/discovery`](https://www.npmjs.com/package/@substrate/discovery)**: Lets an application discover browser extensions that expose a light client.
- **[`@substrate/light-client-extension-helpers`](https://www.npmjs.com/package/@substrate/light-client-extension-helpers)**: Helps add a light client to a browser extension.

A light client running in a browser extension can be shared by several applications, which avoids repeating startup and sync in each one. It also addresses a browser limitation: HTTPS pages can only open secure WebSocket connections, and many nodes do not offer TLS, so an in-page light client may find too few peers. An extension keeps chains synced in the background and improves application performance. When no such extension is installed, Substrate Connect falls back to an in-page smoldot instance.

See the [Substrate Connect API documentation](https://paritytech.github.io/substrate-connect) for details.

## Works With

Other client libraries can use smoldot as their transport:

- **Subxt**: The Rust library can connect through smoldot with its `light-client` feature. See [Subxt](subxt.md#light-client-support).
- **Third-party libraries**: For example, [Polkadot-API (PAPI)](https://developers.polkadot.com/reference/tools/papi/) supports smoldot-based light client connections.

## Resources

- [smoldot repository](https://github.com/smol-dot/smoldot)
- [Substrate Connect repository](https://github.com/paritytech/substrate-connect)
- [Introducing Substrate Connect: Browser-Based Light Clients for Connecting to Substrate Chains](https://www.parity.io/blog/introducing-substrate-connect)
