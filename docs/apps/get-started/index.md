---
title: Run a Host for Local Development
description: Run Polkadot Desktop as a local development Host, pair it with the Polkadot App on your phone with a QR scan, and set up the foundation for Polkadot Product development.
categories: Apps
page_badges:
  tutorial_badge: Beginner
---

# Run a Host for Local Development

## Introduction

A Polkadot Product runs inside a Host, which mediates chain access, storage, and signing through [TrUAPI](/apps/reference/protocol/truapi/index.md). For local development, you run [Polkadot Desktop](https://developers.polkadot.com/reference/apps/hosts/polkadot-desktop/) as your Host and pair it with the [Polkadot App](https://developers.polkadot.com/reference/apps/hosts/polkadot-app/) on your phone, which holds your key and approves signing. This page covers starting Desktop and pairing the two with a QR scan. About 10 minutes.

![Diagram: Polkadot Desktop displays a QR code that the Polkadot App scans; the App returns a session public key, and from then on every signing request routes from Desktop to the App for signature.](/images/apps/get-started/install-and-pair/pairing-flow.svg){: .browser-extension}

Polkadot Desktop never holds your private key. Your identity lives on the Polkadot [People Chain](/get-started/glossary.md#people-chain), your private key lives in the Polkadot App, and Polkadot Desktop only ever holds a derived session public key: enough to identify you and construct per-Product accounts, but not enough to sign anything on its own.

## Prerequisites

Before getting started, ensure you have:

- The Polkadot App installed on your phone with an account created (your developer identity and signing device for Polkadot Products). For end-user builds of the App and of Polkadot Desktop, see the host pages on [developers.polkadot.com](https://developers.polkadot.com/reference/apps/hosts/polkadot-app/)
- A workstation running macOS, Windows, or Linux
- A device (iOS or Android) with a working camera
- Network connectivity on both devices

--8<-- 'text/apps/network-operators.md'

## Start Polkadot Desktop

1. Install a build of Polkadot Desktop for your platform. See the [Polkadot Desktop page on developers.polkadot.com](https://developers.polkadot.com/reference/apps/hosts/polkadot-desktop/) for available builds.

2. Launch Polkadot Desktop. On first launch, Desktop opens into the pairing flow with a QR code. The network selector at the top right chooses the environment Desktop connects to.

    ![Polkadot Desktop login screen showing the pairing QR code and the network selector at the top right with Paseo Next V2 selected.](/images/apps/get-started/install-and-pair/install-and-pair-02.webp){: .browser-extension}

    !!! note "Skip for development"
        Development builds of Desktop also expose a **Skip (Dev only)** button on the login screen. Skipping the pairing drops you straight into Desktop without a paired signer, useful for inspecting Desktop or testing a local Product that does not require signing, but most development flows assume a paired Polkadot App.

## Pair Polkadot Desktop with the Polkadot App

Pairing is a one-time cryptographic handshake. Desktop displays the QR code, the App scans it, and the App returns a session public key that Desktop stores. From that point forward, Desktop knows who you are and can construct per-Product sub-accounts, but every signing prompt still routes back to the App for approval.

1. Leave the QR code visible on the Polkadot Desktop login screen.

2. In the Polkadot App, open the camera-scanning view and scan the QR code shown on Desktop. A **Link a new device?** prompt appears with the Desktop version details.

    ![Polkadot App showing the "Link a new device?" prompt listing Polkadot Desktop with Cancel and Link buttons.](/images/apps/get-started/install-and-pair/install-and-pair-03.webp){: .browser-extension}

3. Tap **Link** to confirm. The App briefly shows a **Connecting device...** state while the handshake completes.

    ![Polkadot App showing the "Connecting device..." state with a loading spinner.](/images/apps/get-started/install-and-pair/install-and-pair-04.webp){: .browser-extension}

4. Desktop transitions from the QR code to a **Completing pairing...** state.

    ![Polkadot Desktop showing the "Completing pairing..." state with a loading spinner where the QR code was.](/images/apps/get-started/install-and-pair/install-and-pair-05.webp){: .browser-extension}

5. Once the handshake completes, Desktop opens the main dashboard.

    ![Polkadot Desktop showing the paired dashboard with the address bar and Browse section listing available Products.](/images/apps/get-started/install-and-pair/install-and-pair-06.webp){: .browser-extension}

After pairing, your identity on the People Chain is bound to the Polkadot App for this Desktop session. Every subsequent signing request will route to the App, and you approve or reject each one on the signing device.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> **Build Guides**

    ---

    Load your Product from `localhost` in Desktop, then add capabilities one recipe at a time.

    [:octicons-arrow-right-24: Set Up Your Project](/apps/build/index.md#set-up-your-project)

-   <span class="badge external">External</span> **Get TestNet Tokens**

    ---

    Fund your account and request per-service allowances on the TestNet, on developers.polkadot.com.

    [:octicons-arrow-right-24: Continue](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/)

</div>
