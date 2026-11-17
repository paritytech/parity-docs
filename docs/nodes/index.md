---
title: Nodes
description: Learn which Polkadot SDK node binaries exist, the modes they run in, and where to find guides for installing, running, and monitoring them.
categories: Nodes
---

# Nodes Overview

## Introduction

The Polkadot SDK ships node binaries that run relay chains and parachains. The same binary can serve RPC requests, keep a full archive, author parachain blocks, or validate, depending on the flags you start it with. This section covers installing these binaries, choosing a run mode, and operating the node.

## Node Binaries

Each Polkadot SDK release includes the following node binaries:

- **`polkadot`**: The relay chain node. It runs any relay chain it has a chain specification for and ships with two worker binaries, `polkadot-prepare-worker` and `polkadot-execute-worker`, used in validator mode. See [Install and Run a Polkadot Node](/nodes/run-a-node/relay-chain/full-node/).
- **`polkadot-parachain`**: A parachain node for the system parachain runtimes and parachains built from the standard Cumulus templates. It embeds a relay chain node so that it can follow the relay chain. See [Run a Parachain RPC Node](/nodes/run-a-node/parachain-rpc/).
- **`polkadot-omni-node`**: A parachain node that loads the runtime from a chain specification, so it can run most parachains without a dedicated node binary. See [Polkadot Omni Node](/tools/omninode/).

## Run Modes

Startup flags determine what a node does. The main modes are as follows:

- **RPC node**: Exposes JSON-RPC over WebSocket and HTTP so applications, wallets, and explorers can query state and submit transactions. A node that receives a transaction through a method such as `author_submitExtrinsic` validates it, adds it to its transaction pool, and gossips it to peers, where block producers pick it up.
- **Pruned or archive node**: Pruned nodes keep recent state and a limited number of finalized blocks, which suits most applications. Archive nodes keep the state and bodies of every block since genesis, which explorers, indexers, and historical queries need. The `--state-pruning` and `--blocks-pruning` flags set this.
- **Bootnode**: A node with a stable network identity that new nodes contact to discover peers. See [Set Up a Bootnode](/nodes/run-a-node/relay-chain/bootnode/).
- **Collator**: A parachain node started with `--collator`. It collects transactions, builds parachain block candidates with their Proof-of-Validity (PoV), and sends them to relay chain validators. See [Run a Block-Producing Collator](/nodes/run-a-collator/).
- **Validator mode**: A relay chain node started with `--validator`. It authors relay chain blocks and validates parachain candidates inside a sandbox. See [Validator Mode](/nodes/run-a-node/relay-chain/full-node/#validator-mode).

Joining a live network as a validator or collator, including staking, rewards, slashing, and collator selection, is a network process. See [Node Infrastructure](https://developers.polkadot.com/node-infrastructure/) on the Polkadot Developer Docs.

## Next Steps

<div class="grid cards" markdown>

-   **Install and Run a Node**

    ---

    Install the `polkadot` binary, verify the release, and run a pruned, archive, or RPC node.

    [:octicons-arrow-right-24: Get Started](/nodes/run-a-node/relay-chain/full-node/)

-   **Run a Parachain RPC Node**

    ---

    Serve RPC for a parachain with `polkadot-parachain`, and add the Ethereum RPC adapter.

    [:octicons-arrow-right-24: Get Started](/nodes/run-a-node/parachain-rpc/)

-   **Run a Collator**

    ---

    Run a parachain node in collator mode and generate its keys.

    [:octicons-arrow-right-24: Get Started](/nodes/run-a-collator/)

-   **Monitor and Harden a Node**

    ---

    Set up Prometheus, Grafana, and Alertmanager, tune the kernel, and use secure-validator mode.

    [:octicons-arrow-right-24: Get Started](/nodes/monitor-and-harden/)

</div>
