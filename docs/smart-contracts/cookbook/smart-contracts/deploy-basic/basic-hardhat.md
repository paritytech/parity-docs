---
title: Deploy a Basic Contract with Hardhat
description: Learn how to compile a basic Solidity contract to PVM and deploy it to a local pallet-revive dev node using the Hardhat Polkadot plugin.
categories: Smart Contracts
tools: [Hardhat]
page_badges:
  tutorial_badge: Beginner
  test_workflow: polkadot-docs-basic-hardhat
page_tests:
  path: polkadot-docs/smart-contracts/basic-hardhat/tests/docs.test.ts
---

# Deploy a Basic Contract with Hardhat

## Introduction

This guide demonstrates how to compile a basic Solidity smart contract to PVM and deploy it to a local dev node running `pallet-revive`, using [Hardhat](https://hardhat.org/) with the [Hardhat Polkadot plugin](/smart-contracts/dev-environments/hardhat-polkadot/). Hardhat provides a development environment with built-in testing, debugging, and deployment capabilities.

## Prerequisites

Before you begin, ensure you have the following:

- A basic understanding of [Solidity](https://www.soliditylang.org/) programming.
- [Node.js](https://nodejs.org/) version 22.5+ and [npm](https://www.npmjs.com/) version 10.9.0+.
- The `revive-dev-node` and `eth-rpc` binaries, built as described in [Local Development Node](/smart-contracts/dev-environments/local-dev-node/) or downloaded from the [`hardhat-polkadot` releases](https://github.com/paritytech/hardhat-polkadot/releases).

## Set Up Your Project

1. Create a directory and initialize an npm project inside it:

    ```bash
    mkdir hardhat-deployment
    cd hardhat-deployment
    npm init -y
    ```

2. Install the Hardhat Polkadot plugin and the `resolc` compiler:

    ```bash
    npm install --save-dev @parity/hardhat-polkadot@{{dependencies.javascript_packages.hardhat_polkadot.version}}
    npm install --save-dev @parity/resolc@{{dependencies.javascript_packages.resolc.version}}
    ```

3. Initialize the Hardhat project and follow the project creation wizard:

    ```bash
    npx hardhat-polkadot init
    ```

4. Ignore Ignition deployment files and install the project dependencies:

    ```bash
    echo '/ignition/deployments/' >> .gitignore
    npm install
    ```

## Configure Hardhat

Open `hardhat.config.js` and update the binary paths to point at your `revive-dev-node` and `eth-rpc` binaries. For details on the plugin configuration, see [Hardhat Polkadot Plugin](/smart-contracts/dev-environments/hardhat-polkadot/#start-a-local-node).

## Create the Contract

Follow these steps to create your smart contract:

1. Delete the default contract file(s) in the `contracts` directory.

2. Create a new file named `Storage.sol` inside the `contracts` directory.

3. Add the following code to create the `Storage.sol` smart contract:

    ```solidity title="Storage.sol"
    --8<-- 'code/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat/Storage.sol'
    ```

## Compile the Contract

Compile your `Storage.sol` contract to PVM bytecode using the following command:

```bash
npx hardhat compile
```

After successful compilation, the contract ABI and bytecode are written to the `artifacts/contracts/Storage.sol/` directory.

## Deploy the Contract

Deploy the contract to the local dev node as follows:

1. Delete the default file(s) inside the `ignition/modules` directory.

2. Create a new file named `Storage.js` inside the `ignition/modules` directory.

3. Open `ignition/modules/Storage.js` and add the following code to create your deployment module:

    ```javascript title="ignition/modules/Storage.js"
    --8<-- 'code/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat/storage.js'
    ```

4. Start the local dev node and the `eth-rpc` adapter:

    ```bash
    npx hardhat node
    ```

    The `eth-rpc` adapter listens on `{{ networks.local.eth_rpc_url }}`. The dev node uses pre-funded development accounts, so you don't need test tokens.

5. In a new terminal window, deploy your contract to the local node:

    ```bash
    npx hardhat ignition deploy ./ignition/modules/Storage.js --network localNode
    ```

    When the deployment finishes, Ignition prints the address of the deployed `Storage` contract.

You've deployed a basic smart contract to a local `pallet-revive` dev node using Hardhat. To deploy the same contract to a live network, see [Deploy to Any eth-rpc Endpoint](/smart-contracts/dev-environments/hardhat-polkadot/#deploy-to-any-eth-rpc-endpoint).

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Hardhat Polkadot Plugin__

    ---

    Test contracts locally and deploy them to any chain running `pallet-revive`.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/dev-environments/hardhat-polkadot/)

-   <span class="badge guide">Guide</span> __Deploy an ERC-20__

    ---

    Walk through deploying an ERC-20 token using Hardhat on developers.polkadot.com.

    [:octicons-arrow-right-24: Get Started](https://developers.polkadot.com/smart-contracts/cookbook/smart-contracts/deploy-erc20/erc20-hardhat/)

</div>
