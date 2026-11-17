---
title: Run a Parachain RPC Node
description: Learn the hardware and software requirements for a parachain RPC node and how to run one with polkadot-parachain using Docker or systemd, plus the Ethereum RPC adapter.
categories: Nodes
---

# Run a Parachain RPC Node

## Introduction

A parachain RPC node gives applications direct access to a parachain's state, transactions, and events. Running your own node also supports infrastructure tasks such as block indexing and serves Polkadot SDK tools that need an endpoint.

Through the RPC port (9944, serving both WebSocket and HTTP), your node acts as the bridge between the parachain and applications. This page walks through setting up a parachain RPC node with the `polkadot-parachain` binary, covering hardware requirements and deployment with Docker or systemd. It also covers the `eth-rpc` adapter, which adds an Ethereum JSON-RPC interface on chains that include `pallet-revive`.

A parachain node embeds a relay chain node so that it can follow the relay chain. The commands on this page configure both. To run a parachain whose runtime works without a dedicated node binary, you can use [Polkadot Omni Node](/tools/omninode.md) with the same flags.

## Prerequisites

### Hardware Requirements

RPC nodes serving production traffic require the following hardware:

- **CPU**: 8+ cores; 16+ cores for high traffic.
- **Memory**: 64 GB RAM minimum; 128 GB recommended for high traffic.
- **Storage**: The pruned relay chain state plus the parachain state. Requirements depend on the chain and on whether you run a pruned or archive node, so check the chain's documentation. An archive node needs far more space than a pruned node, which uses warp sync. Fast NVMe I/O is critical for RPC query performance.
- **Network**: A public IP address and a stable connection with sufficient bandwidth (1 Gbps for high traffic). Consider DDoS protection and rate limiting for production deployments. Open the following ports:
    - **30333**: Parachain P2P over TCP, plus UDP for Web Real-Time Communication (WebRTC).
    - **30334**: Relay chain P2P over TCP, plus UDP for WebRTC.
    - **9944**: Polkadot SDK RPC (WebSocket and HTTP).
    - **8545**: Ethereum JSON-RPC (if running the `eth-rpc` adapter).

!!! note
    For development or low-traffic scenarios, you can reduce these requirements proportionally. Consider using a reverse proxy ([nginx](https://nginx.org/), [Caddy](https://caddyserver.com/)) for production deployments. See [Set Up Secure WebSocket](/nodes/run-a-node/relay-chain/secure-wss.md).

### Software Requirements

Required software:

- **Operating system**: Ubuntu 22.04 LTS (recommended) or a similar Linux distribution.
- **[Docker](https://www.docker.com/get-started/)**: Required for the Docker deployment option.

## Obtain the Chain Specification

To run a node for a parachain, you need its chain specification file. This JSON file defines the network parameters, genesis state, and bootnodes. Obtain it from the parachain's maintainers or documentation.

Chain specifications for the system parachains are stored in the Polkadot SDK repository under [`cumulus/parachains/chain-specs`](https://github.com/paritytech/polkadot-sdk/tree/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/chain-specs). For example, to download one:

```bash
curl -sL -o chain-spec.json \
  https://raw.githubusercontent.com/paritytech/polkadot-sdk/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/chain-specs/asset-hub-polkadot.json
```

For a parachain you build yourself, generate the chain specification with [`chain-spec-builder`](/tools/chain-spec-builder.md). See [Set Up the Polkadot SDK Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md).

## Spin Up a Node

Choose the deployment option that fits your project, and follow the steps in the appropriate tab:

- **Docker**: Best for simpler setup and maintenance.
- **systemd**: Best for production environments requiring more control.

In the following commands, replace `INSERT_NODE_NAME` with a name for your node and `INSERT_RELAY_CHAIN` with the relay chain the parachain connects to, as a built-in chain identifier (such as `polkadot`, `kusama`, `westend`, or `paseo`) or the path to a relay chain specification file. With Docker, mount any relay chain specification file into the container, as the commands do for the parachain specification.

An archive node imports every block from genesis because warp sync cannot be combined with archive state pruning, so its initial sync takes much longer than a pruned node's.

=== "Docker"

    1. Download your parachain's chain specification as described in [Obtain the Chain Specification](#obtain-the-chain-specification).

    2. Create a directory for the node's data:

        ```bash
        mkdir -p my-node-data
        ```

    3. Launch the parachain node using the [`parity/polkadot-parachain` Docker image](https://hub.docker.com/r/parity/polkadot-parachain):

        === "Archive"

            ```bash
            docker run -d --name parachain-rpc --restart unless-stopped \
              -p 9944:9944 \
              -p 9615:9615 \
              -p 30334:30334 \
              -p 30334:30334/udp \
              -p 30333:30333 \
              -p 30333:30333/udp \
              -v $(pwd)/chain-spec.json:/chain-spec.json \
              -v $(pwd)/my-node-data:/data \
              parity/polkadot-parachain:{{dependencies.repositories.polkadot_sdk.docker_image_version}} \
              --name=INSERT_NODE_NAME \
              --base-path=/data \
              --chain=/chain-spec.json \
              --prometheus-external \
              --prometheus-port 9615 \
              --rpc-external \
              --rpc-port=9944 \
              --rpc-cors=all \
              --rpc-methods=safe \
              --rpc-max-connections=1000 \
              --state-pruning=archive \
              --blocks-pruning=archive \
              -- \
              --base-path=/data \
              --chain=INSERT_RELAY_CHAIN \
              --sync=warp \
              --state-pruning=256 \
              --blocks-pruning=256 \
              --rpc-port=0
            ```

        === "Pruned"

            ```bash
            docker run -d --name parachain-rpc --restart unless-stopped \
              -p 9944:9944 \
              -p 9615:9615 \
              -p 30334:30334 \
              -p 30334:30334/udp \
              -p 30333:30333 \
              -p 30333:30333/udp \
              -v $(pwd)/chain-spec.json:/chain-spec.json \
              -v $(pwd)/my-node-data:/data \
              parity/polkadot-parachain:{{dependencies.repositories.polkadot_sdk.docker_image_version}} \
              --name=INSERT_NODE_NAME \
              --base-path=/data \
              --chain=/chain-spec.json \
              --prometheus-external \
              --prometheus-port 9615 \
              --rpc-external \
              --rpc-port=9944 \
              --rpc-cors=all \
              --rpc-methods=safe \
              --rpc-max-connections=1000 \
              --sync=warp \
              --state-pruning=1000 \
              --blocks-pruning=1000 \
              -- \
              --base-path=/data \
              --chain=INSERT_RELAY_CHAIN \
              --sync=warp \
              --state-pruning=256 \
              --blocks-pruning=256 \
              --rpc-port=0
            ```

        !!! note
            The `parity/polkadot-parachain` image works for system parachains and parachains built with standard Cumulus templates. For parachains with custom runtimes, check the parachain's documentation for their specific Docker image or binary.

        Refer to the [Port Mappings](#port-mappings) and [Node Configuration Parameters](#node-configuration-parameters) sections for details on the command's configurations.

=== "systemd"

    1. Download the `polkadot-parachain` binary from the latest stable [Polkadot SDK release](https://github.com/paritytech/polkadot-sdk/releases):

        ```bash
        # Download the latest stable release (check releases page for current version)
        wget https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk.version}}/polkadot-parachain

        # Make it executable and move to system path
        chmod +x polkadot-parachain
        sudo mv polkadot-parachain /usr/local/bin/

        # Verify installation
        polkadot-parachain --version
        ```

    2. Download your parachain's chain specification as described in [Obtain the Chain Specification](#obtain-the-chain-specification).

    3. Create user and directory structures:

        ```bash
        # Create a dedicated user
        sudo useradd -r -s /bin/bash polkadot
        
        # Create data directory
        sudo mkdir -p /var/lib/parachain-rpc

        # Copy the chain spec to the directory
        sudo cp chain-spec.json /var/lib/parachain-rpc/

        # Set permissions
        sudo chown -R polkadot:polkadot /var/lib/parachain-rpc
        ```

    4. Create a systemd service file for the Polkadot SDK RPC node:

        ```bash
        sudo nano /etc/systemd/system/parachain-rpc.service
        ```

    5. Open the new service file and add the configuration for either an archive (complete history) or pruned (recent state) node:

        === "Archive"

            ```ini
            [Unit]
            Description=Parachain RPC Node
            After=network.target

            [Service]
            Type=simple
            User=polkadot
            Group=polkadot
            WorkingDirectory=/var/lib/parachain-rpc

            ExecStart=/usr/local/bin/polkadot-parachain \
              --name=INSERT_NODE_NAME \
              --chain=/var/lib/parachain-rpc/chain-spec.json \
              --base-path=/var/lib/parachain-rpc \
              --port=30333 \
              --rpc-port=9944 \
              --rpc-external \
              --rpc-cors=all \
              --rpc-methods=safe \
              --rpc-max-connections=1000 \
              --prometheus-port=9615 \
              --prometheus-external \
              --state-pruning=archive \
              --blocks-pruning=archive \
              -- \
              --chain=INSERT_RELAY_CHAIN \
              --base-path=/var/lib/parachain-rpc \
              --port=30334 \
              --sync=warp \
              --state-pruning=256 \
              --blocks-pruning=256 \
              --rpc-port=0

            Restart=always
            RestartSec=10
            LimitNOFILE=65536

            [Install]
            WantedBy=multi-user.target
            ```

        === "Pruned"

            ```ini
            [Unit]
            Description=Parachain RPC Node
            After=network.target

            [Service]
            Type=simple
            User=polkadot
            Group=polkadot
            WorkingDirectory=/var/lib/parachain-rpc

            ExecStart=/usr/local/bin/polkadot-parachain \
              --name=INSERT_NODE_NAME \
              --chain=/var/lib/parachain-rpc/chain-spec.json \
              --base-path=/var/lib/parachain-rpc \
              --port=30333 \
              --rpc-port=9944 \
              --rpc-external \
              --rpc-cors=all \
              --rpc-methods=safe \
              --rpc-max-connections=1000 \
              --prometheus-port=9615 \
              --prometheus-external \
              --sync=warp \
              --state-pruning=1000 \
              --blocks-pruning=1000 \
              -- \
              --chain=INSERT_RELAY_CHAIN \
              --base-path=/var/lib/parachain-rpc \
              --port=30334 \
              --sync=warp \
              --state-pruning=256 \
              --blocks-pruning=256 \
              --rpc-port=0

            Restart=always
            RestartSec=10
            LimitNOFILE=65536

            [Install]
            WantedBy=multi-user.target
            ```

        Refer to the [Port Mappings](#port-mappings) and [Node Configuration Parameters](#node-configuration-parameters) sections for details on the command's configurations.

    6. Start the service:

        ```bash
        # Reload systemd
        sudo systemctl daemon-reload

        # Enable service to start on boot
        sudo systemctl enable parachain-rpc
        
        # Start the Polkadot SDK node:
        sudo systemctl start parachain-rpc
        ```

### Port Mappings

- **`9944`**: Polkadot SDK RPC endpoint (WebSocket/HTTP)
- **`9615`**: Prometheus metrics endpoint
- **`30333/30334`**: P2P networking ports. TCP carries regular peer connections. UDP on the same port numbers carries WebRTC connections, which full nodes accept by default on the litep2p network backend.

### Node Configuration Parameters

- **`--rpc-external`**: Listens for RPC connections on all network interfaces instead of only on `localhost`. Combine it with `--rpc-methods=safe` and, for production, a reverse proxy with rate limiting.
- **`--unsafe-rpc-external`**: Behaves like `--rpc-external` but skips the check that rejects external RPC access on validator nodes. An RPC node does not need it.
- **`--rpc-cors=all`**: Allows all origins for CORS.
- **`--rpc-methods=safe`**: Only allows safe RPC methods.
- **`--sync=warp`**: Downloads the latest finalized state with a finality proof instead of executing every block since genesis. When `--blocks-pruning` is set to a number, the node then backfills only the headers and justifications of older blocks. Warp sync only applies when the database is empty and cannot be combined with `--state-pruning=archive` or `--state-pruning=archive-canonical`.
- **`--state-pruning`**: `archive` keeps the state of all blocks, `archive-canonical` keeps the state of finalized blocks only, and `[NUMBER]` keeps the state of the last specified number of finalized blocks. Defaults to `256`.
- **`--blocks-pruning`**: `archive` keeps all block bodies, `archive-canonical` keeps the bodies of finalized blocks only, and `[NUMBER]` keeps the bodies of the last specified number of finalized blocks. Defaults to `archive-canonical`, so set a number explicitly on pruned nodes. Block headers are always kept.
- **`--prometheus-external`**: Exposes metrics externally.
- **`--`**: Separates parachain flags from relay chain flags. Flags after `--` configure the embedded relay chain node.

!!! warning
    The state pruning mode is stored in the database when the node first starts. Later runs fail if `--state-pruning` does not match the stored mode, except that `[NUMBER]` can change. To switch between a pruned node and an archive node, delete the database and sync again.

## Monitor Node Synchronization

Monitor the node synchronization status:

```bash
curl -H "Content-Type: application/json" \
-d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState", "params":[]}' \
http://localhost:9944
```

When synchronization is complete, `currentBlock` will be equal to `highestBlock`:

<div class="termynal" data-termynal>
  <span data-ty="input"><span class="file-path"></span>curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState", "params":[]}' \
  http://localhost:9944</span>
  <span data-ty><pre>{
  "jsonrpc":"2.0",
  "id":1,
  "result":{
    "startingBlock":0,
    "currentBlock":3394816,
    "highestBlock":3394816
  }
}
  </pre></span>
</div>

!!! tip
    You can use the `system_health` command to verify your node is running properly.

    ```bash
    curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
    http://localhost:9944
    ```

## Commands for Managing Your Node

Use the following commands to manage your node:

=== "Docker"

    - **View node logs**:

        ```bash
        docker logs -f parachain-rpc
        ```

    - **Stop container**:

        ```bash
        docker stop parachain-rpc
        ```

    - **Start container**:

        ```bash
        docker start parachain-rpc
        ```

    - **Remove container**:

        ```bash
        docker rm parachain-rpc
        ```

=== "systemd"

    - **Check status**:

        ```bash
        sudo systemctl status parachain-rpc
        ```

    - **View node logs**:

        ```bash
        sudo journalctl -u parachain-rpc -f
        ```

    - **Stop service**:

        ```bash
        sudo systemctl stop parachain-rpc
        ```

    - **Enable service**:

        ```bash
        sudo systemctl enable parachain-rpc
        ```

    - **Start service**:

        ```bash
        sudo systemctl start parachain-rpc
        ```

## Ethereum RPC Compatibility

!!! note
    Ethereum RPC compatibility is available only on chains whose runtime includes `pallet-revive`, such as Polkadot Hub.

Ethereum RPC compatibility is provided through the [`eth-rpc` adapter](/smart-contracts/eth-rpc-adapter.md), which is part of [pallet-revive](https://paritytech.github.io/polkadot-sdk/master/pallet_revive_eth_rpc/index.html). This adapter translates Ethereum JSON-RPC calls into Polkadot SDK-compatible requests, enabling integration with Ethereum tools like [MetaMask](https://metamask.io/), [Hardhat](https://hardhat.org/), and [Ethers.js](https://docs.ethers.org/v6/).

### Prerequisites

Before starting the Ethereum RPC adapter:

- **Node synchronization**: Let your node finish syncing before you start the adapter. On startup, the `eth-rpc` adapter reads `pallet-revive` configuration from the node's latest block and fails to start if that block predates `pallet-revive`, for example, early in the initial sync.
- **Archive node recommended**: For full Ethereum RPC compatibility, run an archive node (`--state-pruning=archive`). The `eth-rpc` adapter may fail to query historical state on pruned nodes.
- **RPC accessibility**: The Polkadot SDK-based RPC endpoint must be accessible (default: `ws://127.0.0.1:9944`).

### Run the Ethereum RPC Adapter

You can run the Ethereum RPC adapter using Docker or as a systemd service.

=== "Docker"

    Start the adapter using the [`parity/eth-rpc` Docker image](https://hub.docker.com/r/parity/eth-rpc/tags):

    ```bash
    docker run -d --name eth-rpc --restart unless-stopped \
      --network=host \
      -v /var/lib/eth-rpc:/data \
      parity/eth-rpc:{{dependencies.repositories.polkadot_sdk.docker_image_version}} \
      --node-rpc-url=ws://127.0.0.1:9944 \
      --rpc-port=8545 \
      --base-path=/data \
      --rpc-external \
      --rpc-cors=all
    ```

    !!! note
        The `-v` flag maps the host directory `/var/lib/eth-rpc` to `/data` inside the container. The `--base-path` flag references this container path to persist the `eth-rpc.db` database across restarts.

    !!! note
        Check the [Docker Hub tags page](https://hub.docker.com/r/parity/eth-rpc/tags) for newer releases. Tags follow the Polkadot SDK release naming, such as `{{dependencies.repositories.polkadot_sdk.docker_image_version}}`.

=== "systemd"

    1. Build the `eth-rpc` binary from the Polkadot SDK source:

        ```bash
        git clone --branch {{dependencies.repositories.polkadot_sdk.version}} --depth 1 https://github.com/paritytech/polkadot-sdk.git
        cd polkadot-sdk
        cargo build -p pallet-revive-eth-rpc --bin eth-rpc --release

        # Move to system path
        sudo mv target/release/eth-rpc /usr/local/bin/
        ```

        !!! note
            Pre-built binaries may not be available for all platforms. Building from source ensures compatibility with your system.

    2. Create a systemd service file:

        ```bash
        sudo nano /etc/systemd/system/eth-rpc.service
        ```

    3. Add the following configuration:

        ```ini
        [Unit]
        Description=Ethereum RPC Adapter
        After=network.target parachain-rpc.service

        [Service]
        Type=simple
        User=polkadot
        Group=polkadot

        ExecStart=/usr/local/bin/eth-rpc \
          --node-rpc-url=ws://127.0.0.1:9944 \
          --rpc-port=8545 \
          --base-path=/var/lib/eth-rpc \
          --rpc-external \
          --rpc-cors=all

        Restart=always
        RestartSec=10

        [Install]
        WantedBy=multi-user.target
        ```

        !!! warning
            The `--rpc-external` flag exposes your RPC endpoint publicly. For production deployments, consider using a reverse proxy with authentication and rate limiting, or bind to a specific interface.

    4. Start the service:

        ```bash
        sudo systemctl daemon-reload
        sudo systemctl enable eth-rpc
        sudo systemctl start eth-rpc
        ```

### Ethereum RPC Configuration

The adapter accepts the following key parameters:

| Parameter | Description | Default |
|:---------:|:-----------:|:-------:|
| `--node-rpc-url` | Polkadot SDK-based node WebSocket URL | `ws://127.0.0.1:9944` |
| `--rpc-port` | Ethereum RPC server port | `8545` |
| `--rpc-external` | Listen for RPC connections on all network interfaces instead of only on `localhost` | Disabled |
| `--rpc-cors` | CORS allowed origins (`all` or a comma-separated list) | `localhost`, `127.0.0.1` (HTTP and HTTPS, any port), and `https://polkadot.js.org`; all origins with `--dev` |
| `--eth-pruning` | Block storage strategy: `archive` for persistent on-disk DB with full historical sync, or `<N>` for in-memory DB keeping latest N blocks | `archive` |
| `--base-path` | Directory for persistent database storage (`eth-rpc.db` is created inside this directory) | OS default data directory |
| `--dev` | Use temporary on-disk directory, deleted on exit | Disabled |

!!! warning
    The `--rpc-external` flag exposes your RPC endpoint publicly. For production deployments, use a reverse proxy with proper authentication and rate limiting.

??? note "Migrating from previous CLI flags"
    As of [PR #11153](https://github.com/paritytech/polkadot-sdk/pull/11153), the following flags have been removed and replaced:

    | Previous Flag | Replacement |
    |:-------------:|:-----------:|
    | `--cache-size N` | `--eth-pruning=<N>` |
    | `--database-url sqlite::memory:` | `--eth-pruning=<N>` |
    | `--database-url /path/to/db` | `--base-path=/path/to/dir` |
    | `--index-last-n-blocks N` | `--eth-pruning=archive` |
    | `--earliest-receipt-block N` | Removed; the adapter auto-discovers the first EVM block |

### API Endpoints

Your node setup provides two distinct API interfaces:

| Interface | Port | Protocol | Use Cases |
|:---------:|:----:|:--------:|:---------:|
| **Polkadot SDK RPC** | 9944 | WebSocket/HTTP | Polkadot SDK-native applications and parachain-specific operations |
| **Ethereum RPC** | 8545 | HTTP | EVM libraries, Ethereum tools, EVM-compatible dApps |

### Verify the Ethereum RPC Adapter

To verify the Ethereum RPC adapter is working correctly, you can test standard Ethereum JSON-RPC methods like `eth_chainId`, `eth_blockNumber`, and `eth_getBlockByNumber`. For a complete list of supported methods and example queries, see the [JSON-RPC APIs](/smart-contracts/for-eth-devs/json-rpc-apis.md) reference.

### Manage the Ethereum RPC Adapter

=== "Docker"

    - **View logs**:

        ```bash
        docker logs -f eth-rpc
        ```

    - **Stop container**:

        ```bash
        docker stop eth-rpc
        ```

    - **Start container**:

        ```bash
        docker start eth-rpc
        ```

=== "systemd"

    - **Check status**:

        ```bash
        sudo systemctl status eth-rpc
        ```

    - **View logs**:

        ```bash
        sudo journalctl -u eth-rpc -f
        ```

    - **Stop service**:

        ```bash
        sudo systemctl stop eth-rpc
        ```

## Where to Go Next

- To expose the RPC endpoint over TLS, see [Set Up Secure WebSocket](/nodes/run-a-node/relay-chain/secure-wss.md).
- To collect metrics and set up alerts, see [Monitor and Harden a Node](/nodes/monitor-and-harden.md).
- To interact with the node from code, see [Chain Interactions](/tools/chain-interactions/index.md).
