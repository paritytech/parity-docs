---
title: TrUAPI Packages
description: Which SDK package wraps which TrUAPI method group, and where the boundary between Product code and Host code lives in the package reference.
categories: Apps, Reference
---

# Packages

## Introduction

A Product developer rarely calls TrUAPI directly. The [`@parity/product-sdk`](https://www.npmjs.com/package/@parity/product-sdk) family of packages is the typed wrapper, and most Products will install one of them per capability they consume. This page maps which package wraps which TrUAPI method group. Each Product SDK page links on to the how-to guides that use it.

!!! warning "Provisional"
    Package names and version pins remain provisional while the SDK matures. This page documents known packages and boundaries; exact versions and future package splits are outside the current scope.

## Package Map

Each row links to the Product SDK page for the package that wraps the group. Groups without a dedicated capability package are reached through the [`host`](/apps/product-sdk/host/) package, the foundation the other packages build on.

| TrUAPI Method Group | SDK Package |
|:-------------------:|:-----------:|
| [TrUAPI Calls](/apps/reference/protocol/truapi/calls/) | [`@parity/product-sdk`](/apps/product-sdk/) (core, used implicitly by every package) |
| [Permissions](/apps/reference/protocol/truapi/permissions/) | [`@parity/product-sdk-host`](/apps/product-sdk/host/) (`requestPermission`, `requestDevicePermission`); [`@parity/product-sdk-signer`](/apps/product-sdk/signer/) requests `ChainSubmit` on connect |
| [Local Storage](/apps/reference/protocol/truapi/local-storage/) | [`@parity/product-sdk-local-storage`](/apps/product-sdk/local-storage/) |
| [Account Management](/apps/reference/protocol/truapi/account-management/) | [`@parity/product-sdk-signer`](/apps/product-sdk/signer/) and [`@parity/product-sdk-address`](https://paritytech.github.io/product-sdk/api/address/) |
| [Signing](/apps/reference/protocol/truapi/signing/) | [`@parity/product-sdk-signer`](/apps/product-sdk/signer/) and [`@parity/product-sdk-tx`](/apps/product-sdk/tx/) |
| [Chat](/apps/reference/protocol/truapi/chat/) | [`@parity/product-sdk-host`](/apps/product-sdk/host/) (`getChatManager`) |
| [Statement Store](/apps/reference/protocol/truapi/statement-store/) | [`@parity/product-sdk-statement-store`](/apps/product-sdk/statement-store/) |
| [Preimage](/apps/reference/protocol/truapi/preimage/) | [`@parity/product-sdk-host`](/apps/product-sdk/host/) (`getPreimageManager`) |
| [Chain Interaction](/apps/reference/protocol/truapi/chain-interaction/) | [`@parity/product-sdk-chain-client`](/apps/product-sdk/chain-client/) and [`polkadot-api`](https://www.npmjs.com/package/polkadot-api) (PAPI) |
| [Payment](/apps/reference/protocol/truapi/payment/) | [`@parity/product-sdk-tx`](/apps/product-sdk/tx/) and [`@parity/product-sdk-utils`](https://paritytech.github.io/product-sdk/api/utils/); [`@parity/product-sdk-host`](/apps/product-sdk/host/) (`getPaymentManager`) |
| [Entropy](/apps/reference/protocol/truapi/entropy/) | [`@parity/product-sdk-host`](/apps/product-sdk/host/) |

`address` and `utils` are supporting packages without their own Product SDK page; their links go to the generated API reference.

## The Boundary Between Product Code and Host Code

A useful way to read the package map is by where the package executes:

- **Pure Product packages**: Packages such as `@parity/product-sdk-local-storage`, `@parity/product-sdk-signer`, `@parity/product-sdk-chain-client`, and `@parity/product-sdk-tx` execute inside the Product sandbox. They are typed wrappers that build a TrUAPI call and dispatch it through the Host API. Your Product depends on them at compile time.
- **Product-SDK umbrella**: `@parity/product-sdk` ties them together at the top of a Product and wires up the App / chain / storage surfaces with consistent defaults.
- **Host implementations of TrUAPI**: Host implementations live in the Hosts themselves, not in any Product-side package. A Product does not depend on Host code at compile time; it depends on the Host honoring the TrUAPI contract at run time.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Versioning**

    ---

    How package versions map to TrUAPI protocol versions.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/versioning/)

</div>
