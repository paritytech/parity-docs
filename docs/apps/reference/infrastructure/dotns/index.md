---
title: dotNS Reference
description: Reference for dotNS, the name-service contracts that resolve .dot Product names to published bundles, with PopRules eligibility and contract architecture.
categories: Apps, Reference
---

# dotNS

## Introduction

dotNS is a set of name-service contracts for Products, authored by Parity, that anyone can deploy. A deployment is the registry that turns a human-readable `.dot` name like `awesome.dot` into the published Polkadot Product it points at. It's the lookup every Host runs whenever a user navigates to a `.dot` address.

If you have used a DNS provider, the role is similar: human-readable names map to content. dotNS exists for the guarantees DNS lacks. A name belongs to its holder alone, with no registry operator able to suspend or transfer it, and it resolves wherever the chain is reachable, with no server in the path. Resolution returns a [Content Identifier (CID)](/get-started/glossary.md#content-identifier-cid) for a Product bundle rather than an IP address. And eligibility for the shorter names is tied to [Proof of Personhood](/apps/reference/infrastructure/pop/index.md), so short, human-scale names go to people rather than to whoever arrives first with the most capital.

Four properties shape how a Product developer interacts with dotNS:

- **The registry is contract state**: Names, owners, and the content references they point at are stored as contract state on the chain where the contracts are deployed, such as Polkadot Hub.
- **Name resolution is content-addressed at the end**: A `.dot` name resolves to a CID, and the CID points at bytes on the [Bulletin Chain](/apps/reference/infrastructure/bulletin-chain/index.md) (or via an IPFS gateway). See [Name Mechanism](/apps/reference/infrastructure/dotns/name-mechanism.md).
- **Eligibility is set by PopRules**: A name's length places it in a band, and the band decides whether the name is open to anyone on the public path, available only through the personhood gateway, or not sold on the public path at all. Every name the public path admits pays the same refundable deposit. See [PopRules and Pricing](/apps/reference/infrastructure/dotns/poprules-pricing.md).
- **The architecture is a small set of cooperating contracts**: Not a single registrar — a set of contracts each handling a slice of the model. See [Architecture](/apps/reference/infrastructure/dotns/architecture.md).

For the Product-side how-to (registering a name, publishing your bundle), see [Register and Publish](/apps/deploy-your-app.md).

## One Deployment per Network

dotNS is deployed separately on each network, and three things follow from that.

- **The TLD is a deployment parameter**: The deployer sets each registry's TLD at deploy time, and each registry carries one TLD. For example, a name reads `awesome.paseo` on the `paseo` preset and `awesome.dot` on Polkadot. Tools take the bare label and append the TLD of the environment they target, so a label carrying a different TLD is rejected rather than translated.
- **The deposit is a deployment parameter**: The deployer sets the registration deposit, paid in the chain's native token, so the amount need not match between deployments. See [PopRules and Pricing](/apps/reference/infrastructure/dotns/poprules-pricing.md).
- **Registrations do not cross networks**: Each deployment keeps its own registry, so holding a name on a TestNet grants no claim to the matching name on Polkadot. A name has to be registered on each network where it is needed.

The contracts are the same in every deployment, but each deployer runs its own instance. To build the contracts and deploy an instance to a local dev node, see [Deploy Your Own dotNS Instance](/apps/reference/infrastructure/dotns/deploy-your-own.md). The [Polkadot Community Foundation](https://docs.polkadotcommunity.foundation/) operates the Products Devnet deployment and the deployment on Polkadot. For the addresses of existing deployments, see [MainNet Contracts](https://developers.polkadot.com/reference/apps/infrastructure/dotns/mainnet-contracts/) and [TestNet Contracts](https://developers.polkadot.com/reference/apps/infrastructure/dotns/testnet-contracts/).

!!! note "Scope"
    This reference covers the developer-facing surface. Operator-side procedures, such as migration, batch updates, and contract upgrade paths, are outside it.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Name Mechanism**

    ---

    How a `.dot` name resolves to a Product bundle — `namehash` derivation, the `contenthash` → CID mapping, and the IPFS/Bulletin delivery path.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/name-mechanism.md)

- <span class="badge learn">Learn</span> **Architecture**

    ---

    The contract architecture that backs the registry: what each contract is responsible for and how they cooperate.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/architecture.md)

- <span class="badge learn">Learn</span> **PopRules and Pricing**

    ---

    Which length band a name falls in, who may register it, and what the deposit is.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/poprules-pricing.md)

- <span class="badge guide">Guide</span> **Register and Publish**

    ---

    The Product-side how-to: registering a `.dot` name, attaching your published Product bundle, and going live.

    [:octicons-arrow-right-24: Get Started](/apps/deploy-your-app.md)

- <span class="badge guide">Guide</span> **Deploy Your Own dotNS Instance**

    ---

    Build the dotNS contracts, deploy them to a local dev node with your own TLD and deposit, and point the CLI at them.

    [:octicons-arrow-right-24: Get Started](/apps/reference/infrastructure/dotns/deploy-your-own.md)
</div>
