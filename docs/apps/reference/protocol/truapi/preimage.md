---
title: Preimage Method Group
description: TrUAPI method group for submitting and looking up off-chain content that on-chain operations dereference by hash, with the Statement vs Preimage distinction.
categories: Apps, Reference
---

# Preimage

## Introduction

The Preimage method group lets a Product submit off-chain content that on-chain operations dereference by hash, and look that content up again. A typical use case is a governance proposal: the on-chain referendum points at a hash, the preimage is the bytes of the proposal call, and the bytes must be available when the referendum executes.

The Statement Store and Preimage surfaces look superficially similar because both submit bytes, return a hash, and let other parties reference that hash. They are not interchangeable. See [Statement vs Preimage](#statement-vs-preimage) for which to use when.

## Conceptual Contract

The group covers:

- **Submitting a preimage**: The Product submits bytes and receives the preimage key (its hash). The Host submits on the user's behalf, gated by the `PreimageSubmit` permission.
- **Looking up a preimage**: The Product subscribes to a key and receives the bytes once the Host finds them, or `null` until then.
- **Lifecycle**: Preimages stay available until the referencing operation completes or until they are explicitly cleaned up, unlike statements, which expire after their time to live (TTL).

In the [`@parity/truapi`](https://www.npmjs.com/package/@parity/truapi) client, the `preimage` domain exposes `submit` and `lookupSubscribe`. The Product SDK wraps them as a `PreimageManager` returned by `getPreimageManager()` in [`@parity/product-sdk-host`](/apps/product-sdk/host/), with `submit` and `lookup` methods.

!!! warning "Provisional"
    Supported preimage size limits, the exact relationship with Bulletin Chain storage, and any preimage-availability hooks for runtime introspection are still being finalized.

## Statement vs Preimage

| Property               | Statement (Statement Store)                                                             | Preimage                                                                                                     |
| :--------------------: | :-------------------------------------------------------------------------------------: | :----------------------------------------------------------------------------------------------------------: |
| What it is for         | Real-time signaling between users: chat, presence, typing indicators, multiplayer state | Off-chain content that an on-chain operation must dereference: governance proposals, scheduled call payloads |
| Lifetime               | Short TTL (default 30 seconds, gossip-distributed)                                      | Lives until the referencing on-chain operation has completed (or until explicitly cleaned up)                |
| Availability guarantee | Best-effort gossip; can be missed entirely if no peer was listening on the topic        | Must be available for the referencing operation to execute; storage is durable                               |
| Validation gate        | Allowance-gated by `pallet-statement-store` (`max_count` and `max_size` per account)    | Gated by the `PreimageSubmit` permission                                                                     |
| When to use            | Anything users observe in real time and accept losing if the network dropped it         | Anything an on-chain operation needs to read back later                                                      |

A useful rule of thumb: if your Product would still be correct after the bytes expire from network gossip, it is a statement. If the bytes need to be there in 30 minutes for a referendum to execute, it is a preimage.

## Host Requirements

A Host implementing this method group must:

- **Gate on `PreimageSubmit`**: Require the [`PreimageSubmit`](/apps/reference/protocol/truapi/permissions/#remote-operation-permissions) permission, prompting for it implicitly on the first submission if it has not been granted.
- **Return the key**: Resolve a successful submission to the preimage key, so the Product can reference it on chain.
- **Keep the bytes reachable**: Make the submitted bytes available for lookup when the referencing operation executes. Hosts commonly store them on the [Bulletin Chain](/apps/reference/infrastructure/bulletin-chain/).
- **Report lookups as they resolve**: Deliver `null` to a lookup subscription until the bytes are found, then deliver the bytes.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Statement Store**

    ---

    The other half of the Statement vs Preimage decision: content that belongs on the gossip-distributed signaling layer.

    [:octicons-arrow-right-24: Reference](/apps/reference/protocol/truapi/statement-store/)

- <span class="badge guide">Guide</span> **Store Data on Chain**

    ---

    The Product-side how-to for Bulletin Chain storage, which the preimage surface often uses underneath.

    [:octicons-arrow-right-24: Get Started](/apps/build/store-data-on-chain/)

</div>
