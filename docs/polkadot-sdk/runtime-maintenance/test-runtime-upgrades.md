---
title: Test Runtime Upgrades with try-runtime
description: Dry-run a runtime upgrade with try-runtime-cli against a node snapshot to check migrations, try-state invariants, weight, and idempotency.
short_description: Dry-run runtime upgrades and migrations before you ship them.
categories: Polkadot SDK
---

# Test Runtime Upgrades with try-runtime

## Introduction

A runtime upgrade runs its storage migrations exactly once, on real chain state, with no way to undo them. [`try-runtime-cli`](https://github.com/paritytech/try-runtime-cli) lets you rehearse that moment. It loads the state of a chain, replaces the on-chain runtime with the Wasm blob you built, and calls a dedicated runtime API that runs your migrations and their test hooks without producing a real block.

This guide explains what `try-runtime` checks, how to enable the `try-runtime` feature in a runtime, how to install and run `try-runtime-cli` against a local node or a snapshot, and how to add the check to continuous integration (CI). The examples use the [Polkadot SDK parachain template](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain) runtime.

## Prerequisites

Before you begin, make sure you have the following:

- A Polkadot SDK runtime you can build, such as the one from the [Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md) tutorial.
- A local node running the current version of that runtime, as described in [Runtime Upgrades](/polkadot-sdk/runtime-maintenance/runtime-upgrades.md#start-your-chain).
- A runtime change to test, typically a [storage migration](/polkadot-sdk/runtime-maintenance/storage-migrations.md) and a bumped `spec_version`.

## How try-runtime Works

Every `try-runtime` command follows the same pattern, described in the [`try-runtime-cli` README](https://github.com/paritytech/try-runtime-cli#background-knowledge):

1. Load chain state, either by downloading it from a running node (`live`) or by reading a snapshot file (`snap`).
2. Overwrite the `:code` storage key with the Wasm runtime you pass through `--runtime`.
3. Call a runtime API on top of that state. The `on-runtime-upgrade` subcommand calls `TryRuntime_on_runtime_upgrade`.

The `TryRuntime` runtime API only exists in a runtime compiled with the `try-runtime` Cargo feature. The feature also compiles in the test-only hooks described in the next section. Never deploy a runtime built with this feature on a live chain.

## What the on-runtime-upgrade Command Checks

The [`on-runtime-upgrade` command](https://github.com/paritytech/try-runtime-cli/blob/{{ dependencies.repositories.try_runtime_cli.version }}/core/src/commands/on_runtime_upgrade/mod.rs) runs the following checks and exits with a non-zero status if any of them fails:

- **Runtime compatibility**: The `spec_name` of your runtime must match the one in the loaded state, and its `spec_version` must be greater than or equal to the existing one. The runtime must also be compiled with the `try-runtime` feature.
- **Migrations**: The runtime's [`Executive::try_runtime_upgrade`](https://paritytech.github.io/polkadot-sdk/master/frame_executive/struct.Executive.html#method.try_runtime_upgrade) runs every [`OnRuntimeUpgrade`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/trait.OnRuntimeUpgrade.html) implementation in the order the real upgrade uses: migrations passed to `Executive`, then `frame_system::Config::SingleBlockMigrations`, then each pallet's `on_runtime_upgrade` hook.
- **Pre- and post-upgrade hooks**: When enabled, each migration's `pre_upgrade` hook runs before it and its `post_upgrade` hook runs after it, receiving the bytes `pre_upgrade` returned. Neither hook is allowed to modify storage.
- **State decoding and invariants**: When any check is enabled, the runtime tries to decode every storage item of every pallet. When try-state checks are enabled, it also runs each pallet's `try_state` hook.
- **Idempotency**: The CLI runs the migrations a second time on the migrated state and compares the storage roots. A migration that changes storage again, or fails on the second run, is reported as not idempotent.
- **Weight and proof size**: The CLI runs the migrations again to measure them and warns when the consumed `ref_time` reaches 80% of the block maximum or the compressed proof of validity (PoV) exceeds 80% of 5 MB.
- **Multi-block migrations**: If the runtime uses [multi-block migrations](/polkadot-sdk/runtime-maintenance/multi-block-migrations.md), the CLI produces empty blocks until they finish, up to `--mbm-max-blocks`, and then runs `TryRuntime_on_runtime_upgrade` once more with the checks you selected.

The `--checks` flag selects which optional checks the runtime performs. It accepts the values of [`UpgradeCheckSelect`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/enum.UpgradeCheckSelect.html):

| Value | Runs |
|:---:|:---:|
| `none` | No optional checks |
| `pre-and-post` | `pre_upgrade` and `post_upgrade` hooks (default when `--checks` is absent) |
| `try-state` | `try_state` hooks |
| `all` | All of the above (default when `--checks` is passed without a value) |

!!! note
    When multi-block migration checks are enabled, which is the default, the first single-block pass runs without optional checks, and the selected checks run in the final pass after the multi-block migrations finish. Pass `--disable-mbm-checks` to skip multi-block migrations and run the selected checks in the first pass instead.

## Add Test Hooks to Your Code

The checks above are only as useful as the hooks you write. All of them are compiled only when the `try-runtime` feature is enabled.

### Pre- and Post-Upgrade Hooks

Implement [`pre_upgrade`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/trait.OnRuntimeUpgrade.html#method.pre_upgrade) and [`post_upgrade`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/trait.OnRuntimeUpgrade.html#method.post_upgrade) on each migration. A common pattern is to encode the old values in `pre_upgrade` and confirm in `post_upgrade` that each one was transformed as expected. The [Storage Migrations](/polkadot-sdk/runtime-maintenance/storage-migrations.md#migration-testing-hooks) guide covers these hooks and includes a complete example.

### Try-State Invariants

A [`try_state`](https://paritytech.github.io/polkadot-sdk/master/frame_support/traits/trait.Hooks.html#method.try_state) hook checks invariants that must always hold for a pallet's storage, such as a stored total matching the sum of individual entries. It must not modify storage. Declare it in the pallet's hooks block, following the same pattern as FRAME pallets such as [`pallet-alliance`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/alliance/src/lib.rs):

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    #[cfg(feature = "try-runtime")]
    fn try_state(_n: BlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
        // Return an error if any invariant of this pallet's storage does not hold.
        Ok(())
    }
}
```

`try-runtime` also runs these hooks in the `execute-block`, `follow-chain`, and `fast-forward` subcommands, so invariants you add here are checked well beyond upgrades.

## Enable the try-runtime Feature

Each crate in the runtime forwards a `try-runtime` feature to its dependencies. The parachain template already wires it, so you only need to replicate this pattern for pallets you add:

- In the pallet's `Cargo.toml`, add a `try-runtime` feature that enables the same feature on the FRAME dependency. The template pallet declares `try-runtime = ["frame/try-runtime"]`.
- In the runtime's `Cargo.toml`, add the pallet's feature to the runtime's `try-runtime` feature list:

    ```toml title="runtime/Cargo.toml"
    --8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/Cargo.toml:88:92'
    ```

- In the runtime APIs, implement [`frame_try_runtime::TryRuntime`](https://paritytech.github.io/polkadot-sdk/master/frame_try_runtime/inner/trait.TryRuntime.html) behind the feature, delegating to `Executive`:

    ```rust title="runtime/src/apis.rs"
    --8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/src/apis.rs:240:259'
    ```

The standalone [`polkadot-sdk-parachain-template`]({{dependencies.repositories.polkadot_sdk_parachain_template.repository_url}}) repository uses the same feature list and API implementation.

Build the runtime with the feature enabled from the root of your template project:

```bash
cargo build --release --features try-runtime
```

The Wasm runtime to test is written to `./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm`.

## Install try-runtime-cli

`try-runtime-cli` is a separate binary from your node. Install a tagged version with Cargo, as the README recommends for reproducible setups:

```bash
cargo install --git https://github.com/paritytech/try-runtime-cli --tag {{ dependencies.repositories.try_runtime_cli.version }} --locked
```

Each [GitHub release](https://github.com/paritytech/try-runtime-cli/releases/tag/{{ dependencies.repositories.try_runtime_cli.version }}) also publishes prebuilt `try-runtime-x86_64-unknown-linux-musl` and `try-runtime-aarch64-apple-darwin` binaries. Confirm the installation:

```bash
try-runtime --version
```

The CLI provides the following subcommands. Run `try-runtime INSERT_SUBCOMMAND --help` for the full flag list:

- **`on-runtime-upgrade`**: Runs all `OnRuntimeUpgrade` hooks of the given runtime and the checks described in this guide.
- **`execute-block`**: Re-executes a block on top of its parent state with the given runtime, optionally running `try_state` hooks.
- **`fast-forward`**: Runs the runtime upgrade, then produces a number of empty blocks while checking `try_state` invariants.
- **`offchain-worker`**: Re-executes the offchain worker code path of a block.
- **`follow-chain`**: Applies the finalized blocks of a running chain on top of the given runtime. It only works with a live node.
- **`create-snapshot`**: Downloads state from a node and writes it to a snapshot file.

## Run a Runtime Upgrade Check

The command line has two levels of flags. Shared flags such as `--runtime` go before the subcommand, command flags such as `--checks` go after it, and the state source (`live` or `snap`) comes last.

### Create a Snapshot

A snapshot makes runs repeatable and avoids downloading state on every run. With your local node running the current runtime, create one:

```bash
try-runtime create-snapshot --uri {{ networks.local.ws_url }} -- parachain-template.snap
```

Without `--at`, the CLI uses the latest finalized block. The snapshot keeps the runtime that was on-chain, which is expected: you replace it with `--runtime` in the next step.

### Test the Upgrade Against the Snapshot

Run the migrations of your new runtime on top of the snapshot:

```bash
try-runtime \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  on-runtime-upgrade \
  --blocktime 6000 \
  --checks=all \
  snap --path parachain-template.snap
```

The flags in this command are:

- **`--runtime`**: The path to the Wasm runtime compiled with `try-runtime`. The default, `existing`, keeps the code from the state source, which almost never includes the `TryRuntime` API.
- **`--blocktime`**: Your chain's block time in milliseconds, used to produce blocks while multi-block migrations run. The parachain template sets `MILLI_SECS_PER_BLOCK` to `6000`. The flag has no default, so `on-runtime-upgrade` requires it.
- **`--checks=all`**: Runs the `pre_upgrade`, `post_upgrade`, and `try_state` hooks.
- **`snap --path`**: Reads state from the snapshot file.

Set `RUST_LOG=runtime=debug,try-runtime=info` to see the log output of your migrations. The `try-runtime-cli` README recommends logging from migrations under a `runtime` target prefix, such as `runtime::my-pallet`.

### Test the Upgrade Against a Running Node

To test against the current state of a node instead of a snapshot, replace `snap` with `live`:

```bash
try-runtime \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  on-runtime-upgrade \
  --blocktime 6000 \
  --checks=all \
  live --uri {{ networks.local.ws_url }}
```

The `live` source accepts the following options:

- **`--uri`**: The node to download state from. Repeat the flag, or pass comma-separated URLs, to download from several nodes in parallel.
- **`--at`**: A block hash to load state at, instead of the latest finalized block. The node must still have that state, so older blocks require an archive node.
- **`--pallet`**: Only download the storage of the named pallets, which is much faster for large chains when your migration touches few pallets.

To rehearse an upgrade for a deployed chain, point `--uri` at a node of that network that you trust, or create a snapshot from it with `create-snapshot`. Downloading the full state of a large chain can take a long time.

### Other Useful Flags

The `on-runtime-upgrade` command also accepts the following flags:

- **`--disable-spec-version-check`**: Skips the requirement that the new `spec_version` is greater than or equal to the existing one.
- **`--disable-idempotency-checks`**: Skips running the migrations a second time.
- **`--print-storage-diff`**: When the idempotency check fails, prints the storage keys that changed between the two runs.
- **`--no-weight-warnings`**: Skips the weight and proof size analysis.
- **`--disable-mbm-checks`**: Skips running multi-block migrations to completion.
- **`--mbm-max-blocks`**: The maximum number of blocks all multi-block migrations may take before the check fails. Defaults to `600`.

The shared `--disable-spec-name-check` flag, passed before the subcommand, skips the `spec_name` comparison.

## Run try-runtime in CI

Run the upgrade check on every pull request that changes the runtime, so a broken migration fails before it can be released. A CI job needs the same three steps you ran locally:

1. Install a pinned `try-runtime-cli` version, either from a release binary or with `cargo install --tag`.
2. Build the runtime with `--features try-runtime`.
3. Run `on-runtime-upgrade` against a snapshot of the chain you plan to upgrade, and fail the job on a non-zero exit code.

For example, on a Linux runner:

```bash
curl -sL https://github.com/paritytech/try-runtime-cli/releases/download/{{ dependencies.repositories.try_runtime_cli.version }}/try-runtime-x86_64-unknown-linux-musl -o try-runtime
chmod +x ./try-runtime

cargo build --release --locked -p parachain-template-runtime --features try-runtime

./try-runtime \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  on-runtime-upgrade --checks=all --blocktime 6000 \
  snap -p snapshot.snap
```

Creating a snapshot of a large chain is slow, so cache it between runs and refresh it on a schedule. The Polkadot SDK's [`check-runtime-migration.yml`](https://github.com/paritytech/polkadot-sdk/blob/master/.github/workflows/check-runtime-migration.yml) workflow is a complete example: it creates snapshots in a separate job, caches them, and runs `on-runtime-upgrade` for each runtime with `--checks=all`.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Write Multi-Block Migrations__

    ---

    Spread large migrations across several blocks with `pallet-migrations` and test them with `try-runtime`.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/runtime-maintenance/multi-block-migrations.md)

-   <span class="badge guide">Guide</span> __Storage Migrations__

    ---

    Write single-block storage migrations with `pre_upgrade` and `post_upgrade` hooks.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/runtime-maintenance/storage-migrations.md)

-   <span class="badge tutorial">Tutorial</span> __Runtime Upgrades__

    ---

    Build a new runtime version and submit the upgrade to your local chain.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/runtime-maintenance/runtime-upgrades.md)

</div>
