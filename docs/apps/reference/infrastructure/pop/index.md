---
title: Proof of Personhood Reference
description: Reference for Polkadot's Proof of Personhood infrastructure — Ring-VRF and aliases, plus per-pallet documentation for the pallets that gate logic on personhood.
categories: Apps, Reference
---

# Proof of Personhood

## Introduction

Proof of Personhood (PoP) is Polkadot's privacy-preserving way to confirm a user is a real human, without exposing _which_ human. A user completes PoP once (in the [Polkadot App](https://developers.polkadot.com/reference/apps/hosts/polkadot-app/)); from then on, a single proof unlocks personhood-gated features — the TestNet faucet, short `.dot` names, alias-gated checks inside a Polkadot Product, and on-chain payments routed through personhood-aware pallets.

For a Product developer, PoP shows up in two places:

- **As a primitive your Product calls into**: Through the Product SDK your Product requests an alias for the current user, has the user prove control of that alias against a challenge, and gates features on the result. The Polkadot App is where the proving happens; your Product never sees the underlying biometric or the user's identity record.
- **As a runtime origin your Product dispatches under**: Polkadot's PoP pallets accept calls under the `under_alias` origin, so your Product can submit on-chain operations that resolve to the user's alias inside the called pallet — pallet state stays unlinkable to the underlying account.

This reference is organized into two halves: the mechanism (Ring-VRF, aliases, the unlinkability property, how `under_alias` works) on its own page, and per-pallet reference for the pallets a Product is most likely to dispatch into.

## The Mechanism

[Ring-VRF and Aliases](/apps/reference/infrastructure/pop/ring-vrf-and-aliases.md) is the conceptual deep dive: how the PoP cryptography produces unlinkable per-Product aliases, what `under_alias` is at the runtime layer, and the privacy property the whole stack delivers.

## The Pallets

| Pallet                                                                   | What It Covers                                                                                                                        |
|--------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| [`pallet-people`](/apps/reference/infrastructure/pop/pallet-people.md)     | Personhood registration, alias-issuing, and the `under_alias` runtime origin. The foundational pallet the rest of the set depends on. |
| [`pallet-game`](/apps/reference/infrastructure/pop/pallet-game.md)         | Personhood-gated on-chain games and randomized selection flows.                                                                       |
| [`pallet-score`](/apps/reference/infrastructure/pop/pallet-score.md)       | Personhood-anchored reputation and scoring primitives.                                                                                |
| [`pallet-identity`](/apps/reference/infrastructure/pop/pallet-identity.md) | The identity record on People Chain associated with a PoP account.                                                                    |
| [`pallet-ubc`](/apps/reference/infrastructure/pop/pallet-ubc.md)           | Universal Basic Capacity — the per-person on-chain capacity primitives.                                                               |
| [`pallet-coinage`](/apps/reference/infrastructure/pop/pallet-coinage.md)   | The personhood-aware peer-to-peer payment surface behind the App's Coinage feature.                                                   |

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Ring-VRF and Aliases**

    ---

    The cryptographic and runtime mechanism — how PoP produces unlinkable per-Product aliases, what `under_alias` is, and the privacy property the whole stack delivers.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/pop/ring-vrf-and-aliases.md)

- <span class="badge learn">Learn</span> **pallet-people**

    ---

    The foundational pallet of the PoP set — registration, alias-issuing, and the `under_alias` runtime origin.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/pop/pallet-people.md)

</div>
