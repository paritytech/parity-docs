---
title: HOP Reference
description: Reference for the Hand-Off Protocol (HOP), a Substrate node service that holds ephemeral, addressed data for named recipients, with its RPC methods, limits, and promotion.
categories: Apps, Reference
---

# HOP

## Introduction

The Hand-Off Protocol (HOP) is a node-level service for Substrate collators that delivers ephemeral, addressed data between users. It works like a dead-drop with an expiry timer: a sender deposits data for named recipients, each recipient collects it, and the data is deleted once every recipient has acknowledged it or when its retention period ends.

HOP is implemented in the Polkadot SDK as two crates:

- **[`sc-hop`](https://paritytech.github.io/polkadot-sdk/master/sc_hop/index.html)**: The node component. It provides a disk-backed data pool, an RPC interface for submit, claim, and acknowledge operations, per-account rate limiting, and best-effort promotion of entries to on-chain storage.
- **[`sp-hop`](https://paritytech.github.io/polkadot-sdk/master/sp_hop/index.html)**: The runtime primitives. It defines `HopRuntimeApi`, which the node calls to check whether an account is authorized to submit and to promote pool data to on-chain storage.

HOP data is [ephemeral](/get-started/glossary.md#ephemeral): it lives in the node's pool, not in chain blocks, unless it is promoted.

!!! warning "Provisional"
    HOP is under active development in the Polkadot SDK, and its interfaces and defaults may change. This page reflects the `sc-hop` and `sp-hop` rustdocs on the Polkadot SDK `master` branch; check them for the current surface.

## How a Handoff Works

A handoff involves a sender, one or more recipients, and a collator with HOP enabled:

1. **The sender generates recipient keys**: For each handoff, the sender generates an ephemeral public key per recipient, and the public keys identify the recipients in the pool entry. Each recipient needs the matching private key to collect the data; how that key reaches the recipient is outside HOP.
2. **The sender submits**: The sender calls `hop_submit` with the data, the recipients' public keys, a timestamp, and a signature from the sender's account. The node checks the size limit, asks the runtime whether the account is authorized and within quota, verifies the signature, and stores the entry in its pool. The entry is keyed by the Blake2b-256 hash of the data.
3. **Each recipient claims**: A recipient calls `hop_claim` with the data hash and a signature made with its ephemeral private key. Claiming is a read-only download and does not change the entry.
4. **Each recipient acknowledges**: After receiving the data, the recipient calls `hop_ack` with the same hash and a signature made with its ephemeral key. This marks that recipient as claimed. Once every recipient has acknowledged, the node deletes the entry.

If not every recipient acknowledges, the entry stays in the pool until its retention period ends. Before expiry, the node's maintenance task tries to promote the entry to on-chain storage; entries that are not promoted are deleted when they expire.

## RPC Methods

The `HopApiServer` trait in `sc-hop` defines the RPC interface:

| Method | Parameters | Returns | Description |
|:--|:--|:--|:--|
| `submit` | `data`, `recipients`, `signature`, `signer`, `submit_timestamp` | `SubmitResult` | Submit data to the pool. `recipients` are SCALE-encoded `MultiSigner` values (ed25519, sr25519, or ecdsa), `signer` is the submitting account, and `submit_timestamp` is in milliseconds since the Unix epoch |
| `claim` | `hash`, `signature` | Bytes | Download data by its 32-byte hash. Read-only: it does not mark the recipient as claimed |
| `ack` | `hash`, `signature` | None | Acknowledge receipt. Marks the recipient as claimed and triggers cleanup once all recipients have acknowledged |
| `pool_status` | None | `PoolStatus` | Return pool statistics, including entry count and size |

Each signature is domain-separated, so a signature for one method cannot be reused for another:

- **Submit**: The sender's account signs `blake2_256(HOP_SUBMIT_CONTEXT || blake2_256(data) || submit_timestamp)`, with the timestamp in little-endian bytes. Binding the timestamp into the payload prevents replaying an old submission.
- **Claim and acknowledge**: The recipient signs the data hash, prefixed with `HOP_CLAIM_CONTEXT` or `HOP_ACK_CONTEXT`, using the ephemeral private key that matches one of the entry's recipient public keys.

An `ack` for an entry that has already been deleted returns `NotFound`. Treat this as a benign terminal state: the handoff is complete.

## Submission Checks and Errors

The node validates a submission in the following order, so that cheap checks run before runtime reads and signature verification:

1. Each recipient key must decode, and the number of recipients must not exceed `MAX_RECIPIENTS`.
2. The signer and signature must decode.
3. The data must not exceed the runtime's maximum promotion size.
4. The runtime must report that the account can submit data of that length. This fails for an unauthorized account or an exhausted per-account quota.
5. The signature must verify.
6. The entry must fit in the pool, within the per-account rate limits and quotas.

The `HopError` type covers these failures, including `InvalidRecipientKey`, `TooManyRecipients`, `InvalidSigner`, `InvalidSignature`, `InvalidHashLength`, `DataTooLarge`, `NotAuthorized`, and `NotFound`.

## Node Configuration

A collator runs HOP when started with `--enable-hop`. The pool stores metadata in a parity-db database and data as content-addressed blob files, under `<chain-data-dir>/hop` unless a different directory is configured.

`HopParams` exposes the following settings. The defaults come from the constants in `sc-hop`:

| Setting | Default | Description |
|:--|:--|:--|
| Maximum pool size | 10 GiB | Total size of the data pool |
| Maximum per-user size | 256 MiB | Hard per-account cap, not scaled by the number of active users |
| Retention period | 24 hours | How long an entry stays in the pool before it expires |
| Maintenance interval | 5 minutes | How often the node promotes near-expiry entries and cleans up expired ones |
| Promotion buffer | 2 hours | How long before expiry the node starts promoting an entry on-chain |
| Submit rate and burst | Set in `sc-hop` | Per-account limit on submit requests |
| Bandwidth rate and burst | Set in `sc-hop` | Per-account limit on submitted bytes |

Per-account rate limiting can be turned off with `--hop-disable-rate-limit`, which is intended for tests and development nodes. The node-wide `--rpc-rate-limit` flag applies on top of the HOP limits.

## Promotion to On-Chain Storage

When the runtime implements `HopRuntimeApi`, the node's maintenance task promotes entries that are close to expiry: it asks the runtime API to build a promotion extrinsic and submits it to the local transaction pool. Failed promotions are retried with back-off up to `MAX_PROMOTION_ATTEMPTS`, after which the entry is left to expire. The runtime rejects promotions whose submit timestamp is too far from on-chain time.

If the runtime does not implement `HopRuntimeApi`, the node logs a warning at startup and runs HOP without promotion.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge external">External</span> **`sc-hop` Rustdocs**

    ---

    The node component: data pool, RPC interface, rate limiting, and promotion.

    [:octicons-arrow-right-24: Reference](https://paritytech.github.io/polkadot-sdk/master/sc_hop/index.html)

- <span class="badge external">External</span> **`sp-hop` Rustdocs**

    ---

    The runtime API for authorization checks and promotion.

    [:octicons-arrow-right-24: Reference](https://paritytech.github.io/polkadot-sdk/master/sp_hop/index.html)

</div>
