---
title: Statement Store Allowance
description: How per-account statement quota is enforced — max_count for live statements and max_size for total bytes — and why submissions are gated rather than fee-priced.
categories: Apps, Reference
---

# Allowance

## Introduction

Submissions to the Statement Store are _not_ priced by transaction fees. The People Chain doesn't charge per statement; instead, every account that wants to publish needs an on-chain allowance record that caps how many live statements it may have at once and how many bytes those statements may total.

The allowance is the spam-resistance mechanism. Without it, a misbehaving account could flood the network's gossip layer with valid (and validly signed) statements that no fee model would meaningfully limit. With it, the network has a deterministic upper bound on per-account traffic that subscribers don't have to defend against at the application layer.

## What an Allowance Records

Each account's allowance is a `StatementAllowance` record with two limits, kept in runtime storage under the well-known `:statement_allowance:` key prefix:

- **`max_count`**: The maximum number of statements the account may have live in the pool at once. As statements expire (TTL elapses, channel replacement evicts an older value), they free up count for new submissions.
- **`max_size`**: The maximum total bytes across the account's live statements. Each submission's payload counts against this; as statements expire, their bytes free up.

The Polkadot SDK does not provide a call that grants allowances. The runtime decides who gets one, for example from a privileged call in one of its own pallets, using the `increase_allowance_by` helper from `sp-statement-store`. See [Grant Allowances from the Runtime](/apps/reference/infrastructure/statement-store/enable.md#grant-allowances-from-the-runtime).

Both limits are checked at validation (step 2 of the [lifecycle](/apps/reference/infrastructure/statement-store/lifecycle.md)). A submission that would exceed either is rejected at the pallet boundary before it ever enters the gossip layer.

## How the Limits Interact

A typical Product hits the _count_ limit before the size limit if it publishes many small messages, and the _size_ limit before the count limit if it publishes fewer but larger payloads. Two practical consequences:

- A chat-like Product (lots of small statements) should budget primarily against `max_count`. Each message stays live until its TTL elapses, so a chatty user can occupy several `max_count` slots simultaneously.
- A Product that publishes occasional large payloads (configuration blobs, large status updates) should budget primarily against `max_size`. The size limit is total-across-live, not per-statement, so a few big payloads can saturate the quota even if `max_count` is far from full.

A Product can lower its size pressure by moving content to the Bulletin Chain and publishing only a small statement carrying the resulting CID. The CID is short; the bytes live on Bulletin and don't count against the Statement Store allowance.

## Per-Statement Size Limit

Independent of the per-account allowance, each statement payload has a hard 512-byte limit (after JSON encoding) enforced by the SDK. Larger payloads throw `StatementDataTooLargeError` before submission. For content bigger than 512 bytes, the recommended pattern is the Bulletin Chain + CID composition above.

## Provisioning on TestNet

To get a Statement Store allowance on TestNet, see [Get TestNet Tokens](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/).

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Lifecycle**

    ---

    Where allowance validation sits in the statement lifecycle (step 2 — before gossip propagation).

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/statement-store/lifecycle.md)

- <span class="badge guide">Guide</span> **Get TestNet Funds**

    ---

    The setup step where TestNet allowances are provisioned.

    [:octicons-arrow-right-24: Get Started](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/)
</div>
