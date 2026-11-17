---
title: Build
description: Build Polkadot Product capabilities with product-sdk packages for signing, on-chain reads, decentralized storage, off-chain pub/sub, and local persistence.
categories: Apps
---

# Build

## Introduction

The Build section is a cookbook of focused recipes, one per [Product SDK](/apps/product-sdk/index.md) package; each recipe takes a single capability and walks you from an empty project to working Product code. They apply no matter how you started: a [Quick Start](/apps/quick-start.md) deploy or a project you [set up yourself](#set-up-your-project). Pick the capability your Product needs, in any order; the recipes are ordered by how little they ask of you, and the first one requires no account and no tokens.

For the concepts behind each package, which package each recipe uses, and how the umbrella and individual packages compare, see the [Product SDK overview](/apps/product-sdk/index.md). For the complete surface, see the [Product SDK API reference](https://paritytech.github.io/product-sdk/).

## Set Up Your Project

Every guide assumes a Product project running locally in [Polkadot Desktop](/apps/get-started/index.md). The steps below apply no matter how you bootstrapped your Product: a [Quick Start](/apps/quick-start.md) deploy or a project created from scratch.

1. Scaffold a project. Any framework that serves on `localhost` works; this example uses Next.js:

    ```bash
    npx create-next-app@latest my-product
    cd my-product
    ```

2. Install the [Product SDK](https://github.com/paritytech/product-sdk):

    ```bash
    npm install @parity/product-sdk
    ```

3. Start the dev server:

    ```bash
    npm run dev
    ```

4. Open Polkadot Desktop. If you have not paired it with the Polkadot App, click **Skip (Dev only)**, which development builds of Desktop show on the login screen.

    ![Polkadot Desktop showing the "Skip (Dev only)" button](/images/apps/build/start-a-local-dev-loop/start-a-local-dev-loop-01.webp)

5. Type `localhost:3000` (or your dev server's port) in the address bar and press **Enter**.

    Desktop recognizes `localhost` as a whitelisted origin, skips `.dot` resolution, and loads your Product directly from the local server.

    ![Polkadot Desktop showing a local Product loaded from localhost:3000](/images/apps/build/start-a-local-dev-loop/start-a-local-dev-loop-02.webp)

Your Product is now running inside the Polkadot Desktop sandbox, served from your local machine. Live reload works through your dev server's hot module replacement: edit, save, and watch the change appear in Desktop. The sandbox and permission prompts apply exactly as they would for a published `.dot` Product, and no Proof of Personhood or funds are needed to load from `localhost`.

## Capabilities

<div class="grid cards" markdown>

-   <span class="badge beginner">Beginner</span> **Read On-Chain Data**

    ---

    Your Product needs to show live balances, storage, and chain state. Reads are unsigned: no account, no tokens, no infrastructure to run.

    [:octicons-arrow-right-24: Read On-Chain Data](/apps/build/read-chain-state.md)

-   <span class="badge beginner">Beginner</span> **Sign and Submit Transactions**

    ---

    Your Product needs to act on chain on the user's behalf. Derive a per-user account and request signatures; every approval happens on the user's phone.

    [:octicons-arrow-right-24: Sign and Submit Transactions](/apps/build/sign-and-submit.md)

-   <span class="badge intermediate">Intermediate</span> **Store Data on Chain**

    ---

    Your Product needs content that outlives a session: profile photos, published posts, file uploads. Write to the [Bulletin Chain](/apps/reference/infrastructure/bulletin-chain/index.md) and fetch from anywhere by CID.

    [:octicons-arrow-right-24: Store Data on Chain](/apps/build/store-data-on-chain.md)

-   <span class="badge intermediate">Intermediate</span> **Publish and Subscribe to Off-Chain Data**

    ---

    Your Product needs real-time state between users: presence, typing indicators, multiplayer cursors. Signed pub/sub via the [Statement Store](/apps/reference/infrastructure/statement-store/index.md), no fees per message.

    [:octicons-arrow-right-24: Publish and Subscribe to Off-Chain Data](/apps/build/pub-sub-off-chain-data.md)

-   <span class="badge beginner">Beginner</span> **Persist Data Locally**

    ---

    Your Product needs to remember things on this device: preferences, drafts, cached values. Per-Product key-value storage backed by the Host, with no browser `localStorage` fallback.

    [:octicons-arrow-right-24: Persist Data Locally](/apps/build/persist-data-locally.md)

-   <span class="badge advanced">Advanced</span> **Add a Smart Contract to Your Product**

    ---

    Your Product needs enforced, shared on-chain logic: a leaderboard, a registry, an escrow. Deploy a PolkaVM contract to Asset Hub and call it by name from your frontend.

    [:octicons-arrow-right-24: Add a Smart Contract to Your Product](/apps/build/deploy-a-smart-contract.md)

</div>
