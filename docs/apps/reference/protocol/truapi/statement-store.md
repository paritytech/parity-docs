---
title: Statement Store Method Group
description: TrUAPI method group for publishing and subscribing to gossip-distributed signed statements on the People Chain through the Host and SDK client.
categories: Apps, Reference
---

# Statement Store

## Introduction

The Statement Store method group lets a Product publish and subscribe to signed, gossip-distributed statements on the People Chain. The Statement Store itself, including the pallet, gossip layer, lifecycle, and validation rules, is a network-layer pub/sub primitive. It is short-lived, allowance-gated, and propagated peer-to-peer. This TrUAPI group is how a Product reaches it through the Host.

The Product SDK wraps this surface as `StatementStoreClient` from [`@parity/product-sdk-statement-store`](https://www.npmjs.com/package/@parity/product-sdk-statement-store). For the conceptual model of what statements are good for (versus the Bulletin Chain or local storage), see [Where to Store Data](/apps/concepts/data-placement/).

## Conceptual Contract

The group exposes the standard pub/sub surface with the Host filling in the parts a Product cannot do safely on its own:

- **Publishing a statement**: The Product hands the Host a payload (under the per-statement 512-byte limit), a topic, a channel, a time to live (TTL), and any options. The Host requests an authentication proof from the paired App, attaches the per-Product account context and primary topic (`blake2b(appName)`), and submits to the People Chain node's `statement_submit` JSON-RPC.
- **Subscribing to a topic filter**: The Product registers a callback for statements matching its primary topic plus any secondary `topic2`. The Host opens the underlying `statement_subscribeStatement` JSON-RPC, decodes payloads, and deduplicates by content hash before invoking the callback.
- **Channel-based last-write-wins**: When a statement carries a `channel`, the pallet replaces older statements from the same account on the same channel. The SDK abstracts this as `ChannelStore<T>` on top of the underlying method group.

Delivery is best-effort gossip, with no retry, acknowledgment, or ordering guarantee. Reliability is composed at the application layer. For example, if a recipient needs to confirm receipt, they publish an acknowledgment statement back.

!!! warning "Provisional"
    Detailed error taxonomy, allowance-management primitives, and any direct topic-walking API remain provisional.

## Host Requirements

A Product never talks to a Statement Store node directly. A Host implementing this method group must add what a Product cannot do safely on its own:

- **Route the proof**: Every submission carries an authentication proof. The Host obtains it from the user's signer (for example, a paired phone) rather than letting the Product produce it, then submits the statement with the proof attached.
- **Attach the account context**: Sign statements with the user's per-Product account, derived from the user's identity and scoped to the Product's `.dot` domain. The Product does not need to know the address up front.
- **Enforce the topic scope**: Apply the per-Product primary topic at the Host boundary, so a misconfigured Product cannot publish into another Product's topic.
- **Gate on `StatementSubmit`**: Require the [`StatementSubmit`](/apps/reference/protocol/truapi/permissions/#remote-operation-permissions) permission, prompting for it implicitly on the first submission if it has not been granted.

Some responsibilities stay with the Product, and a Host should not take them over:

- **Product-chosen settings**: The secondary `topic2`, the channel, and the per-statement TTL come from the Product's call.
- **Delivery semantics**: The Host does not add a queue, retry, or acknowledgment layer. Reliability is composed at the application layer.

When content needs to outlive the statement TTL, the usual pattern is to store it on the Bulletin Chain and publish the resulting CID as the statement payload. The same per-Product account context applies to both layers. See the [Statement Store infrastructure reference](/apps/reference/infrastructure/statement-store/) for the composition.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge guide">Guide</span> **Exchange Ephemeral Messages**

    ---

    The Product-side how-to: setting up the client, subscribing, publishing typed statements, and using channels.

    [:octicons-arrow-right-24: Get Started](/apps/build/pub-sub-off-chain-data/)

- <span class="badge learn">Learn</span> **Statement Store Infrastructure**

    ---

    The pallet, gossip layer, lifecycle, and validation rules behind this method group.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/statement-store/)

</div>
