---
title: Hardhat Polkadot Plugin
description: Set up the Hardhat Polkadot plugin to compile Solidity contracts to PVM with resolc, test them on a local dev node, and deploy to any chain running pallet-revive.
short_description: Compile, test, and deploy Solidity contracts to PVM with Hardhat.
categories: Smart Contracts, Tooling
page_badges:
  tutorial_badge: Beginner
  test_workflow: polkadot-docs-hardhat-pvm
page_tests:
  path: polkadot-docs/smart-contracts/dev-environments/hardhat-pvm/tests/docs.test.ts
---

# Hardhat Polkadot Plugin

## Introduction

The [`@parity/hardhat-polkadot`](https://github.com/paritytech/hardhat-polkadot) plugin extends [Hardhat](https://hardhat.org/) so you can compile Solidity contracts to PVM bytecode with the [`resolc`](/smart-contracts/resolc.md) compiler, test them against a local dev node, and deploy them to any chain running `pallet-revive` through its [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter.md) (`eth-rpc`).

pallet-revive runs contracts on two backends. REVM executes standard EVM bytecode and is the default choice for existing Solidity projects. PVM runs contracts compiled to PolkaVM with `resolc`; it is newer, and its tooling is still maturing. This plugin targets PVM. For standard Hardhat against REVM, see the [Hardhat guide on developers.polkadot.com](https://developers.polkadot.com/smart-contracts/dev-environments/hardhat/).

## Prerequisites

Before setting up Hardhat, make sure the following are installed:

- [Node.js](https://nodejs.org/) version 22.5+ and [npm](https://www.npmjs.com/) version 10.9.0+ to avoid issues with the Polkadot plugin
- A package manager like npm, [pnpm](https://pnpm.io/), or [yarn](https://yarnpkg.com/)

## Initialize a Hardhat Project

1. Create a directory to hold your project files:

    ```bash
    mkdir hardhat-pvm-example
    cd hardhat-pvm-example
    ```

2. Initialize a new npm project:

    ```bash
    npm init -y
    ```

3. Install the Polkadot plugin for Hardhat:

    ```bash
    npm install --save-dev @parity/hardhat-polkadot@{{dependencies.javascript_packages.hardhat_polkadot.version}}
    ```

4. Install the `resolc` compiler (required for compiling Solidity to PVM):

    ```bash
    npm install --save-dev @parity/resolc@{{dependencies.javascript_packages.resolc.version}}
    ```

5. Initialize a Hardhat project:

    ```bash
    npx hardhat-polkadot init
    ```

    Follow the project creation wizard. Your project will be created with three main folders:

    - **`contracts`**: Stores your Solidity smart contracts.
    - **`ignition`**: Contains deployment modules for safely deploying your contracts to various networks.
    - **`test`**: Contains test files that validate contract functionality.

6. Add the following folder to the `.gitignore` file if it is not already there:

    ```bash
    echo '/ignition/deployments/' >> .gitignore
    ```

7. Complete the setup by installing all dependencies:

    ```bash
    npm install
    ```

## Compile Your Contract

The generated `hardhat.config.js` includes a network entry that reads a `PRIVATE_KEY` configuration variable, and Hardhat stops with error `HH1201` while that variable is unset. Either set it as described in [Deploy to Any eth-rpc Endpoint](#deploy-to-any-eth-rpc-endpoint), or remove that network entry until you need it.

The plugin compiles your Solidity contracts (version `0.8.0` and higher) to PVM bytecode using `resolc`:

```bash
npx hardhat compile
```

After successful compilation, you'll see the artifacts generated in the `artifacts` directory:

```bash
ls artifacts/contracts/*.sol/
```

You should see JSON files containing the contract ABIs and bytecodes for the contracts you compiled.

## Test Your Contract Locally

The plugin can start a local dev node together with the `eth-rpc` adapter, so you can test against `pallet-revive` without connecting to a live network.

### Start a Local Node

The plugin needs a `pallet-revive` dev node binary and an `eth-rpc` binary. You can get them in either of the following ways, then update the binary paths in your `hardhat.config.js`:

- Build `revive-dev-node` and `eth-rpc` from the Polkadot SDK, as described in [Local Development Node](/smart-contracts/dev-environments/local-dev-node.md).
- Download prebuilt binaries for your platform from the [`hardhat-polkadot` releases](https://github.com/paritytech/hardhat-polkadot/releases).

!!! note
    You might need to give executable permissions to the binaries:

    ```bash
    chmod +x /path/to/your/binary
    ```

    In macOS environments, binaries are sometimes quarantined. To remove this, run:

    ```bash
    xattr -d com.apple.quarantine /path/to/your/binary
    ```

By default, the plugin starts an Anvil-compatible node and passes it Anvil flags, which `revive-dev-node` rejects. Set `useAnvil: false` in the `hardhat` network's `nodeConfig` to run `revive-dev-node` with the `eth-rpc` adapter. Also set `ignition.requiredConfirmations` to `1`, because the dev node seals a block only when it receives a transaction:

```javascript title="hardhat.config.js"
module.exports = {
  // ...
  ignition: {
    requiredConfirmations: 1,
  },
  networks: {
    hardhat: {
      polkadot: {
        target: 'evm',
      },
      nodeConfig: {
        nodeBinaryPath: 'INSERT_PATH_TO_REVIVE_DEV_NODE',
        useAnvil: false,
        rpcPort: 8000,
        dev: true,
      },
      adapterConfig: {
        adapterBinaryPath: 'INSERT_PATH_TO_ETH_RPC',
        dev: true,
      },
    },
    // ...
  },
};
```

Once you have set up the binaries, start your local testing node:

```bash
npx hardhat node
```

This command launches a local node with the `eth-rpc` adapter. By default, the node's Substrate RPC runs on `localhost:8000`, and the `eth-rpc` adapter on `localhost:8545`.

The output will be something like this:

--8<-- 'code/smart-contracts/dev-environments/hardhat-polkadot/hardhat-node-output.html'

### Run Tests

To run your contract tests:

```bash
npx hardhat test
```

!!! warning "Compatibility Note"
    The `eth-rpc` adapter exposes a subset of the Ethereum JSON-RPC API, so [`@nomicfoundation/hardhat-toolbox/network-helpers`](https://hardhat.org/hardhat-network-helpers/docs/overview) is not fully compatible with it. Specifically, helpers like `time` and `loadFixture` may not work due to missing RPC calls in the node. For a broader overview, see [Ethereum Tool Differences on Polkadot EVM](/smart-contracts/get-started.md#ethereum-tool-differences-on-polkadot-evm). For plugin-specific details, refer to the [Compatibility](https://github.com/paritytech/hardhat-polkadot/tree/main/packages/hardhat-polkadot-node#compatibility) section in the `hardhat-polkadot` documentation.

### Deploy to the Local Node

Deploy your contract to the local node using [Ignition](https://hardhat.org/ignition/docs/getting-started#overview) modules:

1. Ensure your local node is running:

    ```bash
    npx hardhat node
    ```

    If you run the [Local Development Node](/smart-contracts/dev-environments/local-dev-node.md) yourself instead, its `eth-rpc` adapter listens on the same endpoint, `{{ networks.local.eth_rpc_url }}`.

2. In a new terminal window, deploy the contract using Ignition:

    ```bash
    npx hardhat ignition deploy ./ignition/modules/MyToken.js --network localNode
    ```

!!! tip
    The `ignition.requiredConfirmations: 1` setting from [Start a Local Node](#start-a-local-node) ensures that deployment doesn't hang on local nodes that only produce blocks when transactions are submitted.

## Deploy to Any eth-rpc Endpoint

After testing locally, you can deploy to any chain running `pallet-revive` that exposes an `eth-rpc` endpoint. You need the endpoint URL, the chain ID, and an account funded on that chain.

1. Add a network entry to the `networks` section of `hardhat.config.js`, following the format of the entries the init wizard generated. The following example uses the values for Polkadot Hub TestNet; replace them with the values for your target chain:

    ```javascript title="hardhat.config.js"
    polkadotTestnet: {
      url: '{{ networks.polkadot_hub_testnet.eth_rpc_url }}',
      chainId: {{ networks.polkadot_hub_testnet.chain_id }},
      accounts: [vars.get('PRIVATE_KEY')],
    },
    ```

    For endpoints, chain IDs, test tokens, and explorers of public networks, see [Connect to Polkadot](https://developers.polkadot.com/smart-contracts/connect/) on developers.polkadot.com.

2. Set your private key as a configuration variable:

    ```bash
    npx hardhat vars set PRIVATE_KEY
    ```

    !!! warning
        Never reveal your private key. Anyone with access to it can control your wallet and steal your funds. Store it securely and never share it publicly or commit it to version control systems.

3. Verify your private key is set:

    ```bash
    npx hardhat vars get PRIVATE_KEY
    ```

4. Deploy your contract, passing the name of the network entry you added:

    ```bash
    npx hardhat ignition deploy ./ignition/modules/MyToken.js --network polkadotTestnet
    ```

## Common Issues

- **Compilation fails with `resolc` errors**: Ensure you have `@parity/resolc@{{dependencies.javascript_packages.resolc.version}}` installed and the version is specified in your `hardhat.config.js`.
- **Deployment hangs on local node**: You might need to set `ignition.requiredConfirmations: 1` in your config file.
- **Binary permission issues**: Run `chmod +x /path/to/your/binary` and on macOS, use `xattr -d com.apple.quarantine /path/to/your/binary`.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Deploy a Basic Contract__

    ---

    Compile and deploy a basic contract to the local dev node with the Hardhat Polkadot plugin.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/cookbook/smart-contracts/deploy-basic/basic-hardhat.md)

-   <span class="badge guide">Guide</span> __Local Development Node__

    ---

    Build and run the `pallet-revive` dev node and the `eth-rpc` adapter from the Polkadot SDK.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/dev-environments/local-dev-node.md)

-   <span class="badge guide">Guide</span> __The resolc Compiler__

    ---

    Install `resolc` and learn its compiler options for Solidity to PVM compilation.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/resolc.md)

-   <span class="badge external">External</span> __Hardhat Documentation__

    ---

    Learn more about Hardhat's advanced features and best practices.

    [:octicons-arrow-right-24: Get Started](https://hardhat.org/docs)

-   <span class="badge external">External</span> __Hardhat Polkadot Plugin__

    ---

    Explore the full capabilities of the Hardhat Polkadot plugin.

    [:octicons-arrow-right-24: Get Started](https://github.com/paritytech/hardhat-polkadot)

</div>
