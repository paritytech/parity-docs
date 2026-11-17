---
title: What Parity Builds
description: Parity writes open-source software for Polkadot SDK-based chains. Learn what these docs cover, Parity's role as an author of code, and where network docs live.
---

# What Parity Builds

## Introduction

Parity Technologies writes open-source software: the Polkadot SDK, smart contract tooling, the Product SDK, node binaries, and client libraries. These docs explain how to build with that software, configure it, and run it yourself.

## Parity's Role

--8<-- 'text/parity-role.md'

These docs follow the same principle. They describe software, not live networks. When an example connects to a network, that network and its endpoints are run by others, and you can point the same software at any compatible deployment, including one you run locally.

## What These Docs Cover

- **[Polkadot SDK](../polkadot-sdk/get-started.md)**: Build, test, and upgrade runtimes and parachains with FRAME, Cumulus, and the SDK tooling.
- **[Smart contracts](../smart-contracts/overview.md)**: Develop for `pallet-revive` with its REVM and PVM backends, the Ethereum JSON-RPC adapter, and the Hardhat Polkadot plugin.
- **[Apps](../apps/index.md)**: Build apps with the Product SDK and the protocols behind it.
- **[Nodes](../nodes/index.md)**: Install and run the Polkadot SDK node binaries.
- **Tools**: Use Parity's client libraries and developer tools, such as [Subxt](../tools/subxt.md) and [Zombienet](../tools/zombienet.md).

## Where to Find Polkadot Network Documentation

The Polkadot network and its ecosystem are documented by the Polkadot Community Foundation in the [Polkadot Developer Docs](https://developers.polkadot.com/). Pages on these topics used to live on this site; their old URLs now redirect there.

| Topic | Where to find it |
|:------|:-----------------|
| Connect to Polkadot Hub, get test tokens, and find explorers | [Smart contracts: Connect](https://developers.polkadot.com/smart-contracts/connect/), [Faucet](https://developers.polkadot.com/smart-contracts/faucet/), [Explorers](https://developers.polkadot.com/smart-contracts/explorers/) |
| Ethereum tools and libraries on Polkadot Hub (Remix, Foundry, ethers.js, viem) | [Development environments](https://developers.polkadot.com/smart-contracts/dev-environments/remix/), [Libraries](https://developers.polkadot.com/smart-contracts/libraries/ethers-js/) |
| Smart contract tutorials and dApp examples | [Cookbook](https://developers.polkadot.com/smart-contracts/cookbook/dapps/zero-to-hero/) |
| Deploy a parachain to a live network and obtain coretime | [Deploy to Polkadot](https://developers.polkadot.com/parachains/launch-a-parachain/deploy-to-polkadot/), [Obtain coretime](https://developers.polkadot.com/parachains/launch-a-parachain/obtain-coretime/) |
| Open HRMP channels between parachains | [Channels between parachains](https://developers.polkadot.com/parachains/interoperability/channels-between-parachains/) |
| Run a validator, staking, rewards, and slashing | [Validator requirements](https://developers.polkadot.com/node-infrastructure/run-a-validator/requirements/) |
| Governance, origins, and tracks | [Governance](https://developers.polkadot.com/reference/governance/) |
| Polkadot networks (Polkadot, Kusama, Paseo, Westend) | [Networks](https://developers.polkadot.com/reference/parachains/networks/) |
| Assets and cross-chain transfers on live chains | [Register a local asset](https://developers.polkadot.com/chain-interactions/token-operations/register-local-asset/), [Transfer assets between parachains](https://developers.polkadot.com/chain-interactions/send-transactions/interoperability/transfer-assets-parachains/) |
| Polkadot Hub architecture, consensus, and coretime | [Polkadot Hub](https://developers.polkadot.com/reference/polkadot-hub/) |
| Third-party tools such as Chopsticks, Polkadot-API, and ParaSpell | [Tools](https://developers.polkadot.com/reference/tools/chopsticks/) |
| Wallets, indexers, and oracles for parachains | [Integrations](https://developers.polkadot.com/parachains/integrations/wallets/) |

For background on the Polkadot protocol, see the [Polkadot Wiki](https://wiki.polkadot.com/).
