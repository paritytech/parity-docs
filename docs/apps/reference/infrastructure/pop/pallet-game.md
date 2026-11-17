---
title: pallet-game Reference
description: Provisional reference for pallet-game, the Proof of Personhood pallet intended for personhood-gated games and randomized selection using per-context aliases.
categories: Apps, Reference
---

# `pallet-game`

!!! warning "Provisional"
    The behavior described on this page has not been verified against the `pallet-game` source, and its dispatch surface (extrinsics, storage, events, and supported game types) is not final. Treat this page as a description of the pallet's intended role, not as a specification.

## Introduction

`pallet-game` is the Proof of Personhood pallet intended for personhood-gated on-chain games and randomized selection flows. Its design goal is _fair participation_: when the pallet selects from a set of entrants, each real person should have one entry, regardless of how many accounts they control, because entrants are scoped by alias rather than by account address.

Flows where "one entry per real human" is part of the fairness contract, such as raffles and one-person-one-entry contests, are the kind of use case the pallet targets.

## Intended Properties

The pallet is designed around three properties:

- **One entry per alias**: A user enters under their per-game alias (resolved through `under_alias`), and the pallet stores the alias. A second entry from the same user produces the same alias, so the pallet can detect it.
- **Verifiable selection**: Selection is meant to use randomness that an external party can check later, rather than a choice made by a privileged account after the fact.
- **Account unlinkability for state**: Game-side state is keyed by alias, so it cannot be linked back to a specific user account without that user's cooperation.

## How a Product Would Use It

A Product would typically:

- Submit entries for the current user through a call wrapped in `under_alias`, so the pallet sees the alias.
- Read game state through the standard chain client surface.

The pallet runs the game; the Product is the UI that lets users enter and see results.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **pallet-people**

    ---

    The foundational pallet that issues the aliases this pallet keys on.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/pop/pallet-people.md)

- <span class="badge learn">Learn</span> **Ring-VRF and Aliases**

    ---

    The mechanism that scopes aliases per context, which is what makes one entry per person possible.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/pop/ring-vrf-and-aliases.md)

</div>
