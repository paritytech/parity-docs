---
title: Enable the Statement Store
description: Add the statement store to a Polkadot SDK chain, grant per-account allowances from the runtime, and run polkadot-omni-node locally with the store enabled.
categories: Apps, Reference
---

# Enable the Statement Store

## Introduction

The Statement Store is Polkadot SDK software that any chain can run: a node-side store of signed statements, gossiped between nodes, with permissions read from the chain's state. This page shows how to add it to your own Polkadot SDK chain and run a node with it enabled locally. For what statements are and how Products use them, see the [Statement Store overview](/apps/reference/infrastructure/statement-store/index.md).

This page targets the Polkadot SDK release `{{dependencies.repositories.polkadot_sdk.version}}`. In that release, enabling the store involves two pieces:

- **The node**: Runs the store itself, implemented in [`sc-statement-store`](https://paritytech.github.io/polkadot-sdk/master/sc_statement_store/index.html). `polkadot-omni-node` includes it behind a command-line flag.
- **The runtime**: Holds a per-account allowance in its state. The node accepts a statement only from an account with an allowance, so the runtime decides who can publish.

## How the Node Validates a Statement

The store validates each submission on the node. It verifies the statement's proof and signature, then reads the signer's allowance directly from chain state at the best block. Reading the allowance is a storage read, not a runtime API call, so the runtime does not implement a statement validation API.

The allowance is a [`StatementAllowance`](https://paritytech.github.io/polkadot-sdk/master/sp_statement_store/struct.StatementAllowance.html) value from `sp-statement-store`, with two fields:

- **`max_count`**: The maximum number of statements the account may hold in the store, as a `u32`.
- **`max_size`**: The maximum total size of those statements in bytes, as a `u32`.

It is stored SCALE-encoded under an unhashed key: the prefix `:statement_allowance:` followed by the raw account ID bytes. A submission from an account with no allowance is rejected with `NoAllowance`. For how the two limits interact, see [Allowance](/apps/reference/infrastructure/statement-store/allowance.md).

## Grant Allowances from the Runtime

The [`sp-statement-store`](https://paritytech.github.io/polkadot-sdk/master/sp_statement_store/index.html) crate provides functions that read and write the allowance key from runtime code:

- **`increase_allowance_by(account_id, by)`**: Adds `by` to the account's allowance, saturating each field.
- **`decrease_allowance_by(account_id, by)`**: Subtracts `by` from the account's allowance and removes the entry once either field reaches zero.
- **`get_allowance(account_id)`**: Returns the account's current allowance, or the zero default.

The crate does not include a dispatchable call that grants allowances. Your runtime decides who receives one and when, for example in a privileged call of your own pallet or as part of another flow:

```rust
use sp_statement_store::{increase_allowance_by, StatementAllowance};

// Grant `who` room for 100 statements totaling up to 1,000,000 bytes.
increase_allowance_by(&who, StatementAllowance::new(100, 1_000_000));
```

The account ID passed to these functions must be the same bytes the node extracts from a statement's proof, which for an Sr25519 proof is the 32-byte public key.

### Optional: Emit Statements from the Runtime

[`pallet-statement`](https://paritytech.github.io/polkadot-sdk/master/pallet_statement/index.html) is only needed if runtime code should publish statements itself. Its `submit_statement(account, statement)` function deposits a `NewStatement` event, and the pallet's offchain worker submits every such statement to the node's store. Its configuration in `{{dependencies.repositories.polkadot_sdk.version}}` takes `RuntimeEvent`, a `Currency`, and the constants `StatementCost`, `ByteCost`, `MinAllowedStatements`, `MaxAllowedStatements`, `MinAllowedBytes`, and `MaxAllowedBytes`. The [Substrate kitchensink runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs) shows a complete configuration.

## Enable the Store on the Node

`polkadot-omni-node` builds the store, registers its gossip protocol, and exposes its JSON-RPC methods when you pass `--enable-statement-store`. This works for a node that follows a relay chain and for a node in development mode. The following flags tune the store and apply only when it is enabled:

| Flag | Default | Purpose |
|:-----|:--------|:--------|
| `--enable-statement-store` | Off | Enable the statement store. |
| `--statement-network-workers` | `1` | Number of concurrent workers that validate statements received from the network. |
| `--statement-rate-limit` | `50000` | Maximum statements per second per peer before rate limiting applies. |
| `--statement-store-max-total-statements` | `4194304` | Maximum number of statements the store holds before evicting lower-priority ones. |
| `--statement-store-max-total-size` | `2147483648` (2 GiB) | Maximum total data size, in bytes, before evicting lower-priority statements. |
| `--statement-store-purge-after-sec` | `172800` (48 hours) | Seconds during which a removed statement cannot be added back. |

With the store enabled, the node serves `statement_submit`, which takes a SCALE-encoded statement, and the `statement_subscribeStatement` subscription, which takes a topic filter. For more on Omni Node, see [Polkadot Omni Node](/tools/omninode.md).

If you build your own node instead of using Omni Node, wire `sc-statement-store` and the `sc-network-statement` gossip handler into your service. Omni Node's [`statement_store.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/polkadot-omni-node/lib/src/common/statement_store.rs) is a working reference.

## Run a Node with the Store Enabled Locally

Because validation happens on the node and reads only the allowance key, you can try the store with any runtime that has a sudo key, before adding allowance logic to your own runtime. The following steps use the parachain template:

1. Build the parachain template runtime and generate a development chain spec, as described in [Set Up the Polkadot SDK Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md). The template's development preset sets `//Alice` as the sudo key.

2. Start the node in development mode with the store enabled:

    ```bash
    polkadot-omni-node --chain ./chain_spec.json --dev --enable-statement-store
    ```

3. Give an account an allowance by writing the allowance key directly. With Polkadot.js Apps connected to `{{ networks.local.ws_url }}`, use **Developer** > **Sudo** to call `system.setStorage` with one item:

    - **Key**: `0x3a73746174656d656e745f616c6c6f77616e63653a` (the bytes of `:statement_allowance:`) followed by the account's 32-byte public key in hex. For `//Alice`, the public key is `d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`.
    - **Value**: The SCALE-encoded `StatementAllowance`, two little-endian `u32` values. For example, `0x6400000040420f00` encodes `max_count` 100 and `max_size` 1,000,000.

    The Polkadot SDK's own statement store tests set allowances the same way, through a sudo `System::set_storage` call, or by inserting the same key and value into `genesis.raw.top` of a raw chain spec before the chain starts.

4. Submit statements signed by that account to `statement_submit` at `{{ networks.local.ws_url }}`, and subscribe with `statement_subscribeStatement`. A Product does not call these methods itself: the Host submits and subscribes on its behalf, as described in [Publish and Subscribe to Off-Chain Data](/apps/build/pub-sub-off-chain-data.md).

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Lifecycle**

    ---

    How a statement moves through validation, gossip, expiry, and eviction once the store is running.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/statement-store/lifecycle.md)

- <span class="badge learn">Learn</span> **Allowance**

    ---

    How `max_count` and `max_size` limit what each account can publish.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/statement-store/allowance.md)

- <span class="badge learn">Learn</span> **Subscriptions**

    ---

    How a subscriber registers a topic filter with a node.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/statement-store/subscriptions.md)

- <span class="badge learn">Learn</span> **Channels**

    ---

    The last-write-wins primitive built on top of statements.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/statement-store/channels.md)

</div>
