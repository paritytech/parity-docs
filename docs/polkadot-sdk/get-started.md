---
title: Get Started with Parachain Development
description: Practical tutorials for building parachains with the Polkadot SDK, from installing the toolchain and running the template to customizing, testing, and upgrading your runtime.
categories: Basics, Polkadot SDK
---

# Get Started

The following sections provide hands-on tutorials for building a parachain with the Polkadot SDK. Each tutorial runs locally and focuses on one part of the development workflow: setting up the template, building a runtime with FRAME and pallets, testing, upgrading, and enabling cross-chain messaging.

## Install and Run the Template

Start by installing the Polkadot SDK toolchain, then set up the Polkadot SDK Parachain Template and run it locally with the Polkadot Omni Node.

|                                             Tutorial                                             |       Tools        |                                Description                                 |
|:------------------------------------------------------------------------------------------------:|:------------------:|:--------------------------------------------------------------------------:|
|                    [Install Polkadot SDK](/polkadot-sdk/install-polkadot-sdk.md)                     | Rust, Polkadot SDK | Install the toolchain and dependencies needed to build Polkadot SDK chains |
| [Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template.md) |    Polkadot SDK    |  Learn how to set up and run the Polkadot SDK Parachain Template locally   |

## Build a Runtime

Build custom functionality for your parachain by composing FRAME pallets and creating your own.

<!-- INDEX TABLE START
dir: customize-runtime
flat: true
columns: [title, description, difficulty]
-->
<!-- INDEX TABLE END -->

### Pallet Development

Create, test, and benchmark custom pallets for your runtime.

<!-- INDEX TABLE START
dir: customize-runtime/pallet-development
flat: true
columns: [title, description, difficulty]
-->
<!-- INDEX TABLE END -->

## Test Your Parachain

Run your parachain alongside a local relay chain to test it end to end.

<!-- INDEX TABLE START
dir: testing
flat: true
columns: [title, description, difficulty]
-->
<!-- INDEX TABLE END -->

## Upgrade and Maintain Your Runtime

Ship new runtime logic with forkless upgrades and migrate on-chain storage safely.

<!-- INDEX TABLE START
dir: runtime-maintenance
flat: true
columns: [title, description, difficulty]
-->
<!-- INDEX TABLE END -->

## Enable Interoperability

Configure your parachain for cross-chain communication using Cross-Consensus Messaging (XCM). To set up the XCM executor, `pallet-xcm`, and message queues in your runtime, see [Configure XCM in Your Runtime](/polkadot-sdk/interoperability/configure-xcm.md).

<!-- INDEX TABLE START
dir: interoperability
flat: true
columns: [title, description, difficulty]
-->
<!-- INDEX TABLE END -->

## Reference

The [Polkadot SDK reference](/polkadot-sdk/reference/index.md) explains the concepts behind these tutorials, including the node and runtime architecture, accounts, blocks, transactions and fees, consensus, and data encoding.

## Deploy to a Live Network

To register your parachain on a relay chain and obtain coretime, see [Deploy on Polkadot](https://developers.polkadot.com/parachains/launch-a-parachain/deploy-to-polkadot/) on the Polkadot Developer Docs.

## Additional Resources

- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/index.html)
- [Polkadot Wiki: Parachains](https://wiki.polkadot.com/learn/learn-parachains/)
