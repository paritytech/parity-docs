---
title: Write Multi-Block Migrations
description: Spread large storage migrations across several blocks with pallet-migrations and the SteppedMigration trait, then track, recover, and test them.
short_description: Migrate large storage safely across multiple blocks.
categories: Polkadot SDK
---

# Write Multi-Block Migrations

## Introduction

A [single-block storage migration](/polkadot-sdk/runtime-maintenance/storage-migrations.md) must finish inside the block that enacts the runtime upgrade. When a migration touches more data than fits in one block, it cannot finish there, and on a parachain an overweight block can stall the chain. Multi-block migrations (MBMs) solve this by splitting the work into steps that run over as many blocks as needed, keeping their progress in storage between steps.

This guide shows how to add [`pallet-migrations`](https://paritytech.github.io/polkadot-sdk/master/pallet_migrations/index.html) to a runtime, implement the [`SteppedMigration`](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/trait.SteppedMigration.html) trait, follow a migration's progress, handle failures, and test the result. The code excerpts come from [`pallet-example-mbm`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations) and the Polkadot SDK's [`kitchensink-runtime`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs) at the `{{dependencies.repositories.polkadot_sdk.version}}` release.

## Prerequisites

Before you begin, make sure you have the following:

- A Polkadot SDK runtime you can build and upgrade, as described in [Runtime Upgrades](/polkadot-sdk/runtime-maintenance/runtime-upgrades.md).
- Familiarity with storage versions and single-block migrations from [Storage Migrations](/polkadot-sdk/runtime-maintenance/storage-migrations.md).

## When to Use Multi-Block Migrations

Use a multi-block migration when you cannot guarantee that a migration fits within a single block's weight and proof size limits. Typical cases include the following:

- Migrating a storage map whose number of entries is unbounded or grows with usage.
- Transforming nested or variable-length values whose weight is hard to predict.
- Running several heavy migrations in the same upgrade.

A single-block migration remains the simpler choice when the amount of data is small and bounded and you have measured its weight.

Multi-block migrations come with a cost for users of the chain. While they run, the runtime only accepts inherents, so signed and unsigned transactions are paused until the last migration completes. Design each step to make as much progress as its weight allows so that the pause stays short.

## How Multi-Block Migrations Run

Multi-block migrations are driven by the `MultiBlockMigrator` that you configure in `frame_system`, normally `pallet-migrations`. The process works as follows:

1. On a runtime upgrade, `pallet-migrations` reads the migrations configured in its `Migrations` type, sets its `Cursor` storage item to the first one, and emits `UpgradeStarted`.
2. In each following block, [`frame_executive`](https://paritytech.github.io/polkadot-sdk/master/frame_executive/index.html) calls the migrator right after the inherents are applied. The migrator calls the current migration's `step` function with a [`WeightMeter`](https://paritytech.github.io/polkadot-sdk/master/frame_support/weights/struct.WeightMeter.html) limited to `MaxServiceWeight`. Each migration advances at most one step per block.
3. A step returns a new cursor while work remains, or `None` when the migration is complete. Completed migrations are recorded in the `Historic` storage map and are skipped in future upgrades.
4. When no migrations remain, the pallet clears `Cursor` and emits `UpgradeCompleted`.

While migrations are ongoing, `frame_executive` only allows inherents in blocks and skips the `on_idle` and `on_poll` hooks, and `frame_system` rejects a new runtime code upgrade. Only change the `Migrations` list in a runtime upgrade after all previous migrations have completed.

## Add pallet-migrations to a Runtime

The following steps add `pallet-migrations` to a runtime built on the [parachain template](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain), which imports pallets through the `polkadot-sdk` umbrella crate.

1. In `runtime/Cargo.toml`, add `pallet-migrations` to the features of the `polkadot-sdk` dependency. The umbrella crate forwards the `std`, `runtime-benchmarks`, and `try-runtime` features to it.

    ```toml title="runtime/Cargo.toml"
    polkadot-sdk = { workspace = true, default-features = false, features = [
        # ...existing features
        "pallet-migrations",
        "runtime",
    ] }
    ```

2. Configure the pallet. The following configuration from `kitchensink-runtime` dedicates up to 80% of the maximum block weight to migrations:

    ```rust title="runtime/src/configs/mod.rs"
    --8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs:2410:2427'
    ```

    The `Migrations` type is set to `()` here, and `FailedMigrationHandler` is set to `FreezeChainOnFailedMigration`, which you should review before you deploy, as described in [Handle Failures](#handle-failures). You replace `Migrations` with your migrations in [Register the Migration](#register-the-migration). With the `runtime-benchmarks` feature, `pallet-migrations` requires the mocked migrations shown so that its benchmarks succeed.

3. Add the pallet to the runtime in `runtime/src/lib.rs`, choosing an unused pallet index:

    ```rust title="runtime/src/lib.rs"
    #[runtime::pallet_index(INSERT_PALLET_INDEX)]
    pub type MultiBlockMigrations = pallet_migrations;
    ```

4. Set the pallet as the multi-block migrator in the `frame_system` configuration:

    ```rust title="runtime/src/configs/mod.rs"
    impl frame_system::Config for Runtime {
        // ...existing configuration
        type MultiBlockMigrator = MultiBlockMigrations;
    }
    ```

    The pallet's `integrity_test` hook asserts that `MultiBlockMigrator` reports the state of `pallet-migrations`, so the runtime's integrity tests catch a missing or wrong setting.

The [`Config`](https://paritytech.github.io/polkadot-sdk/master/pallet_migrations/pallet/trait.Config.html) items of `pallet-migrations` are the following:

- **`RuntimeEvent`**: The runtime's overarching event type.
- **`Migrations`**: A tuple of the multi-block migrations to run, each implementing `SteppedMigration`.
- **`CursorMaxLen`**: The maximum encoded length of a migration cursor. The `integrity_test` checks that every configured migration's cursor fits.
- **`IdentifierMaxLen`**: The maximum encoded length of a migration identifier, checked the same way.
- **`MigrationStatusHandler`**: A hook notified when migrations start and complete, for example to pause other subsystems. Use `()` for none.
- **`FailedMigrationHandler`**: Decides what happens when a migration fails. See [Handle Failures](#handle-failures).
- **`MaxServiceWeight`**: The maximum weight spent on migrations in each block. The `integrity_test` checks that it does not exceed the maximum block weight.
- **`WeightInfo`**: Weights for the pallet's own logic and calls.

## Implement a Multi-Block Migration

The `pallet-example-mbm` pallet migrates the values of a `StorageMap` from `u32` to `u64`. The migration lives in the pallet's `migrations/v1` module, following the layout described in [Storage Migrations](/polkadot-sdk/runtime-maintenance/storage-migrations.md#migration-organization).

### Define a Migration Identifier

Every multi-block migration needs a unique identifier. `pallet-migrations` stores the identifiers of completed migrations in `Historic` and skips any migration whose identifier it has already seen, so two migrations with the same identifier are treated as the same migration.

The example defines a constant prefix for the pallet instead of deriving it from the crate name, so renaming the crate never causes old migrations to run again:

```rust title="migrations/mod.rs"
--8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/migrations/mod.rs:23:29'
```

The migration combines this prefix with the storage versions it migrates between using the [`MigrationId`](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/struct.MigrationId.html) helper type.

### Implement the SteppedMigration Trait

The migration reads values through a [`storage_alias`](https://paritytech.github.io/polkadot-sdk/master/frame_support/attr.storage_alias.html) of the old `u32` map and writes them to the new `u64` map. Because both maps use the same key hasher, the last processed key serves as the cursor:

```rust title="migrations/v1/mod.rs"
--8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/migrations/v1/mod.rs:60:125'
```

The trait has the following required items:

- **`Cursor`**: The type that records the migration's progress between steps. It must implement `FullCodec` and `MaxEncodedLen`, and its maximum encoded length must fit in `CursorMaxLen`.
- **`Identifier`**: The type of the migration's unique identifier, with the same codec bounds. Its maximum encoded length must fit in `IdentifierMaxLen`.
- **`id`**: Returns the migration's identifier.
- **`step`**: Performs as much work as the `WeightMeter` allows. It receives `None` on the first call and the previously returned cursor afterward. It returns `Ok(Some(cursor))` while work remains and `Ok(None)` when the migration is complete.

The `step` function must never panic and must never consume more weight than the meter allows. The example checks the remaining weight against the benchmarked cost of one iteration, `W::step()`, before every iteration. Benchmark that cost for your own migration, as the example does in its [`benchmarks.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/migrations/v1/benchmarks.rs).

The trait also provides the following optional items with default implementations:

- **`max_steps`**: The maximum number of steps the migration may take. `pallet-migrations` treats a migration that exceeds it as failed. The default, `None`, sets no limit, but the pallet only guarantees progress if you set a sensible limit.
- **`migrating_prefixes`**: The storage prefixes the migration changes, exposed through the pallet's status so that tools know which state may not be readable during the migration. Defaults to `None`.
- **`pre_upgrade`** and **`post_upgrade`**: Test hooks compiled with the `try-runtime` feature. See [Test with try-runtime](#test-with-try-runtime).

`pallet-migrations` calls `step` through `transactional_step`, which rolls back the storage changes of a step that returns an error.

### Register the Migration

Add the migration to the `Migrations` tuple of `pallet_migrations::Config`. The example's mock runtime registers it as follows:

```rust title="mock.rs"
--8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/mock.rs:47:53'
```

`pallet-migrations` runs the migrations in tuple order, one after another. A production runtime registers its multi-block migrations the same way. For example, [Asset Hub Westend](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs) lists migrations from several pallets in its `Migrations` tuple.

## Track Migration Status

`pallet-migrations` emits the following [events](https://paritytech.github.io/polkadot-sdk/master/pallet_migrations/pallet/enum.Event.html) during an upgrade:

- **`UpgradeStarted`**: Migrations started. The `migrations` field holds how many the upgrade contains.
- **`MigrationAdvanced`**: A migration completed a step but is not finished. The `index` field is its position in the `Migrations` tuple, and `took` is the number of blocks it has run.
- **`MigrationCompleted`**: A migration finished.
- **`MigrationSkipped`**: A migration was skipped because its identifier is already in `Historic`.
- **`MigrationFailed`**: A migration failed, which fails the whole upgrade.
- **`UpgradeCompleted`**: All migrations finished.
- **`UpgradeFailed`**: The upgrade failed and needs intervention.
- **`HistoricCleared`**: Entries were removed from `Historic` by the `clear_historic` call.

You can also read the `Cursor` storage item directly. It is empty when no migrations are running, `Active` with the current migration index and inner cursor while they run, and `Stuck` after a failure. The pallet also exposes view functions such as `status`, which returns whether migrations are ongoing, progress information for the current migration, and its affected storage prefixes.

## Handle Failures

A step can return one of the following [`SteppedMigrationError`](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/enum.SteppedMigrationError.html) variants:

- **`InsufficientWeight`**: The meter does not have the `required` weight for any progress. If the migration was the first to run in the block, or `required` exceeds the meter's limit, the migration fails. Otherwise, the pallet retries it in the next block.
- **`InvalidCursor`**: The migration cannot decode its cursor. The migration fails.
- **`Failed`**: The migration hit a permanent error. The migration fails.

When a migration fails, `pallet-migrations` emits `MigrationFailed` and `UpgradeFailed` and calls the configured `FailedMigrationHandler`. Failed migrations are not added to `Historic`. The handler returns one of the following [`FailedMigrationHandling`](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/enum.FailedMigrationHandling.html) values:

- **`KeepStuck`**: Sets the cursor to `Stuck`. Because the cursor is not empty, the runtime keeps accepting only inherents.
- **`ForceUnstuck`**: Clears the cursor and resumes transaction processing without completing the upgrade, exposing any half-migrated state to users.
- **`Ignore`**: Leaves the cursor unchanged so that the handler can manage it. A handler that returns this value without acting keeps failing in every block.

`frame_support` provides the following handlers:

- **`FreezeChainOnFailedMigration`**: Always returns `KeepStuck`. Its [reference documentation](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/struct.FreezeChainOnFailedMigration.html) warns that it is not a sane default because it prevents governance intervention.
- **`ForceUnstuckOnFailedMigration`**: Always returns `ForceUnstuck`. Its [reference documentation](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/struct.ForceUnstuckOnFailedMigration.html) describes this as equivalent to how a faulty single-block migration is handled.
- **`EnterSafeModeOnFailedMigration`**: Enters or extends [safe mode](https://paritytech.github.io/polkadot-sdk/master/frame_support/migrations/struct.EnterSafeModeOnFailedMigration.html) and returns `KeepStuck`, or falls back to another handler if safe mode cannot be entered.

No handler is safe for every chain. The `pallet-migrations` documentation advises choosing one that lets governance act while preventing other transactions from interacting with the inconsistent state, for example by combining it with `pallet-safe-mode` or `pallet-tx-pause`. Monitor for the `UpgradeFailed` event so that you learn about failures immediately.

The pallet provides the following root-only calls for emergencies and maintenance:

- **`force_set_cursor`**: Sets the cursor to any value, which can start, stop, or move the migration process.
- **`force_set_active_cursor`**: Sets an active cursor with a given migration index and inner cursor.
- **`force_onboard_mbms`**: Starts the configured migrations as if a runtime upgrade had happened. Requires that no migrations are running.
- **`clear_historic`**: Removes entries from `Historic`, either specific identifiers or a bounded batch.

## Test Multi-Block Migrations

Test a multi-block migration at two levels: unit tests against a mock runtime, and a dry run against real chain state with `try-runtime`.

### Write Unit Tests

In a mock runtime, configure `pallet-migrations` and set it as the `MultiBlockMigrator`, as in the example's [`mock.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/mock.rs). Because `frame_executive` does not run in unit tests, the mock calls the migrator's `step` function itself on every block:

```rust title="mock.rs"
--8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/mock.rs:79:88'
```

The example's test fills the old map, gives the migrator enough weight for exactly 16 iterations per block, triggers the upgrade with `on_runtime_upgrade`, and checks after every block that exactly 16 more values were migrated:

```rust title="migrations/v1/tests.rs"
--8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/migrations/v1/tests.rs:34:79'
```

### Test with try-runtime

`SteppedMigration` has its own `pre_upgrade` and `post_upgrade` hooks. With the `try-runtime` feature enabled, `pallet-migrations` calls `pre_upgrade` before a migration's first step and `post_upgrade`, with the bytes `pre_upgrade` returned, after its last step. The example captures the old map and verifies every migrated value:

```rust title="migrations/v1/mod.rs"
--8<-- 'https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/tags/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/examples/multi-block-migrations/src/migrations/v1/mod.rs:127:160'
```

With the `try-runtime` feature enabled, `pallet-migrations` panics instead of calling the `FailedMigrationHandler` when a migration fails, so failures surface immediately in tests.

To dry-run the migrations against a snapshot or a running node, use the `on-runtime-upgrade` command of `try-runtime-cli`. By default, it produces blocks until all multi-block migrations finish and fails if they take longer than `--mbm-max-blocks` blocks. Set `--blocktime` to your chain's block time in milliseconds:

```bash
try-runtime \
  --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
  on-runtime-upgrade \
  --blocktime 6000 \
  --checks=all \
  snap --path parachain-template.snap
```

See [Test Runtime Upgrades with try-runtime](/polkadot-sdk/runtime-maintenance/test-runtime-upgrades.md) for how to build the runtime with the `try-runtime` feature, create a snapshot, and run this check in CI.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Test Runtime Upgrades with try-runtime__

    ---

    Dry-run your runtime upgrade and its migrations against a snapshot of real chain state.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/runtime-maintenance/test-runtime-upgrades.md)

-   <span class="badge guide">Guide</span> __Storage Migrations__

    ---

    Write single-block migrations with `VersionedMigration` for small, bounded storage changes.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/runtime-maintenance/storage-migrations.md)

-   <span class="badge tutorial">Tutorial</span> __Runtime Upgrades__

    ---

    Build a new runtime version and submit the upgrade to your local chain.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/runtime-maintenance/runtime-upgrades.md)

</div>
