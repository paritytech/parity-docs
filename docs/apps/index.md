---
title: Apps
description: Build Polkadot Products, sandboxed apps that run inside a Polkadot Host, with the Product SDK and the playground CLI.
categories: Apps
---

# Build a Polkadot Product

## Introduction

_Polkadot Products_ are sandboxed single-page apps that run inside a Polkadot Host. This section documents the software you use to build them: the [Product SDK](/apps/product-sdk/index.md), which gives your Product chain access, signing, storage, and messaging through the Host, and the `playground` CLI, which builds your Product, uploads the bundle, and registers its dotNS name.

You need the Polkadot App on your phone to pair with the CLI and to sign. For installation options and end-user builds, see the [Polkadot App page on developers.polkadot.com](https://developers.polkadot.com/reference/apps/hosts/polkadot-app/).

## Pick Your Path

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> **Deploy Your First Product**

    ---

    Install the `playground` CLI, pair it with the Polkadot App, and deploy a Product to a dotNS name from the terminal. No local Host setup required.

    [:octicons-arrow-right-24: Quick Start](/apps/quick-start.md)

-   <span class="badge guide">Guide</span> **Develop Locally**

    ---

    Run a Host for local development, pair it with your phone, then build your Product capability by capability.

    [:octicons-arrow-right-24: Get Started](/apps/get-started/index.md)

</div>

## What Is a Polkadot Product?

_Polkadot Products_ are what _you_ build: third-party applications that run _inside_ a Polkadot Host. Products are sandboxed single-page apps (HTML / JS / CSS), addressed by `.dot` names (for example, `awesome.dot`) registered on-chain through dotNS, and they never see the user's private key. The bundle itself is stored on the Bulletin Chain and fetched by the Host on demand.

_Polkadot Apps_ are the three applications that can host Polkadot Products, collectively known as the _Polkadot Triangle_:

- **[Polkadot Desktop](https://developers.polkadot.com/reference/apps/hosts/polkadot-desktop/)**: The workstation host. Loads Polkadot Products by their `.dot` name and runs them in a sandbox.
- **[Polkadot App](https://developers.polkadot.com/reference/apps/hosts/polkadot-app/)**: The mobile wallet and signer. Holds the user's private key and approves every signing request.
- **[Polkadot Web](https://developers.polkadot.com/reference/apps/hosts/polkadot-web/)**: The browser host at `dot.li`. Resolves `.dot` names client-side via a light client and renders the Product in a sandboxed iframe.

You build Polkadot Products. They run inside one of the Polkadot Apps. This section teaches you how.

![The Polkadot Triangle: Polkadot Desktop at the top, Polkadot App at the bottom-left, Polkadot Web at the bottom-right, with Polkadot Product running inside](/images/apps/index/polkadot-triangle.svg){ style="max-width: 480px; display: block; margin: 1.5rem auto;" }

For the architectural breakdown covering how the Product, SDK, Host, and Polkadot infrastructure relate, see the [App Development Reference](/apps/reference/index.md).

## What You Build With

The software in this section covers what you would otherwise integrate yourself:

- **[Product SDK](/apps/product-sdk/index.md)**: A TypeScript SDK that routes every sensitive operation through the Host. It covers chain reads and transaction submission (signing prompts open on the user's phone), content-addressed storage on the Bulletin Chain, signed pub/sub through the [Statement Store](/apps/reference/infrastructure/statement-store/index.md), typed calls to `pallet-revive` contracts on Asset Hub, derived per-Product accounts with optional [Proof of Personhood](/apps/reference/infrastructure/pop/index.md) gating, and per-Product local storage.
- **`playground` CLI**: Pairs with the Polkadot App, builds your Product, uploads the bundle to the Bulletin Chain, and registers its dotNS name. See the [Quick Start](/apps/quick-start.md) and [Deploy Your App](/apps/deploy-your-app.md).
- **Contract Dependency Manager (`cdm`)**: Builds, deploys, and registers PolkaVM contracts your Product calls by name. See [Add a Smart Contract to Your Product](/apps/build/deploy-a-smart-contract.md).

Your Product never handles keys or wallet integration. The Polkadot App on the user's phone is the signer for every Product, and per-Product accounts are derived from the user's `.dot` identity, so there is no signup flow.

Permissions (microphone access, outbound network requests, on-chain transaction submission, and more) are declared in your Product's [manifest](/apps/reference/protocol/truapi/permissions.md) and prompted at runtime. The Host enforces the boundary inside its [sandbox](/apps/reference/protocol/truapi/sandbox.md), not your code. The same `.dot` bundle runs in every Host, so you do not write a separate app per platform.
