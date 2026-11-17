---
title: TrUAPI Reference
description: TrUAPI is the protocol between Polkadot Hosts and the Products that run inside them, covering signing, chain interaction, storage, and messaging.
categories: Apps, Reference
---

# TrUAPI

## Introduction

TrUAPI is the protocol every Polkadot Product uses to talk to the Host running it. It is the _only_ interface a Product has to the rest of the world. Every chain interaction, storage write, outbound request, signing operation, and Proof of Personhood proof crosses this boundary, and the Host on the other side enforces the sandbox.

A Host is any application that loads Products and implements TrUAPI. TrUAPI is the contract by which a Product talks to whichever Host is loading it, so any application that implements it faithfully can run Products. Most Product developers will never call TrUAPI directly. The [Product SDK](/apps/product-sdk/index.md) (the `@parity/product-sdk` family of packages) wraps it in typed methods. The methods you call still go through TrUAPI, and the Host validates and routes everything.

!!! note "TrUAPI and its packages"
    TrUAPI is the name of the protocol specification maintained at [`paritytech/truapi`](https://github.com/paritytech/truapi). It ships as two layers of packages: [`@parity/truapi`](https://www.npmjs.com/package/@parity/truapi) is the low-level generated protocol client (the wire), and the [`@parity/product-sdk`](https://www.npmjs.com/package/@parity/product-sdk) family is the higher-level SDK most Products use. This reference uses "TrUAPI" for the protocol surface and names the `@parity/product-sdk-*` package in the [Packages](/apps/reference/protocol/truapi/packages.md) table that wraps each group.

This reference documents the protocol surface in three layers:

1. **Conceptual model**: Why the Host-Product boundary exists, what derived sub-accounts mean, and how the sandbox enforces isolation. See [Sandbox and Sub-Accounts](/apps/reference/protocol/truapi/sandbox.md).
2. **Operational properties**: The protocol's versioning model and the packages that implement it. See [Versioning](/apps/reference/protocol/truapi/versioning.md) and [Packages](/apps/reference/protocol/truapi/packages.md).
3. **Method groups**: Eleven groups covering the capabilities a Product reaches for. One page per group is linked in [The Eleven Method Groups](#the-eleven-method-groups). A few host-lifecycle methods sit outside these groups; see the note below.

!!! warning "Provisional"
    The complete per-method reference for TrUAPI remains provisional. The method-group pages below cover what each group is for and the conceptual contract; the exhaustive per-method signatures, parameter shapes, and return types will be added as the surface stabilizes.

## Two Audiences

TrUAPI has two distinct readerships, and the per-method pages are written to serve both at once:

- **Product developers**: You build Products that call the TrUAPI surface. You need to know what each method does, what permissions gate it, what it returns, and how the Product SDK wraps it.
- **Host developers**: You build Hosts that implement the TrUAPI surface. You need to know what each method must guarantee, how to validate the Product's call, and how to enforce the sandbox at every entry point. Each method-group page ends with a Host Requirements section, and [Host Implementation Notes](#host-implementation-notes) covers constraints that cut across groups.

A Product developer can usually rely on the Product SDK (the [`@parity/product-sdk`](https://www.npmjs.com/package/@parity/product-sdk) family of packages) and never call TrUAPI directly. A Host developer is reading the same pages to know what the SDK calls underneath, and what their Host has to honor.

## The Eleven Method Groups

| Group                                                                                     | Covers                                                                                               |
|:-----------------------------------------------------------------------------------------:|:----------------------------------------------------------------------------------------------------:|
| [TrUAPI Calls](/apps/reference/protocol/truapi/calls.md)                    | The lifecycle calls Products use to identify themselves and the Host uses to introduce capabilities. |
| [Permissions](/apps/reference/protocol/truapi/permissions.md)               | Declaring, querying, and gating on Product capabilities.                                             |
| [Local Storage](/apps/reference/protocol/truapi/local-storage.md)           | The `KvStore` API for per-Product, per-device key-value persistence.                                 |
| [Account Management](/apps/reference/protocol/truapi/account-management.md) | Resolving per-Product sub-accounts and reading their state.                                          |
| [Signing](/apps/reference/protocol/truapi/signing.md)                       | The mediated signing flow for building transactions and dispatching them through the paired App.     |
| [Chat](/apps/reference/protocol/truapi/chat.md)                             | Room and bot registration, typed message publishing, custom message rendering.                       |
| [Statement Store](/apps/reference/protocol/truapi/statement-store.md)       | Publishing and subscribing to gossip-distributed signed statements on People Chain.                  |
| [Preimage](/apps/reference/protocol/truapi/preimage.md)                     | Off-chain content addressed by hash for on-chain operations to dereference.                          |
| [Chain Interaction](/apps/reference/protocol/truapi/chain-interaction.md)   | Reading chain state and subscribing to changes through the Host.                                     |
| [Payment](/apps/reference/protocol/truapi/payment.md)                       | Payment-flow primitives built on standard `Balances` transfers.                          |
| [Entropy](/apps/reference/protocol/truapi/entropy.md)                       | Deterministic per-Product entropy derived from the user's root key (key-derivation, not randomness). |

!!! note "Methods outside the eleven groups"
    A small number of host-lifecycle methods in the TrUAPI v0.2 spec are not part of any of the eleven groups above: `host_navigate_to`, `host_push_notification`, and `host_feature_supported`. They are Host-environment hooks rather than Product capabilities. The Product SDK reaches navigation and notifications through the [`host`](/apps/product-sdk/host.md) package (`navigateTo` and `getNotificationManager`).

!!! warning "Provisional"
    TrUAPI is specified as a single canonical surface, and the intent is that a Product runs unchanged across every Host. Today that holds for Hosts built on the shared TypeScript host container (SCALE, `host_*` calls), which Polkadot Desktop and Polkadot Web both use. The mobile Polkadot App exposes a hand-written JSON bridge whose method names differ from the spec and, in places, between iOS and Android. Treat "the same surface on every Host" as the conformance goal the spec defines, not a settled property of every shipping Host.

## Host Implementation Notes

The method-group pages each end with a Host Requirements section listing what a Host must guarantee for that group. A few constraints cut across groups and depend on where the Host runs:

- **Same surface everywhere**: The method groups, signatures, and SDK packages are the same on every conforming Host. Per-Product sub-accounts derive to the same addresses, permissions gate the same capabilities, and signing follows the same per-request approval model.
- **Local storage backing**: The Host chooses the storage backend behind [Local Storage](/apps/reference/protocol/truapi/local-storage.md). An installed Host can use on-device storage; a browser-based Host uses browser-managed storage, so capacity, eviction, and clear-history behavior follow the browser's rules.
- **Outbound requests in a browser**: A browser-based Host dispatches requests allowed by the [`Remote` permission](/apps/reference/protocol/truapi/permissions.md#remote-operation-permissions) from a browser context. CORS, cookies, and browser network policy apply on top of the Host's domain check, so a request the Host allows can still be blocked by the target server's CORS policy.
- **Media access in a browser**: Device permissions such as `Microphone` are gated by both the Host's permission system and the browser's own permission prompt. The user has to grant both, so Products should handle a two-prompt path.

A Product that uses none of the browser-dependent capabilities needs no Host-specific code.

!!! warning "Provisional"
    A per-method-group behavior matrix across Hosts is not yet available. The notes above cover the categories where behavior is known to differ.

## High-Level Diagram

!!! warning "Provisional"
    The canonical TrUAPI architecture diagram showing the Host, the Product, the SDK layers in between, and the eleven method groups as a single map, is still being finalized. It will be embedded here and referenced from the section overview once confirmed.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Sandbox and Sub-Accounts**

    ---

    Why a Product gets a derived sub-account rather than the user's root identity, and why the Host API is the only egress.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/sandbox.md)

- <span class="badge learn">Learn</span> **Versioning**

    ---

    How TrUAPI versions evolve and what compatibility a Product can rely on across them.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/versioning.md)

- <span class="badge learn">Learn</span> **Packages**

    ---

    The package map: which SDK package wraps which TrUAPI method group, and where the boundary between Product code and Host code lives.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/packages.md)

</div>
