---
title: Get Started with Smart Contracts
description: Start building smart contracts on pallet-revive with a local development node and Hardhat Polkadot, then connect to a live network when you are ready.
categories: Basics, Smart Contracts
---

# Get Started with Smart Contracts

## Introduction

This page collects the starting points for building smart contracts on [pallet-revive](/smart-contracts/overview.md). Start with a local development environment, then connect to a live network such as Polkadot Hub when you are ready to deploy. To run pallet-revive on your own chain, see [Add pallet-revive to Your Runtime](/polkadot-sdk/customize-runtime/add-pallet-revive.md).

pallet-revive runs contracts on two backends. REVM executes standard EVM bytecode and is the default choice for existing Solidity projects. PVM runs contracts compiled to PolkaVM with [`resolc`](/smart-contracts/resolc.md); it is newer, and its tooling is still maturing.

## Build and Test Locally

Run a local chain and iterate on your contracts before deploying anywhere. The local development node serves Ethereum JSON-RPC at `{{ networks.local.eth_rpc_url }}`.

<!-- INDEX TABLE START
dir: dev-environments
flat: true
extra_rows:
  - title: '[OpenZeppelin Contracts Wizard for Polkadot](https://wizard.openzeppelin.com/polkadot)'
    tools: OpenZeppelin
    description: Generate ERC-20, ERC-721, and other OpenZeppelin-standard contracts without writing code
  - title: '[Rust for PVM](/smart-contracts/overview.md#pvm)'
    tools: "LLMs, coding agents"
    description: Write PVM smart contracts natively in Rust; use AI assistants while native tooling matures
-->
<!-- INDEX TABLE END -->

## Ethereum Tool Differences on Polkadot EVM

Tools like Foundry and Hardhat are built for standard Ethereum nodes. A chain running pallet-revive exposes the same [Ethereum JSON-RPC](https://ethereum.org/developers/docs/apis/json-rpc/) interface through the [`eth-rpc` adapter](/smart-contracts/eth-rpc-adapter.md), but executes contracts in a different environment (a Polkadot SDK runtime with the REVM or PVM backend). As a result:

- **Local tests** (for example, `forge test` or Hardhat's default network) run in the tool's own EVM, not in pallet-revive, so behavior can differ from the real chain.
- **Time and snapshot helpers** (for example, `evm_increaseTime` or `loadFixture`) are often not supported by the adapter.
- **Gas reports** from these tools may not match the gas charged on-chain.

To check chain-specific behavior, run your contracts against a [local development node](/smart-contracts/dev-environments/local-dev-node.md). For more details, see [EVM vs PVM](/smart-contracts/for-eth-devs/evm-vs-pvm.md), [Gas Model](/smart-contracts/for-eth-devs/gas-model.md), and [Contract Deployment](/smart-contracts/for-eth-devs/contract-deployment.md). Guides for using third-party tools such as Remix, Foundry, and vanilla Hardhat are on the [Polkadot Developer Docs](https://developers.polkadot.com/smart-contracts/overview/).

## Connect to a Network

When you are ready to deploy to a live network, see [Connect to Polkadot](https://developers.polkadot.com/smart-contracts/connect/) on the Polkadot Developer Docs for network details, test tokens, and block explorers.

## Ethereum Developer Resources

Learn how pallet-revive differs from Ethereum: account mapping, gas and fees, JSON-RPC, and deployment.

<!-- INDEX TABLE START
dir: for-eth-devs
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

## Cookbook: Hands-on Tutorials

Follow step-by-step guides that walk through common tasks and complete dApp examples.

|                                            Tutorial                                            |        Tools        |                Description                |
|------------------------------------------------------------------------------------------------|---------------------|-------------------------------------------|
| [Deploy a Basic Contract](https://developers.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-remix/) |        Remix        |      Minimal deployment walkthrough       |
|    [Deploy an ERC-20](https://developers.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-remix/)     | Remix, OpenZeppelin | Create, deploy, and mint a fungible token |
|   [Deploy an NFT (ERC-721)](https://developers.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-nft/nft-remix/)   | Remix, OpenZeppelin |    Build and deploy an NFT collection     |
|                 [Uniswap V2](https://developers.polkadot.com/smart-contracts/cookbook/eth-dapps/uniswap-v2/)                  |       Hardhat       | Full dApp project: compile, test, deploy  |
|               [Zero‑to‑Hero dApp](https://developers.polkadot.com/smart-contracts/cookbook/dapps/zero-to-hero/)               |      Multiple       |  End‑to‑end dApp patterns and practices   |

## Libraries

pallet-revive exposes the standard Ethereum JSON-RPC API through the [`eth-rpc` adapter](for-eth-devs/json-rpc-apis.md), so Ethereum client libraries such as ethers.js, viem, and wagmi work unchanged. Setup guides for these libraries are in the [Polkadot Developer Docs](https://developers.polkadot.com/smart-contracts/libraries/ethers-js/).

## Integrations

Wallets and other integrations for dApps are covered in the [Polkadot Developer Docs](https://developers.polkadot.com/smart-contracts/integrations/wallets/).

## Precompiles

Discover the precompiles that pallet-revive implements and how to use them.

<!-- INDEX TABLE START
dir: precompiles
flat: true
columns: [title, description]
-->
<!-- INDEX TABLE END -->

From here, iterate locally with the development node and Hardhat Polkadot, connect to a network when you are ready, and use the guides, libraries, integrations, and precompiles as you grow into production-ready dApps. If you get stuck, [open an issue](https://github.com/paritytech/parity-docs/issues/new?template=docs-issue.yml) or reach out in the community channels.
