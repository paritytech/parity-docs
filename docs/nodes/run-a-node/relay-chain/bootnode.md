---
title: Set Up a Bootnode
description: Learn how to configure and run a bootnode for any Polkadot SDK-based chain, including P2P, WS, and secure WSS connections, node keys, and proxies.
categories: Nodes
---

# Set Up a Bootnode

## Introduction

Bootnodes are essential for helping blockchain nodes discover peers and join the network. When a node starts, it needs to find other nodes, and bootnodes provide an initial point of contact. Once connected, a node can expand its peer connections and take on its role in the network.

This guide walks you through setting up a bootnode for any Polkadot SDK-based chain, configuring P2P, WebSocket (WS), and secure WebSocket (WSS) connections, and managing network keys. The examples use the `polkadot` binary. The same flags work with `polkadot-parachain`, `polkadot-omni-node`, and other Polkadot SDK node binaries. You also learn how to test that your bootnode is running and reachable by other nodes.

## Prerequisites

Before you start, you need to have the following prerequisites:

- Verify a working node binary, such as `polkadot`, is available on your machine. See [Install and Run a Polkadot Node](/nodes/run-a-node/relay-chain/full-node.md).
- Ensure you have nginx installed. See the [nginx installation guide](https://nginx.org/en/docs/install.html) if needed.
- Set up a virtual private server (VPS) or other dedicated server.

## Bootnode Access Channels

Bootnodes must be accessible through three key channels to connect with other nodes in the network:

- **P2P**: A direct peer-to-peer connection, set by the following flag:

    ```bash
    --listen-addr /ip4/0.0.0.0/tcp/INSERT_PORT
    ```

    This is not enabled by default on non-validator nodes like archive RPC nodes.

- **P2P/WS**: A WebSocket (WS) connection, also configured via `--listen-addr`.
- **P2P/WSS**: A secure WebSocket (WSS) connection using SSL, often required for light clients. An SSL proxy is needed, as the node itself cannot handle certificates.

## Node Key

A node key is the ED25519 key used by `libp2p` to assign your node an identity or peer ID. Generating a known node key for a bootnode is crucial, as it gives you a consistent key that can be placed in chain specifications as a known, reliable bootnode.

Starting a node creates its node key in the `chains/INSERT_CHAIN/network/secret_ed25519` file of the node's base path.

You can create a node key using:

```bash
polkadot key generate-node-key --file node.key
```

Pass the key to the node at startup with `--node-key-file node.key`.

Back up the node key. Once your bootnode's address is in a chain specification that is compiled into a node binary, changing the key means recompiling that binary.

## Run the Bootnode

A bootnode can be run as follows:

```bash
polkadot --chain INSERT_CHAIN \
  --name INSERT_BOOTNODE_NAME \
  --node-key-file node.key \
  --listen-addr /ip4/0.0.0.0/tcp/30310 \
  --listen-addr /ip4/0.0.0.0/tcp/30311/ws
```

Replace `INSERT_CHAIN` with a built-in chain identifier or the path to your chain specification file.

This assigns the p2p to port 30310 and p2p/ws to port 30311. For the p2p/wss port, a proxy must be set up with a DNS name and a corresponding certificate. The following nginx example enables p2p/wss on port 30312 by adding a proxy to the p2p/ws port 30311:

```conf title="/etc/nginx/sites-enabled/bootnode"
--8<-- 'code/node-infrastructure/run-a-node/bootnode/bootnode.conf'
```

## Test the Bootnode Connection

Replace `INSERT_BOOTNODE_DOMAIN` with the DNS name of your bootnode, which serves the proxy with a valid certificate, and `INSERT_PEER_ID` with its peer ID. Each of the following commands should output `syncing 1 peers`.

!!! tip
    You can add `-lsub-libp2p=trace` on the end to get libp2p trace logging for debugging purposes.

### P2P

```bash
polkadot --chain INSERT_CHAIN \
--base-path /tmp/node \
--name "Bootnode testnode" \
--reserved-only \
--reserved-nodes "/dns/INSERT_BOOTNODE_DOMAIN/tcp/30310/p2p/INSERT_PEER_ID" \
--no-hardware-benchmarks
```

### P2P/WS

```bash
polkadot --chain INSERT_CHAIN \
--base-path /tmp/node \
--name "Bootnode testnode" \
--reserved-only \
--reserved-nodes "/dns/INSERT_BOOTNODE_DOMAIN/tcp/30311/ws/p2p/INSERT_PEER_ID" \
--no-hardware-benchmarks
```

### P2P/WSS

```bash
polkadot --chain INSERT_CHAIN \
--base-path /tmp/node \
--name "Bootnode testnode" \
--reserved-only \
--reserved-nodes "/dns/INSERT_BOOTNODE_DOMAIN/tcp/30312/wss/p2p/INSERT_PEER_ID" \
--no-hardware-benchmarks
```