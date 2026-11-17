---
title: Add Smart Contract Functionality
description: Add smart contract support to your Polkadot SDK-based blockchain with pallet-revive, which runs both EVM and PVM bytecode behind an Ethereum JSON-RPC interface.
short_description: Enable smart contract capabilities with pallet-revive.
categories: Polkadot SDK
---

# Add Smart Contract Functionality

## Introduction

When you build a blockchain with the Polkadot SDK, you can add smart contract support by including a smart contract pallet in your runtime. Users can then deploy and call contracts on your chain, and developers can build decentralized applications (dApps) on it with familiar tooling.

The Polkadot SDK smart contract pallet is [`pallet-revive`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive). This page explains what `pallet-revive` provides, the components you add to your chain, and where to find reference runtimes that configure it.

## What pallet-revive Provides

`pallet-revive` is the runtime module that executes smart contracts. Because it lives in the runtime rather than in the node binary, your chain gains Ethereum compatibility without changes to its node client.

`pallet-revive` runs contracts on two execution backends:

- **[REVM](/smart-contracts/overview.md#revm)**: Executes standard EVM bytecode compiled with `solc`, so existing Solidity contracts can be deployed unchanged.
- **[PVM](/smart-contracts/overview.md#pvm)**: Executes contracts compiled to PolkaVM bytecode with `resolc`, based on the [RISC-V](https://en.wikipedia.org/wiki/RISC-V) instruction set.

| Language | Compiler | Output Bytecode | Execution Backend |
|----------|----------|-----------------|-------------------|
| Solidity | `resolc` |       PVM       |        PVM        |
| Solidity |  `solc`  |       EVM       |       REVM        |

Any language that can compile to PVM bytecode and use the `pallet-revive` host functions (through [`pallet-revive-uapi`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive_uapi/index.html)) can target the PVM backend.

Both backends share the same account model, gas model, and precompiles. For a full description of the execution model, see the [Smart Contracts Overview](/smart-contracts/overview.md).

## Components to Add

Adding smart contract support involves two components:

- **Runtime pallet**: Add `pallet-revive` to your runtime and implement its `Config` trait. The configuration sets values such as storage deposits, the chain ID reported to Ethereum tools, whether EVM bytecode is allowed, which origins can upload and instantiate contracts, and which precompiles are available.
- **Ethereum JSON-RPC adapter**: Run the [`pallet-revive-eth-rpc`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/rpc) server (the `eth-rpc` binary) next to your node. It translates Ethereum JSON-RPC requests into Polkadot SDK transactions so that Ethereum wallets and tools such as Hardhat can interact with your chain.

To try `pallet-revive` and the adapter locally before integrating them into your own runtime, follow the [Local Development Node](/smart-contracts/dev-environments/local-dev-node.md) guide.

## Reference Implementations

The Polkadot SDK repository includes runtimes that configure `pallet-revive`, which you can use as a starting point for your own `Config` implementation:

- **Asset Hub Westend runtime**: See the [`pallet_revive::Config` implementation](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs#L1387-L1435) for a parachain configuration that exposes asset, XCM, and asset conversion precompiles.
- **Kitchensink runtime**: See the [`pallet_revive::Config` implementation](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs#L1556-L1590) in the Substrate node runtime used by the [kitchensink node](/polkadot-sdk/install-polkadot-sdk.md#optional-run-the-kitchensink-node).

!!! note
    Line ranges refer to the Polkadot SDK version `{{dependencies.repositories.polkadot_sdk.version}}`. If the links point to the wrong lines in a later version, search the file for `impl pallet_revive::Config for Runtime`.

## Other Smart Contract Pallets

[`pallet-contracts`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/contracts) is the earlier Wasm-based smart contract pallet and is considered legacy, as development has moved to `pallet-revive`.

[Frontier](https://github.com/polkadot-evm/frontier), maintained by the polkadot-evm project, is an alternative Ethereum compatibility layer for Polkadot SDK chains built around `pallet-evm` and `pallet-ethereum`.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Add pallet-revive to Your Runtime__

    ---

    Follow step-by-step instructions to add `pallet-revive` and its runtime APIs to your runtime.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/customize-runtime/add-pallet-revive.md)

-   <span class="badge guide">Guide</span> __Add a Pallet to the Runtime__

    ---

    Learn the step-by-step process for integrating Polkadot SDK pallets into your blockchain's runtime.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/customize-runtime/add-existing-pallets.md)

-   <span class="badge tutorial">Tutorial</span> __Local Development Node__

    ---

    Run a local node with `pallet-revive` and the Ethereum JSON-RPC adapter for smart contract development.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/dev-environments/local-dev-node.md)

</div>
