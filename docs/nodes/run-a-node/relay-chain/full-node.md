---
title: Install and Run a Polkadot Node
description: Install the polkadot node binary, verify the release signature, and run it as a pruned, archive, or RPC node for any relay chain built with the Polkadot SDK.
categories: Nodes
---

# Install and Run a Polkadot Node

## Introduction

The `polkadot` binary is the relay chain node built from the [Polkadot SDK](https://github.com/paritytech/polkadot-sdk). A single binary can run any relay chain it has a chain specification for, so the same installation serves Polkadot, Kusama, Westend, Paseo, or a relay chain you define yourself.

Running your own node gives you direct access to chain data, full control over the RPC requests and transactions you submit, and a source of blocks for indexers and applications. Startup flags select the run mode:

- **Pruned full node**: Keeps recent state and finalized blocks. This is the default.
- **Archive node**: Keeps the state and bodies of every block since genesis, for explorers and historical queries.
- **RPC node**: A pruned or archive node that exposes its JSON-RPC server to external clients.
- **Validator mode**: The `--validator` flag, which turns on block authoring and parachain validation. See [Validator Mode](#validator-mode).

This page covers installing the `polkadot` binary, verifying the release, and configuring each run mode. To run a parachain node, see [Run a Parachain RPC Node](/nodes/run-a-node/parachain-rpc.md) or [Polkadot Omni Node](/tools/omninode.md).

## Prerequisites

Before you install the node, check the following:

- Pre-built release binaries and Debian packages target Linux. On macOS or Windows, use [Docker](#install-with-docker) or [build from source](#build-from-source). On Windows, use [WSL and Ubuntu](https://learn.microsoft.com/en-us/windows/wsl/install) and follow the Linux instructions.
- To build from source, [install the Polkadot SDK dependencies](/polkadot-sdk/install-polkadot-sdk.md) first.

## Install the Binaries

A Polkadot SDK release ships three relay chain binaries:

- `polkadot`: The node itself. A full, archive, or RPC node needs only this binary.
- `polkadot-prepare-worker` and `polkadot-execute-worker`: Worker processes that prepare and execute parachain validation code. The node uses them only in [validator mode](#validator-mode), where all three binaries must be in the same directory.

Choose one of the following installation methods.

### Download a Release Binary

Download the binaries from the [Polkadot SDK releases](https://github.com/paritytech/polkadot-sdk/releases) page, or use the following commands, which also download and verify each binary's signature:

=== "`polkadot`"

    ```bash
    # Download the binary
    curl -LO https://github.com/paritytech/polkadot-sdk/releases/download/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot

    # Verify signature
    curl -LO https://github.com/paritytech/polkadot-sdk/releases/download/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot.asc

    gpg --keyserver hkps://keyserver.ubuntu.com --receive-keys 90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE

    gpg --verify polkadot.asc
    ```

=== "`polkadot-prepare-worker`"

    ```bash
    # Download the binary
    curl -LO https://github.com/paritytech/polkadot-sdk/releases/download/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot-prepare-worker

    # Verify signature
    curl -LO https://github.com/paritytech/polkadot-sdk/releases/download/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot-prepare-worker.asc

    gpg --keyserver hkps://keyserver.ubuntu.com --receive-keys 90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE

    gpg --verify polkadot-prepare-worker.asc
    ```

=== "`polkadot-execute-worker`"

    ```bash
    # Download the binary
    curl -LO https://github.com/paritytech/polkadot-sdk/releases/download/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot-execute-worker

    # Verify signature
    curl -LO https://github.com/paritytech/polkadot-sdk/releases/download/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot-execute-worker.asc

    gpg --keyserver hkps://keyserver.ubuntu.com --receive-keys 90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE

    gpg --verify polkadot-execute-worker.asc
    ```

Signature verification uses GPG signing keys to confirm that the downloaded binaries are authentic and have not been tampered with. Polkadot SDK releases use two signing keys:

- ParityReleases (release-team@parity.io) with key [`90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE`](https://keyserver.ubuntu.com/pks/lookup?search=90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE&fingerprint=on&op=index) for current and new releases.
- Parity Security Team (security@parity.io) with key [`9D4B2B6EB8F97156D19669A9FF0812D491B96798`](https://keyserver.ubuntu.com/pks/lookup?search=9D4B2B6EB8F97156D19669A9FF0812D491B96798&fingerprint=on&op=index) for old releases.

!!! warning
    A "Good signature" message indicates successful verification. Any other output signals a potential security risk, so do not run the binary.

After verification, make the binaries executable:

```bash
chmod +x polkadot polkadot-prepare-worker polkadot-execute-worker
```

!!! tip
    Pre-built binaries may not work on your architecture or Linux distribution. An error like `cannot execute binary file: Exec format error` means the binary is incompatible with your system. Use [Docker](#install-with-docker) or [build from source](#build-from-source) instead.

### Install with APT

On Debian-based distributions such as Ubuntu, you can install the binaries with the [APT](https://wiki.debian.org/Apt) package manager.

Run the following commands as root to add the Parity package repository and install the binaries:

```bash
# Import the release-team@parity.io GPG key
gpg --keyserver hkps://keyserver.ubuntu.com --receive-keys 90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE
gpg --export 90BD75EBBB8E95CB3DA6078F94A4029AB4B35DAE > /usr/share/keyrings/parity.gpg

# Add the Parity repository and update the package index
echo 'deb [signed-by=/usr/share/keyrings/parity.gpg] https://releases.parity.io/deb release main' > /etc/apt/sources.list.d/parity.list
apt update

# Install the `parity-keyring` package - This will ensure the GPG key
# used by APT remains up-to-date
apt install parity-keyring

# Install polkadot
apt install polkadot
```

### Install with Docker

To use Docker or another OCI-compatible container runtime, pull the `parity/polkadot` image from [Docker Hub](https://hub.docker.com/r/parity/polkadot/tags):

```bash
docker pull parity/polkadot:{{ dependencies.repositories.polkadot_sdk.docker_image_version }}
```

Inside the container, the node listens only on `localhost` by default. To reach its RPC and Prometheus endpoints from the host, publish the ports and add the `--rpc-external` and `--prometheus-external` flags:

```bash
docker run -p 9944:9944 -p 9615:9615 \
  parity/polkadot:{{ dependencies.repositories.polkadot_sdk.docker_image_version }} \
  --chain INSERT_CHAIN \
  --name "INSERT_NODE_NAME" \
  --rpc-external \
  --prometheus-external
```

On Apple silicon, add `--platform linux/amd64` after `docker run`.

### Install with Ansible

To manage installations on several machines, the [Parity chain operations Ansible collection](https://github.com/paritytech/ansible-galaxy/) provides a Substrate node role.

### Build from Source

Building from source works on any platform the Polkadot SDK supports. After you [install the dependencies](/polkadot-sdk/install-polkadot-sdk.md), clone the release tag and build it:

```bash
git clone --branch {{ dependencies.repositories.polkadot_sdk.version }} --depth 1 https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk
cargo build --release --locked
```

Depending on your machine, compiling may take an hour or more. The `polkadot`, `polkadot-prepare-worker`, and `polkadot-execute-worker` binaries are written to `./target/release/`. If you move `polkadot` elsewhere, for example into a directory on your `$PATH`, move the two worker binaries with it.

A community-maintained [snap package](https://snapcraft.io/polkadot) also exists. Check its publisher and version before you use it.

## Verify the Installation

Check that the binaries run and report the same version:

```bash
polkadot --version
polkadot-execute-worker --version
polkadot-prepare-worker --version
```

The output should show matching version numbers, similar to the following (the specific version may vary):

--8<-- 'code/node-infrastructure/run-a-validator/onboarding-and-offboarding/set-up-validator/terminal-output-01.html'

If the versions differ or a command fails, confirm that all the binaries were installed from the same release and that their directory is on your `$PATH`:

```bash
echo $PATH
```

If necessary, move the binaries to a shared location such as `/usr/local/bin/`.

## Configure and Run Your Node

The `--chain` flag selects the network. It accepts a built-in chain identifier, such as `polkadot`, `kusama`, `westend`, or `paseo`, or the path to a chain specification JSON file for any other relay chain. Replace `INSERT_CHAIN` in the following commands with your value. If you built from source, replace `polkadot` with `./target/release/polkadot`.

The base commands for each storage mode are as follows:

=== "Default pruned node"

    This uses the default state pruning value of the last 256 blocks:

    ```bash
    polkadot --chain INSERT_CHAIN \
      --name "INSERT_NODE_NAME"
    ```

=== "Custom pruned node"

    You can customize the pruning value, for example, to keep the last 1000 finalized blocks:

    ```bash
    polkadot --chain INSERT_CHAIN \
      --name "INSERT_NODE_NAME" \
      --state-pruning 1000 \
      --blocks-pruning 1000
    ```

=== "Archive node"

    To keep the full state and every block, use the `archive` option:

    ```bash
    polkadot --chain INSERT_CHAIN \
      --name "INSERT_NODE_NAME" \
      --state-pruning archive \
      --blocks-pruning archive
    ```

The [Node Configuration Parameters](/nodes/run-a-node/parachain-rpc.md#node-configuration-parameters) section of the parachain RPC guide explains the pruning and sync flags in detail. To list every available flag, run:

```bash
polkadot --help
```

To run the node on a local relay chain for development instead of a live network, see [Zombienet](/tools/zombienet.md).

### RPC Configuration

The node's startup flags control which RPC methods are exposed, how many connections are accepted, and which origins can connect:

- Limit the exposed methods with `--rpc-methods`. Use `--rpc-methods safe` for public endpoints.
- Set the maximum number of connections with `--rpc-max-connections`, for example, `--rpc-max-connections 200`.
- By default, only `localhost` origins and `https://polkadot.js.org` can access the RPC server. Change this with `--rpc-cors`. To allow access from any origin, use `--rpc-cors all`.
- Listen on all network interfaces with `--rpc-external`. Combine it with `--rpc-methods safe` and, in production, a TLS proxy. See [Set Up Secure WebSocket](/nodes/run-a-node/relay-chain/secure-wss.md).

For more flags relevant to RPC nodes, see [Important Flags for Running an RPC Node](https://paritytech.github.io/devops-guide/guides/rpc_index.html?#important-flags-for-running-an-rpc-node) in the Parity DevOps guide.

### Telemetry

Telemetry reports your node's name, version, and sync status to a telemetry server. By default, the node connects to the telemetry endpoints listed in the chain specification, if any. Override this with the following flags:

- `--telemetry-url 'INSERT_URL INSERT_VERBOSITY'`: Sends telemetry to your own server instead. Verbosity ranges from `0` (least verbose) to `9`. Pass the flag more than once to report to several endpoints.
- `--no-telemetry`: Disables telemetry.

For example, to report to a self-hosted telemetry server:

```bash
polkadot --chain INSERT_CHAIN \
  --name "INSERT_NODE_NAME" \
  --telemetry-url 'wss://INSERT_TELEMETRY_HOST/submit 0'
```

The node name set with `--name` is the label that appears on the telemetry server.

## Sync Your Node

Syncing time depends on the chain's size, your bandwidth, processing power, disk speed, and RAM. A healthy node syncing blocks outputs logs like the following:

--8<-- 'code/node-infrastructure/run-a-node/full-node/run-node.html'

To check progress from another terminal, call the `system_syncState` RPC method. Syncing is complete when `currentBlock` equals `highestBlock`:

```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState", "params":[]}' \
  http://localhost:9944
```

The process is the same for any Polkadot SDK-based chain, although individual chains may require chain-specific flags.

### Connect to Your Node

By default, the node serves JSON-RPC over WebSocket and HTTP at the following local endpoint:

```text
{{ networks.local.ws_url }}
```

Connect to it from any Polkadot SDK client, such as [Subxt](/tools/subxt.md) or a block explorer like Polkadot.js Apps. See [Chain Interactions](/tools/chain-interactions/index.md) for client options.

## Validator Mode

The `--validator` flag runs the node as an authority: it authors blocks, takes part in finality, and validates parachain candidates. In this mode, the node also needs the `polkadot-prepare-worker` and `polkadot-execute-worker` binaries and enforces [secure-validator mode](/nodes/monitor-and-harden.md#secure-validator-mode), a set of Linux sandboxing checks.

One of the sandboxing features the node uses is [Landlock](https://docs.kernel.org/userspace-api/landlock.html), which lets an unprivileged process restrict its own access to the system. Landlock is available in Linux kernels 5.13 and later and is activated by default on most distributions. Check whether it is active by running the following command as root:

```bash
dmesg | grep landlock || journalctl -kg landlock
```

If the command prints nothing, Landlock is not active. Activate it or use a kernel and distribution that support it. See the [kernel support documentation](https://docs.kernel.org/userspace-api/landlock.html#kernel-support) for details.

Becoming a validator on a live network involves session keys, staking, and network-specific requirements. See [Set Up a Validator](https://developers.polkadot.com/node-infrastructure/run-a-validator/onboarding-and-offboarding/set-up-validator/) on the Polkadot Developer Docs.
