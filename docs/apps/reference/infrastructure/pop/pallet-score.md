---
title: pallet-score Reference
description: Provisional reference for pallet-score, the Proof of Personhood pallet intended for per-alias reputation scores that stay unlinkable to user accounts.
categories: Apps, Reference
---

# `pallet-score`

!!! warning "Provisional"
    The behavior described on this page has not been verified against the `pallet-score` source, and its dispatch surface (extrinsics, scoring rules, events, and storage) is not final. Treat this page as a description of the pallet's intended role, not as a specification.

## Introduction

`pallet-score` is the Proof of Personhood pallet intended for personhood-anchored reputation and scoring. It is meant to record per-alias scores that a Product, or another pallet, can read to answer questions such as "has this real person built up enough reputation in this context to be trusted with X?"

Because scores are keyed by alias, two scores belonging to the same user in different contexts are not meant to link to each other or to the user's account.

## Intended Data Model

Conceptually, the pallet maps an alias in a given context to a score:

- A user's alias in a specific context, such as a Product, has a score for that context.
- The same user has different aliases in different contexts, so their scores are stored under different keys and cannot be linked.

How scores are adjusted, and by whom, is not yet documented.

## How a Product Would Use It

- **Sybil-resistant reputation**: A reputation system that needs real-person reputation, rather than account reputation, would read the score for an alias.
- **Thresholds**: A Product could gate a feature on a score threshold in its context.

Scores are chain state, so anyone can read them. The alias does not reveal the underlying account, but it is observable to anyone who knows the context.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **pallet-people**

    ---

    The foundational pallet that issues the aliases this pallet keys on.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/pop/pallet-people.md)

- <span class="badge learn">Learn</span> **Ring-VRF and Aliases**

    ---

    The mechanism that makes the alias-scoping property work — why scores under different contexts cannot be linked.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/pop/ring-vrf-and-aliases.md)

</div>
