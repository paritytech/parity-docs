---
title: Payment Method Group
description: TrUAPI method group for payment flows, including payment requests, Balances transfer extrinsics, and status tracking through the standard signing flow.
categories: Apps, Reference
---

# Payment

## Introduction

The Payment method group lets a Product accept or initiate payments. Payments are signed transactions like any other transaction; this group collects the shapes a Product is most likely to need around the standard `Balances.transfer_keep_alive` flow.

For most Product use cases, transfers are built with [`@parity/product-sdk-tx`](/apps/product-sdk/tx.md) and helpers in [`@parity/product-sdk-utils`](https://paritytech.github.io/product-sdk/api/utils/) (formatting, validation), with the same `signAndSubmit` round trip every other signed call goes through. Host-level payment requests, top-ups, and status tracking are reached through `getPaymentManager()` in [`@parity/product-sdk-host`](/apps/product-sdk/host.md).

## Conceptual Contract

The group covers:

- **Building a payment request**: The Product encodes what a merchant is asking for, including recipient, amount, and optional memo, into a payload the payer's Product can parse and pre-fill into a transfer form. No on-chain interaction happens at request-build time; the request is an application-layer payload.
- **Constructing a transfer extrinsic**: Given a validated recipient and a planck-denominated amount, building a `Balances.transfer_keep_alive` (or `transfer_allow_death` when the sender intends to drain the account) ready to sign.
- **Submitting and watching the payment**: The Product passes the extrinsic through the standard [Signing](/apps/reference/protocol/truapi/signing.md) surface and observes status transitions of `signing`, `broadcasting`, `in-block`, and `finalized`.

The Payment group does not introduce new signing semantics. Every payment transaction goes through the same per-request confirmation as any other signing request.

!!! warning "Provisional"
    Payment-request URL formats as a canonical wire format, asset-aware extensions such as multi-asset transfers, and any Host-UI payment flows (including Pocket) are not yet specified as part of this method group.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Signing**

    ---

    The signing flow every payment transaction goes through.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/signing.md)

- <span class="badge learn">Learn</span> **Transactions Package**

    ---

    The Product SDK package for building, submitting, and tracking transactions.

    [:octicons-arrow-right-24: Transactions](/apps/product-sdk/tx.md)

</div>
