---
title: Set Up the Polkadot SDK Parachain Template
description: Learn how to set up and run the Polkadot SDK Parachain Template locally, creating a ready-to-customize foundation for your parachain.
short_description: Set up and run the Polkadot SDK Parachain Template locally.
categories: Basics, Polkadot SDK
page_badges:
  tutorial_badge: Beginner
  test_workflow: polkadot-docs-set-up-parachain-template
page_tests:
  path: polkadot-docs/parachains/set-up-parachain-template/tests/docs.test.ts
---

# Set Up the Polkadot SDK Parachain Template

## Introduction

The [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) includes several [templates](/polkadot-sdk/customize-runtime/#starting-templates) designed to help you quickly start building your own blockchain. Each template offers a different level of configuration, from minimal setups to feature-rich environments, allowing you to choose the foundation that best fits your project's needs.

Among these, the [Parachain Template](https://github.com/paritytech/polkadot-sdk-parachain-template) provides a preconfigured runtime with commonly used pallets, making it an ideal starting point for most parachain development projects.

This guide walks you through the full process of working with this template. You will:

- Set up the Polkadot SDK Parachain Template.
- Understand the project structure and key components.
- Verify your template is ready for development.
- Run the parachain template locally in development mode.

By the end of this guide, you'll have a working template ready to customize and deploy as a parachain.

## Prerequisites

Before getting started, ensure you have done the following:

- Completed the [Install Polkadot SDK](/polkadot-sdk/install-polkadot-sdk/) guide and successfully installed [Rust](https://rust-lang.org/) and the required packages to set up your development environment.

For this tutorial series, you need to use Rust `{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_version}}`. Newer versions of the compiler may not work with this parachain template version.

Run the following commands to set up the correct Rust version:

=== "macOS"

    ```bash
    rustup install {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_version}}
    rustup default {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_version}}
    rustup target add wasm32-unknown-unknown --toolchain {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_toolchain_macos}}
    rustup component add rust-src --toolchain {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_toolchain_macos}}
    ```

=== "Ubuntu"

    ```bash
    rustup toolchain install {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_toolchain_ubuntu}}
    rustup default {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_toolchain_ubuntu}}
    rustup target add wasm32-unknown-unknown --toolchain {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_toolchain_ubuntu}}
    rustup component add rust-src --toolchain {{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.rust_toolchain_ubuntu}}
    ```

## Polkadot SDK Utility Tools

This tutorial requires two essential tools:

- **[Chain spec builder](https://crates.io/crates/staging-chain-spec-builder/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.staging_chain_spec_builder_version}})**: A Polkadot SDK utility for generating chain specifications. See the [chain-spec-builder reference](/tools/chain-spec-builder/) for its commands and options.
- **[Polkadot Omni Node](https://crates.io/crates/polkadot-omni-node/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_omni_node_version}})**: A white-labeled binary, released as part of the Polkadot SDK, that can act as the collator of a parachain in production, with the auxiliary functionality a collator node needs, such as an RPC server and state archiving. It can also run the Wasm blob of the parachain locally for testing and development. See the [Omni Node reference](/tools/omninode/) for details.

Download the pre-built binaries from the [Polkadot SDK release](https://github.com/paritytech/polkadot-sdk/releases/tag/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_sdk_version}}) (recommended):

=== "macOS"

    ```bash
    curl -L -o chain-spec-builder https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_sdk_version}}/chain-spec-builder-aarch64-apple-darwin
    curl -L -o polkadot-omni-node https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_sdk_version}}/polkadot-omni-node-aarch64-apple-darwin
    chmod +x chain-spec-builder polkadot-omni-node
    sudo mv chain-spec-builder polkadot-omni-node /usr/local/bin/
    ```

=== "Ubuntu"

    ```bash
    curl -L -o chain-spec-builder https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_sdk_version}}/chain-spec-builder
    curl -L -o polkadot-omni-node https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_sdk_version}}/polkadot-omni-node
    chmod +x chain-spec-builder polkadot-omni-node
    sudo mv chain-spec-builder polkadot-omni-node /usr/local/bin/
    ```

Alternatively, you can install from source using `cargo`:

```bash
cargo install --locked staging-chain-spec-builder@{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.staging_chain_spec_builder_version}}
cargo install --locked polkadot-omni-node@{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.polkadot_omni_node_version}}
```

## Clone the Template

The [Polkadot SDK Parachain Template](https://github.com/paritytech/polkadot-sdk-parachain-template) provides a ready-to-use development environment for building with the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk). Follow these steps to set up the template:

1. Clone the template repository:

    ```bash
    git clone https://github.com/paritytech/polkadot-sdk-parachain-template.git parachain-template
    ```

2. Navigate into the project directory:

    ```bash
    cd parachain-template
    ```

## Explore the Project Structure

Before building the template, take a moment to familiarize yourself with its structure. Understanding this organization will help you navigate the codebase as you develop your parachain.

The template follows a standard Polkadot SDK project layout:

```text
parachain-template/
├── node/              # Node implementation and client
├── pallets/           # Custom pallets for your parachain
├── runtime/           # Runtime configuration and logic
├── Cargo.toml         # Workspace configuration
└── README.md          # Documentation
```

Key directories explained:

- **runtime/**: Contains your parachain's state transition function and pallet configuration. This is where you'll define what your blockchain can do.
- **node/**: Houses the client implementation that runs your blockchain, handles networking, and manages the database.
- **pallets/**: Where you'll create custom business logic modules (pallets) for your specific use case.
- **Cargo.toml**: The workspace configuration that ties all components together.

!!!note
    The runtime is compiled to WebAssembly (Wasm), enabling forkless upgrades. The node binary remains constant while the runtime can be updated on-chain.

## Compile the Runtime

Now that you understand the template structure, compile the runtime to confirm everything works.

1. Compile the runtime:

    ```bash
    cargo build --release --locked
    ```

    !!!tip
        Initial compilation may take several minutes, depending on your machine specifications. Use the `--release` flag for improved runtime performance compared to the default `--debug` build. If you need to troubleshoot issues, the `--debug` build provides better diagnostics.
        
        For production deployments, consider using a dedicated `--profile production` flag - this can provide an additional 15-30% performance improvement over the standard `--release` profile.

2. Upon successful compilation, you should see output indicating the build was successful. The compiled runtime will be located at:
    
    `./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm`

## Verify the Build

After compilation completes, verify that the runtime was created successfully by checking for the Wasm blob:

```bash
ls -la ./target/release/wbuild/parachain-template-runtime/
```

You should see the `parachain_template_runtime.compact.compressed.wasm` file in the output, confirming the build was successful.

## Run the Node Locally

After successfully compiling your runtime, you can spin up a local chain and produce blocks. This process will start your local parachain using the Polkadot Omni Node and allow you to interact with it. You'll first need to generate a chain specification that defines your network's identity, initial connections, and genesis state, providing the foundational configuration for how your nodes connect and what initial state they agree upon.

Follow these steps to launch your node in development mode:

1. Generate the chain specification file of your parachain:

    ```bash
    chain-spec-builder create -t development \
    --relay-chain paseo \
    --para-id 1000 \
    --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
    named-preset development
    ```

    !!! note
        The `--relay-chain` flag records the identifier of the relay chain the parachain is meant to connect to. When you run the parachain against a relay chain, set it to match that relay chain (for local development, the relay chain spec you start with [Zombienet](/polkadot-sdk/testing/run-a-parachain-network/)).

2. Start the Omni Node with the generated chain spec. You'll start it in development mode (without a relay chain config), producing and finalizing blocks:

    ```bash
    polkadot-omni-node --chain ./chain_spec.json --dev
    ```

    The `--dev` option does the following:

    - Deletes all active data (keys, blockchain database, networking information) when stopped.
    - Ensures a clean working state each time you restart the node.

    For the other flags `polkadot-omni-node` accepts, see the Omni Node [Command-Line Reference](/tools/omninode/#command-line-reference).

3. Verify that your node is running by reviewing the terminal output. You should see log messages indicating block production and finalization.

4. Confirm that your blockchain is producing new blocks by checking if the number after `finalized` is increasing in the output.

The details of the log output will be explored in a later tutorial. For now, knowing that your node is running and producing blocks is sufficient.

## Interact with the Node

When running the template node, it's accessible by default at `ws://localhost:9944`. You can connect to it with any client that supports Polkadot SDK chains. The following steps use [Polkadot.js Apps](https://polkadot.js.org/apps/#/explorer), a community-maintained web interface, as an example:

1. Open [Polkadot.js Apps](https://polkadot.js.org/apps/#/explorer) in your web browser and click the network icon (which should be the Polkadot logo) in the top left corner:
    
    ![](/images/parachains/launch-a-parachain/set-up-the-parachain-template/parachain-template-01.webp)

2. Connect to your local node:

    1. Scroll to the bottom and select **Development**.
    2. Choose **Custom**.
    3. Enter `ws://localhost:9944` in the **custom endpoint** input field.
    4. Click the **Switch** button.
    
    ![](/images/parachains/launch-a-parachain/set-up-the-parachain-template/parachain-template-02.webp)

3. Once connected, you should see **parachain-template-runtime** in the top left corner, with the interface displaying information about your local blockchain.
    
    ![](/images/parachains/launch-a-parachain/set-up-the-parachain-template/parachain-template-03.webp)

You are now connected to your local node and can explore blocks, submit transactions, and interact with your blockchain's features. For guidance on using Polkadot.js Apps, see the [Polkadot.js guides](https://wiki.polkadot.com/general/polkadotjs/) on the Polkadot Wiki.

## Stop the Node

When you're done exploring your local node, you can stop it to remove any state changes you've made. Since you started the node with the `--dev` option, stopping the node will purge all persistent block data, allowing you to start fresh the next time.

To stop the local node:

1. Return to the terminal window where the node output is displayed.
2. Press `Control-C` to stop the running process.
3. Verify that your terminal returns to the prompt in the `parachain-template` directory.

## Prepare for a Live Network

Running on a live relay chain uses the artifacts you produce locally:

- **Wasm runtime**: The `parachain_template_runtime.compact.compressed.wasm` file you built in [Compile the Runtime](#compile-the-runtime).
- **Chain specification**: The `chain_spec.json` file you generated with `chain-spec-builder`. For a live network, generate it with the `--relay-chain` value and parachain ID of your target relay chain.
- **Collator keys**: The node key and session keys your collator uses. See [Run a Block-Producing Collator](/nodes/run-a-collator/) for how to generate them.

Registering a parachain and obtaining coretime are network processes. See [Deploy on Polkadot](https://developers.polkadot.com/parachains/launch-a-parachain/deploy-to-polkadot/) on the Polkadot Developer Docs for those steps.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge tutorial">Tutorial</span> __Add Existing Pallets__

    ---

    Add pre-built FRAME pallets to the template runtime and test them locally.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/customize-runtime/add-existing-pallets/)

-   <span class="badge tutorial">Tutorial</span> __Run a Parachain Network__

    ---

    Run the template as a parachain alongside a local relay chain using Zombienet.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/testing/run-a-parachain-network/)

</div>