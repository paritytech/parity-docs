---
title: Local Development Node
description: Follow this step-by-step guide to install a Revive Dev node and ETH-RPC adapter for smart contract development in a local environment.
short_description: Spin up a local node for iterative development.
categories: Smart Contracts
tools: [Polkadot SDK node]
page_badges:
  tutorial_badge: Beginner
  test_workflow: polkadot-docs-local-dev-node
page_tests:
  path: polkadot-docs/smart-contracts/local-dev-node/tests/docs.test.ts
---

# Local Development Node

## Introduction

A local development node provides an isolated blockchain environment where you can deploy, test, and debug smart contracts without incurring network fees or waiting for block confirmations. This guide demonstrates how to build and run the `pallet-revive` dev node and the [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter.md) (`eth-rpc`) from the Polkadot SDK.

By the end of this guide, you'll have:

- A running node with smart contract support.
- An ETH-RPC adapter for Ethereum-compatible tooling integration accessible at `{{ networks.local.eth_rpc_url }}`.

!!! info "Tooling differences"
    The local node exposes a subset of the Ethereum JSON-RPC API. Some methods used by Foundry, Hardhat, or other Ethereum-native tools (for example, for time manipulation or debugging) may be missing or behave differently. See [Ethereum Tool Differences on Polkadot EVM](/smart-contracts/get-started.md#ethereum-tool-differences-on-polkadot-evm) for implications when running tests and tools against Polkadot nodes.

## Prerequisites

Before getting started, ensure you have done the following:

- Completed the [Install Polkadot SDK Dependencies](/polkadot-sdk/install-polkadot-sdk.md) guide and successfully installed [Rust](https://rust-lang.org/) and the required packages to set up your development environment.

## Install the Revive Dev Node and ETH-RPC Adapter

The Polkadot SDK repository contains both the [Revive Dev node](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node) implementation and the [ETH-RPC adapter](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/rpc) required for Ethereum compatibility. Start by cloning the repository and checking out the release these docs are written against:

```bash
git clone https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk
git checkout {{dependencies.repositories.polkadot_sdk.version}}
```

Next, you need to compile the two essential components for your development environment. The Substrate node provides the core blockchain runtime with smart contract support, while the ETH-RPC adapter enables Ethereum JSON-RPC compatibility for existing tooling:

```bash
cargo build -p revive-dev-node --bin revive-dev-node --release
cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release
```

The compilation process may take some time depending on your system specifications, potentially up to 30 minutes. Release builds are optimized for performance but take longer to compile than debug builds. After successful compilation, you can verify the binaries are available in the `target/release` directory:

- **Revive Dev node path**: `polkadot-sdk/target/release/revive-dev-node`
- **ETH-RPC adapter path**: `polkadot-sdk/target/release/eth-rpc`

## Run the Local Node

With the binaries compiled, you can now start your local development environment. The setup requires running two processes.

Start the node first, which will initialize a local blockchain with the `dev` chain specification. This configuration includes `pallet-revive` for smart contract functionality and uses pre-funded development accounts for testing:

```bash
./target/release/revive-dev-node --dev
```

The node will begin producing blocks immediately and display initialization logs:

--8<-- 'code/smart-contracts/dev-environments/local-dev-node/local-development-node-1.html'

For debugging purposes or to monitor low-level operations, you can enable detailed logging by setting environment variables before running the command:

```bash
RUST_LOG="error,evm=debug,sc_rpc_server=info,runtime::revive=debug" ./target/release/revive-dev-node --dev
```

Once the node is running, open a new terminal window and start the ETH-RPC adapter. This component translates Ethereum JSON-RPC calls into Substrate-compatible requests, allowing you to use familiar Ethereum tools like MetaMask, Hardhat, or Ethers.js:

```bash
./target/release/eth-rpc --dev
```

You should see logs indicating that the adapter is ready to accept connections:

--8<-- 'code/smart-contracts/dev-environments/local-dev-node/local-development-node-2.html'

Similar to the Revive Dev node, you can enable detailed logging for the ETH-RPC adapter to troubleshoot issues:

```bash
RUST_LOG="info,eth-rpc=debug" ./target/release/eth-rpc --dev
```

Your local development environment is now active and accessible at `{{ networks.local.eth_rpc_url }}`. This endpoint accepts standard Ethereum JSON-RPC requests, so it works with existing Ethereum development tools and workflows. 

You can connect wallets, deploy contracts using Remix or Hardhat, and interact with your smart contracts as you would on any Ethereum-compatible network. To compile contracts for the PVM backend, use the [`resolc` compiler](/smart-contracts/resolc.md).

