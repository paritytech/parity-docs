---
title: Run a Block-Producing Collator
description: Learn how to run a Polkadot SDK parachain node in collator mode, including the node key, chain specification, session keys, and node maintenance.
categories: Nodes
---

# Run a Block-Producing Collator

## Introduction

A collator is a parachain node started in collator mode. Unlike RPC or archive nodes, which serve state, collators produce blocks: they collect transactions, build parachain block candidates, generate the Proof-of-Validity (PoV) that relay chain validators need to check them, and submit the candidates for inclusion. Collators keep both the parachain and the relay chain in sync and handle cross-chain messages through XCM. They provide liveness and censorship resistance; security comes from the relay chain validators through the [ELVES protocol](https://wiki.polkadot.com/learn/learn-parachains-protocol/).

This guide runs the `polkadot-parachain` binary with the `--collator` flag. It covers the node key, the chain specification, the collator startup flags, session key generation, and routine maintenance. The same flags work with [Polkadot Omni Node](/tools/omninode.md) for parachains that use it.

Which collators a parachain accepts is set by its runtime and governance. For the process to become a collator on a Polkadot system parachain, including invulnerables, bonds, and hardware expectations, see [Run a Block-Producing Collator](https://developers.polkadot.com/node-infrastructure/run-a-collator/) on the Polkadot Developer Docs.

## Prerequisites

Before you start, make sure you have the following:

- **Operating system**: Ubuntu 22.04 LTS (recommended) or a similar Linux distribution.
- **[Docker](https://www.docker.com/get-started/)**: Required for the Docker deployment option.
- **Open ports**: 30333 for parachain P2P and 30334 for relay chain P2P.

## Install the Binary

This guide provides two deployment options. Select the option that best fits your needs:

- **Docker**: Best for simpler setup and maintenance.
- **systemd**: Best for production environments requiring more control.

=== "Docker"

    1. Pull the Polkadot Parachain Docker image using the latest stable tag on [Docker Hub](https://hub.docker.com/r/parity/polkadot-parachain/tags):

        ```bash
        docker pull parity/polkadot-parachain:{{dependencies.repositories.polkadot_sdk.docker_image_version}}
        ```

    2. Verify the installation:

        ```bash
        docker run --rm parity/polkadot-parachain:{{dependencies.repositories.polkadot_sdk.docker_image_version}} --version
        ```

=== "systemd"

    1. Download the `polkadot-parachain` binary using the latest stable [Polkadot SDK release](https://github.com/paritytech/polkadot-sdk/releases):

        ```bash
        wget https://github.com/paritytech/polkadot-sdk/releases/download/{{dependencies.repositories.polkadot_sdk.version}}/polkadot-parachain
        ```

    2. Make it executable and move it to your system path:
        
        ```bash
        chmod +x polkadot-parachain
        sudo mv polkadot-parachain /usr/local/bin/
        sudo chown root:root /usr/local/bin/polkadot-parachain
        ```

    3. Verify installation:

        ```bash
        polkadot-parachain --version
        ```

## Generate Node Key

--8<-- 'text/node-infrastructure/generate-node-key.md:introduction'

--8<-- 'text/node-infrastructure/generate-node-key.md:command'
    --8<-- 'text/node-infrastructure/generate-node-key.md:polkadot-parachain'

--8<-- 'text/node-infrastructure/generate-node-key.md:conclusion'

## Obtain the Chain Specification

The collator needs the parachain's chain specification file, saved as `chain-spec.json`. See [Obtain the Chain Specification](/nodes/run-a-node/parachain-rpc.md#obtain-the-chain-specification) for where to find it.

## Run the Collator

In the following commands, replace `INSERT_RELAY_CHAIN` with the relay chain the parachain connects to, as a built-in chain identifier (such as `polkadot`, `kusama`, `westend`, or `paseo`) or the path to a relay chain specification file. With Docker, place a relay chain specification file in `collator-data` and pass its path under `/data`.

Using your preferred deployment method, take the following steps to set up and run your collator:

=== "Docker"

    1. Create a directory for collator data and copy the chain spec and node key:

        ```bash
        mkdir -p collator-data
        cp chain-spec.json collator-data/
        cp node.key collator-data/
        ```

    2. Launch the collator using Docker:

        ```bash
        docker run -d --name polkadot-collator --restart unless-stopped \
          -p 30333:30333 \
          -p 30334:30334 \
          -p 9944:9944 \
          -p 9615:9615 \
          -v $(pwd)/collator-data:/data \
          parity/polkadot-parachain:{{dependencies.repositories.polkadot_sdk.docker_image_version}} \
          --collator \
          --chain=/data/chain-spec.json \
          --base-path=/data \
          --port=30333 \
          --rpc-port=9944 \
          --prometheus-port=9615 \
          --prometheus-external \
          --node-key-file=/data/node.key \
          --name="INSERT_YOUR_COLLATOR_NAME" \
          --blocks-pruning=256 \
          --state-pruning=256 \
          --database=paritydb \
          -- \
          --chain=INSERT_RELAY_CHAIN \
          --port=30334 \
          --sync=fast \
          --blocks-pruning=256 \
          --state-pruning=256 \
          --database=paritydb \
          --pool-limit=0 \
          --rpc-port=0
        ```

    3. View logs to monitor sync progress:

        ```bash
        docker logs -f polkadot-collator
        ```

=== "systemd"

    1. Create a dedicated user:

        ```bash
        sudo useradd -r -s /bin/bash polkadot
        ```

    2. Create the data directory, then copy your chain spec and node key into it:

        ```bash
        sudo mkdir -p /var/lib/polkadot-collator
        sudo cp chain-spec.json /var/lib/polkadot-collator/
        sudo cp node.key /var/lib/polkadot-collator/
        ```

    3. Set permissions:

        ```bash
        sudo chown -R polkadot:polkadot /var/lib/polkadot-collator
        ```

    4. Create a systemd service file:

        ```bash
        sudo nano /etc/systemd/system/polkadot-collator.service
        ```

    5. Add the following configuration:

        ```ini title="systemd/system/polkadot-collator.service"
        [Unit]
        Description=Parachain Collator
        After=network.target

        [Service]
        Type=simple
        User=polkadot
        Group=polkadot
        WorkingDirectory=/var/lib/polkadot-collator

        ExecStart=/usr/local/bin/polkadot-parachain \
          --collator \
          --chain=/var/lib/polkadot-collator/chain-spec.json \
          --base-path=/var/lib/polkadot-collator \
          --port=30333 \
          --rpc-port=9944 \
          --prometheus-port=9615 \
          --node-key-file=/var/lib/polkadot-collator/node.key \
          --name="INSERT_YOUR_COLLATOR_NAME" \
          --blocks-pruning=256 \
          --state-pruning=256 \
          --database=paritydb \
          -- \
          --chain=INSERT_RELAY_CHAIN \
          --port=30334 \
          --sync=fast \
          --blocks-pruning=256 \
          --state-pruning=256 \
          --database=paritydb \
          --pool-limit=0 \
          --rpc-port=0

        Restart=always
        RestartSec=10
        LimitNOFILE=65536

        [Install]
        WantedBy=multi-user.target
        ```

    6. Start the service:

        ```bash
        sudo systemctl daemon-reload
        sudo systemctl enable polkadot-collator
        sudo systemctl start polkadot-collator
        ```

    7. Check the status:

        ```bash
        sudo systemctl status polkadot-collator
        ```

    8. View logs:

        ```bash
        sudo journalctl -u polkadot-collator -f
        ```

??? interface "Configuration Arguments"

    - **`--collator`**: Enables block production mode.
    - **`--node-key-file`**: Uses the generated node key for stable peer ID.
    - **`--name`**: Your collator name, shown in logs and on any [telemetry](/nodes/run-a-node/relay-chain/full-node.md#telemetry) server the node reports to.
    - **`--blocks-pruning=256`**: Keeps the last 256 blocks.
    - **`--state-pruning=256`**: Keeps the state history of the last 256 blocks.
    - **`--database=paritydb`**: Uses ParityDB for better performance.
    - **`--sync=fast`**: Fast sync mode for the relay chain.
    - **`--pool-limit=0`**: Disables transaction pool on relay chain (not needed for collators).
    - **`--rpc-port=0` (relay chain)**: Disables RPC on the embedded relay chain node (not needed for collators).

Your collator must sync both the relay chain and parachain before producing blocks. The relay chain uses fast sync to speed up synchronization. Overall sync time depends on network bandwidth, disk I/O speed, and the size of each chain.

!!! note
    If you also operate relay chain nodes, such as RPC nodes, you can point the collator to their RPC endpoints using the `--relay-chain-rpc-urls` argument. This allows the collator to retrieve relay chain data from these nodes, reducing the resources required to sync and maintain the embedded relay chain node.

!!! warning
    Do not register your session keys until both chains are fully synced. Monitor sync progress using the commands in the [Log Management](#commands-for-log-management) section.

## Generate Session Keys

Session keys are cryptographic keys your collator uses to sign authorship information when producing blocks. They identify your collator on the network and must be registered on-chain before it can take part in block production.

Generate session keys by calling the `author_rotateKeys` RPC method on your node:

```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' \
  http://localhost:9944
```

The `result` field contains the hex-encoded public session keys. The node stores the private keys in its keystore.

If the chain's runtime requires a proof of key ownership at registration, call `author_rotateKeysWithOwner` instead and pass the owner data the runtime verifies, typically the account ID that registers the keys. It returns the session keys in `keys` and the ownership proof in `proof`:

```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeysWithOwner", "params":["INSERT_ACCOUNT_ID"]}' \
  http://localhost:9944
```

!!! warning "Save your session key output immediately"
    Each call generates new keys. It does not return previously generated keys, so if you lose the output, call the method again and register the new result.

## Register the Session Keys

On parachains that use [`pallet-collator-selection`](https://paritytech.github.io/polkadot-sdk/master/pallet_collator_selection/index.html) and `pallet-session`, such as those built from the standard templates, you register session keys with the `session.setKeys` extrinsic, signed by your collator's account. Pass the `keys` value from [Generate Session Keys](#generate-session-keys) and the `proof` value, or `0x00` as the proof if the runtime does not verify one. Whether your collator is then selected to produce blocks depends on the parachain's collator selection rules, which may be a governance-managed list or an on-chain candidacy. Check the parachain's documentation for its process.

Once your collator is selected, it starts producing blocks in the next session. Verify that it is active by checking the logs for block production messages such as "Prepared block for proposing" and "Imported #123". See [Commands for Log Management](#commands-for-log-management).

## Commands for Node Management

Use the following commands to manage your node:

=== "Docker"

    - **Stop container**:

        ```bash
        docker stop polkadot-collator
        ```

    - **Start container**:

        ```bash
        docker start polkadot-collator
        ```

    - **Remove container**:

        ```bash
        docker rm polkadot-collator
        ```

=== "systemd"

    - **Check status**:

        ```bash
        sudo systemctl status polkadot-collator
        ```

    - **Stop service**:

        ```bash
        sudo systemctl stop polkadot-collator
        ```

    - **Enable service**:

        ```bash
        sudo systemctl enable polkadot-collator
        ```

    - **Start service**:

        ```bash
        sudo systemctl start polkadot-collator
        ```

## Commands for Log Management

Efficient log management is essential to ensure collator performance and uptime. Use the following commands to help you manage logs to monitor and maintain your collator:

=== "Docker"

    - **View logs**:

        ```bash
        docker logs -f polkadot-collator
        ```

    - **View recent logs (last 100 lines)**:

        ```bash
        docker logs --tail 100 polkadot-collator
        ```

    - **Filter for errors**:

        ```bash
        docker logs polkadot-collator 2>&1 | grep -i error
        ```

    - **Filter for block production**:

        ```bash
        docker logs polkadot-collator 2>&1 | grep -i "imported"
        ```

=== "systemd"

    - **View recent logs**:

        ```bash
        sudo journalctl -u polkadot-collator -n 100
        ```

    - **Follow logs in real-time**:

        ```bash
        sudo journalctl -u polkadot-collator -f
        ```

    - **Filter for errors**:

        ```bash
        sudo journalctl -u polkadot-collator | grep -i error
        ```

    - **Filter for block production**:

        ```bash
        sudo journalctl -u polkadot-collator | grep -i "imported"
        ```

## Database Maintenance

Check database size periodically using the commands for your selected setup:

=== "Docker"

    ```bash
    # Replace with your mounted data directory path
    du -sh ./collator-data
    ```

=== "systemd"

    ```bash
    du -sh /var/lib/polkadot-collator
    ```

The collator node automatically prunes based on configuration.

## Updates and Upgrades

Updates can happen to either the runtime or the node binary. Runtime upgrades are applied on-chain and need no action on the node. Node upgrades require replacing the binary or image as follows:

=== "Docker"

    1. Stop and remove the container:

        ```bash
        docker stop polkadot-collator
        docker rm polkadot-collator
        ```

    2. Back up data (recommended):

        ```bash
        cp -r ./collator-data ./collator-data.backup
        ```

    3. Pull the new Docker image:

        ```bash
        docker pull parity/polkadot-parachain:INSERT_NEW_TAG
        ```

    4. Start the collator again with the `docker run` command from [Run the Collator](#run-the-collator), using the new image tag.

=== "systemd"

    1. Stop the service:

        ```bash
        sudo systemctl stop polkadot-collator
        ```

    2. Back up data (recommended):

        ```bash
        sudo cp -r /var/lib/polkadot-collator /var/lib/polkadot-collator.backup
        ```

    3. Download the new binary from [GitHub releases](https://github.com/paritytech/polkadot-sdk/releases):

        ```bash
        wget https://github.com/paritytech/polkadot-sdk/releases/download/INSERT_NEW_VERSION/polkadot-parachain
        chmod +x polkadot-parachain
        sudo mv polkadot-parachain /usr/local/bin/
        ```

    4. Verify `polkadot-parachain` version to confirm successful update:

        ```bash
        polkadot-parachain --version
        ```

    5. Restart the service:

        ```bash
        sudo systemctl start polkadot-collator
        ```

    6. Verify the service is running:

        ```bash
        sudo systemctl status polkadot-collator
        ```

## Where to Go Next

- To collect metrics and set up alerts, see [Monitor and Harden a Node](/nodes/monitor-and-harden.md).
- To test a collator against a local relay chain, see [Run a Parachain Network](/polkadot-sdk/testing/run-a-parachain-network.md).
