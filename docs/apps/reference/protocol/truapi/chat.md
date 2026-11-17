---
title: Chat Method Group
description: TrUAPI method group for Host chat, including room and bot registration, typed message publishing, custom message rendering, and Host requirements.
categories: Apps, Reference
---

# Chat

## Introduction

The Chat method group lets a Product take part in a Host's chat surface: it can create rooms, register as a bot, publish typed messages, and render its own custom message types inline. The underlying transport composes the Statement Store for signaling with the Bulletin Chain for content that has to outlive the gossip time to live (TTL). The Chat group exposes a higher-level surface on top of that pattern.

The Product SDK reaches this group through `getChatManager()` in [`@parity/product-sdk-host`](/apps/product-sdk/host.md).

## Conceptual Contract

Methods in this group fall into three families:

- **Room and bot registration**: A Product registers as a bot inside a room and creates simple group chats via `host_chat_create_simple_group`, introduced in `v0.2`. The registration call is gated by the standard Permissions surface.
- **Publishing typed messages**: The Host accepts `Text`, `RichText`, `Actions`, `File`, `Reaction`, and `Custom` message types. It handles encoding, attaches the per-Product account context, and dispatches to the Statement Store and Bulletin Chain underneath.
- **Custom-message rendering (reverse subscription)**: `product_chat_custom_message_render_subscribe` lets the Host ask the Product to render its own `Custom` message types in place. The Host opens a subscription against the Product, and the Product responds with rendered output for each message it owns.

Messages a Product submits before bot registration completes are queued automatically. The Product does not need to track registration state.

In the [`@parity/truapi`](https://www.npmjs.com/package/@parity/truapi) client, the `chat` domain exposes `createRoom`, `registerBot`, `listSubscribe`, `postMessage`, and `actionSubscribe`. The `messageId` that `postMessage` returns is the correlation key for any action the message carries: a later `actionSubscribe` trigger names it.

!!! warning "Provisional"
    The `v0.2` method names above and the current `@parity/truapi` client names differ, and lifecycle semantics around room membership, the precise shape of typed-message payloads, and the rendering-subscription contract are still being finalized.

## Host Requirements

A Host implementing this method group must bound and screen what it forwards. The current `@parity/truapi` client documents these limits for posted messages:

- **Message text**: At most 16 KiB. Line breaks and tabs are kept; other control characters and bidirectional overrides are rejected.
- **Attachments**: At most 32 actions and 32 media items per message.
- **Custom payloads**: At most 256 KiB.
- **URLs**: At most 2 KiB, and either `https` or an inline raster image.
- **Identifiers and display names**: Normalized and screened.

A rejection reports `MessageTooLarge` when the body or custom payload is over budget, and `Unknown` with a reason naming the field otherwise.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Statement Store**

    ---

    The signaling layer Chat composes with the Bulletin Chain.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/statement-store.md)

- <span class="badge learn">Learn</span> **Host Package**

    ---

    The Product SDK package that exposes `getChatManager()` and the other Host-level capabilities.

    [:octicons-arrow-right-24: Host](/apps/product-sdk/host.md)

</div>
