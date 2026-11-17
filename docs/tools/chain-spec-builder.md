---
title: Chain Spec Builder
description: Use chain-spec-builder to generate, inspect, verify, and convert chain specifications for Polkadot SDK runtimes from a compiled runtime Wasm blob.
categories: Polkadot SDK, Tooling
---

# Chain Spec Builder

## Introduction

`chain-spec-builder` is a command-line utility from the Polkadot SDK that builds chain specification (chain spec) files for any runtime that implements the [`GenesisBuilder`](https://docs.rs/sp-genesis-builder/latest/sp_genesis_builder/trait.GenesisBuilder.html) runtime API. A chain spec is the JSON file a node loads to learn a chain's name, identifier, properties, boot nodes, runtime code, and genesis state.

Because the tool reads the genesis presets that the runtime itself exposes, it works with a compiled runtime Wasm blob and does not need a node binary that embeds the runtime. This makes it the usual companion to the [Polkadot Omni Node](omninode.md), which starts a parachain node from a chain spec file.

This page covers installing `chain-spec-builder`, its subcommands and flags, and producing plain and raw chain specs for a runtime built from the parachain template. The commands and flags are taken from the tool's [CLI source](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/utils/chain-spec-builder/src/lib.rs) and [README](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/utils/chain-spec-builder/README.md) at the `{{dependencies.repositories.polkadot_sdk.version}}` release. The [crate documentation on docs.rs](https://docs.rs/staging-chain-spec-builder/latest/staging_chain_spec_builder/) has the same reference in rustdoc form.

## Install Chain Spec Builder

Download the pre-built `chain-spec-builder` binary from the [Polkadot SDK release](https://github.com/paritytech/polkadot-sdk/releases/tag/{{dependencies.repositories.polkadot_sdk.version}}):

=== "macOS"

    ```bash
    curl -L -o chain-spec-builder https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk.version}}/chain-spec-builder-aarch64-apple-darwin
    chmod +x chain-spec-builder
    sudo mv chain-spec-builder /usr/local/bin/
    ```

=== "Ubuntu"

    ```bash
    curl -L -o chain-spec-builder https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk.version}}/chain-spec-builder
    chmod +x chain-spec-builder
    sudo mv chain-spec-builder /usr/local/bin/
    ```

Alternatively, install it from source with `cargo`. The binary is published on crates.io as [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder) because of a name conflict, and the installed command is still `chain-spec-builder`:

```bash
cargo install --locked staging-chain-spec-builder
```

!!! note
    The [Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md) tutorial pins `chain-spec-builder` to version `{{dependencies.repositories.polkadot_sdk_parachain_template.subdependencies.staging_chain_spec_builder_version}}`, the version that matches the template's Polkadot SDK dependencies. The subcommands and flags on this page are the same in that version.

To confirm the installation, run:

```bash
chain-spec-builder --version
```

The `polkadot-omni-node` binary also bundles the same tool as a subcommand, so `polkadot-omni-node chain-spec-builder` accepts the same subcommands and flags as the standalone binary.

## Command Structure

Every invocation takes the following form:

```bash
chain-spec-builder [-c INSERT_OUTPUT_PATH] INSERT_SUBCOMMAND [INSERT_SUBCOMMAND_ARGS]
```

The only top-level option is `-c` (`--chain-spec-path`), the path where commands that produce a chain spec write their output. It defaults to `./chain_spec.json`. Because it belongs to the top-level command, place it before the subcommand name.

The tool provides the following subcommands:

| Subcommand | Description |
|---|---|
| `create` | Creates a new chain spec by calling the genesis builder API of the provided runtime Wasm blob. |
| `list-presets` | Prints the names of the genesis config presets the runtime provides. |
| `display-preset` | Prints a genesis config preset. Without `--preset-name`, prints the default preset. |
| `verify` | Checks that a chain spec can be converted to raw format, which surfaces uninitialized or invalid genesis config fields. |
| `convert-to-raw` | Converts a plain chain spec into raw format. |
| `update-code` | Replaces the runtime code stored in a chain spec. Supports plain and raw chain specs. |
| `add-code-substitute` | Adds an entry to the chain spec's `codeSubstitute` object so that nodes use the given runtime code from a given block height. |

Commands that produce a chain spec (`create`, `convert-to-raw`, `update-code`, and `add-code-substitute`) never modify their input file in place. They write the result to the `-c` path.

## Create a Chain Spec

The `create` subcommand builds a chain spec from a runtime Wasm blob. It takes the following options:

| Option | Default | Description |
|---|---|---|
| `-r`, `--runtime` | Required | Path to the runtime Wasm blob. `--runtime-wasm-path` is accepted as an alias. |
| `-n`, `--chain-name` | `Custom` | The chain name. |
| `-i`, `--chain-id` | `custom` | The chain identifier. |
| `-t` | `live` | The chain type: `development`, `local`, or `live`. This option has no long form. |
| `-c`, `--relay-chain` | None | The relay chain the parachain connects to. Stored in the chain spec's `relay_chain` field. |
| `-p`, `--para-id` | None | The parachain ID. Stored in the chain spec's `para_id` field. Requires `--relay-chain`. |
| `-s`, `--raw-storage` | `false` | Export the chain spec in raw format. |
| `-v`, `--verify` | `false` | Build the raw genesis storage silently to check the genesis config, then write the plain chain spec. |
| `--properties` | `tokenSymbol=UNIT,tokenDecimals=12` | Chain properties as comma-separated `KEY=VALUE` pairs. Can be passed more than once. |

The short form `-c` means `--chain-spec-path` before the subcommand and `--relay-chain` after `create`. Use the long forms if the distinction is unclear in a script.

After the options, `create` takes one of the following genesis actions, which decide where the genesis config comes from:

- **`named-preset INSERT_PRESET_NAME`**: Uses a named genesis config preset that the runtime provides.
- **`patch INSERT_PATCH_PATH`**: Applies a JSON patch file on top of the runtime's default genesis config.
- **`full INSERT_CONFIG_PATH`**: Uses a JSON file that sets every genesis config field. No defaults are used.
- **`default`**: Uses the runtime's default genesis config. The default config may not be valid for every runtime; for example, it may lack session keys.

### List the Runtime's Presets

Before creating a chain spec, list the presets your runtime provides. The following example uses the runtime built in [Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md):

```bash
chain-spec-builder list-presets \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm
```

The command prints a JSON object with a `presets` array. The parachain template runtime defines the `development` and `local_testnet` presets in its [`genesis_config_presets.rs`](https://github.com/paritytech/polkadot-sdk-parachain-template/blob/{{dependencies.repositories.polkadot_sdk_parachain_template.version}}/runtime/src/genesis_config_presets.rs) file.

To print the genesis config a preset produces, use `display-preset`:

```bash
chain-spec-builder display-preset \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
--preset-name development
```

### Generate a Plain Chain Spec

A plain chain spec stores the genesis config as human-readable JSON, so you can review and edit it. The following command generates a development chain spec for the parachain template runtime. It is the same command the parachain tutorials in these docs use:

```bash
chain-spec-builder create -t development \
--relay-chain paseo \
--para-id 1000 \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
named-preset development
```

The command writes `chain_spec.json` in the current directory. The `--relay-chain` value records which relay chain the parachain is meant to connect to; set it to match the relay chain you run against. The `--para-id` value is stored for nodes to read when the runtime does not implement the [`GetParachainInfo`](https://paritytech.github.io/polkadot-sdk/master/cumulus_primitives_core/trait.GetParachainInfo.html) runtime API. The parachain template implements that API, and its presets set the parachain ID to `1000`.

To set chain properties such as the token symbol, add `--properties`:

```bash
chain-spec-builder create -t development \
--relay-chain paseo \
--para-id 1000 \
--properties tokenSymbol=UNIT,tokenDecimals=12,ss58Format=42 \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
named-preset development
```

### Customize Genesis with a Patch

To change only some genesis values, write a patch file that contains the keys to override, and pass it to `create patch`. Keys you leave out keep their default values. The following patch, adapted from the example in the tool's README, sets the genesis balances of two accounts and the sudo key for the parachain template runtime:

```json title="patch.json"
{
  "balances": {
    "balances": [
      ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000],
      ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000]
    ]
  },
  "sudo": {
    "key": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
  }
}
```

```bash
chain-spec-builder create -t development \
--relay-chain paseo \
--para-id 1000 \
--runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
patch patch.json
```

The keys in a patch or full config file must match the runtime's genesis config. Run `display-preset` to see the structure your runtime expects.

## Produce a Raw Chain Spec

A raw chain spec stores the genesis state as encoded storage keys and values instead of a JSON genesis config. Nodes on the same network must share the same raw genesis, so distribute the raw chain spec when other people run nodes for your chain.

You can produce a raw chain spec in either of the following ways:

- Pass `-s` (`--raw-storage`) to `create`:

    ```bash
    chain-spec-builder -c raw_chain_spec.json create -s -t development \
    --relay-chain paseo \
    --para-id 1000 \
    --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
    named-preset development
    ```

- Convert an existing plain chain spec with `convert-to-raw`:

    ```bash
    chain-spec-builder -c raw_chain_spec.json convert-to-raw chain_spec.json
    ```

Converting a plain chain spec lets you edit it first and then freeze the result. To check a plain chain spec without writing a raw one, run `verify`:

```bash
chain-spec-builder verify chain_spec.json
```

## Update an Existing Chain Spec

The following subcommands modify the runtime code in a chain spec and write the result to the `-c` path. Both work with plain and raw chain specs.

- Replace the runtime code with `update-code`:

    ```bash
    chain-spec-builder -c updated_chain_spec.json update-code chain_spec.json \
    ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm
    ```

- Add a code substitute with `add-code-substitute`, which takes the input chain spec, the runtime Wasm blob, and a block height:

    ```bash
    chain-spec-builder -c updated_chain_spec.json add-code-substitute chain_spec.json \
    INSERT_RUNTIME_WASM_PATH INSERT_BLOCK_HEIGHT
    ```

    A code substitute tells nodes to use the given runtime code at the given block height. It is meant for recovering a chain that cannot produce blocks because of a runtime bug. For a parachain, the validation code registered on the relay chain must also be updated, or the relay chain rejects blocks built with the substituted runtime.

## Use the Chain Spec

Start a node with the generated chain spec by passing it to the `--chain` flag of the [Polkadot Omni Node](omninode.md):

```bash
polkadot-omni-node --chain ./chain_spec.json --dev
```

To run the parachain with a local relay chain, reference the chain spec from a [Zombienet](zombienet.md) network configuration, as shown in [Run a Parachain Network](/polkadot-sdk/testing/run-a-parachain-network.md). That guide also uses `chain-spec-builder` with the `local_testnet` preset to build the relay chain spec.

## Where to Go Next

- [Polkadot Omni Node](omninode.md): Run a parachain node from the chain spec you generated.
- [Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md): Build the parachain template runtime and start it locally.
- [Chain Spec Genesis reference](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/chain_spec_genesis/index.html): Read how genesis presets and the `GenesisBuilder` API work in the Polkadot SDK.
