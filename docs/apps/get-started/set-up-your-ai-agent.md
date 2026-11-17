---
title: Set Up Your AI Agent
description: Point your AI coding agent at the right skills, repos, and docs so it produces idiomatic Polkadot Product code instead of generic boilerplate.
categories: Apps
page_badges:
  tutorial_badge: Beginner
---

# Set Up Your AI Agent

## Introduction

Many developers build Polkadot Products with an AI coding agent. Out of the box, an agent does not know that Products run inside a Host, that chain access routes through the SDK rather than a direct RPC, or where the skills and repos live, so it falls back on generic patterns that do not work here. A few minutes of setup points the agent at the right context and tilts it toward correct, idiomatic code.

This page gives you a paste-ready setup prompt, then the resources to wire in: the skills, the repos, and the machine-readable docs.

## The Setup Prompt

Paste this at the start of a session to orient your agent. It states the toolchain, the repos, the docs, and the rules that most often trip agents up:

```text
You are helping me build a Polkadot Product: a sandboxed web app (HTML/JS/CSS)
that runs inside a Polkadot Host (Polkadot Desktop, the Polkadot App, or Polkadot
Web) and is addressed by a .dot name.

Toolchain:
- Build app code with @parity/product-sdk (TypeScript). Install its skills first.
- Deploy with the playground CLI (`pg` / `playground`).
- For smart contracts, use the Contract Dependency Manager (`cdm`); contracts compile
  to PolkaVM on pallet-revive (Asset Hub). Author them in Rust or Solidity (Solidity
  via resolc, Foundry, or Hardhat). Do not target mainnet-EVM/Ethereum tooling or
  legacy ink!/Wasm.

Docs (read before answering; append `.md` to any page URL for raw markdown):
- Guides: {{ config.site_url }}apps/
- CLI reference: {{ config.site_url }}apps/playground-cli/
- Full corpus: {{ config.site_url }}ai/llms-full.jsonl
- Index: {{ config.site_url }}llms.txt
- SDK API reference: https://paritytech.github.io/product-sdk/

Repos:
- https://github.com/paritytech/product-sdk (SDK packages + skills)
- https://github.com/paritytech/playground-cli (the deploy CLI)
- https://github.com/paritytech/contract-dependency-manager (contracts)

Rules to follow:
- All chain reads, signing, and storage route through the Host via @parity/product-sdk.
  Never open a direct WebSocket/RPC connection or dial a public IPFS gateway.
- Sign with a product account: getProductAccount(...).getSigner(). Approvals happen
  on the user's phone. Do not use Polkadot.js-style signing on chains that use the
  AsPgas signed extension, such as Paseo Next v2; it breaks there. Use the
  product-account signer.
- Most fallible SDK calls return a Result; check `.ok` before reading `.value`. Two
  exceptions: a contract read, `contract.method.query()`, returns `{ success, value }`,
  so check `.success`; and the `createApp` facade is not Result-typed except for
  `app.cloudStorage`; `app.wallet` and `app.chain` throw, and `app.localStorage`
  returns plain values. Prefer the individual packages (`signer`, `chain-client`,
  `local-storage`), which are Result-typed throughout.
- From `@parity/product-sdk` v0.30.0, call `createApp()` with no `name`: the Host's
  product ID supplies the identity, and a `name` is ignored with a warning. If the Host
  declines to derive an account, `wallet.connect()` resolves with zero accounts rather
  than erroring, so always check `accounts.length`.
- Storage: put files/blobs on the Bulletin Chain (content-addressed by CID, retained
  ~2 weeks, renewable). Keep bulk data OUT of contracts; store only small enforced
  state on-chain. Service allowances are granted per account.
- Prefer the SDK packages and the installed skills over generic JavaScript. If you are
  unsure how a surface works, read the docs page or the package README rather than guessing.
```

Trim the sections your project does not need. If you scaffolded from a template, much of this is already encoded in the project's agent files (see below).

## Install the Skills

Product SDK skills are small, modular instruction sets that teach an agent how to work across one slice of the Products stack (chain connections, Bulletin Chain storage, contracts, and more). When you ask for help in that area, the agent has the right context loaded, which tilts it toward idiomatic answers instead of generic boilerplate. The skills live in the [`paritytech/product-sdk`](https://github.com/paritytech/product-sdk) monorepo under `product-sdk/skills/`.

| Skill | What It Covers |
|:-----:|:--------------:|
| [`product-sdk-app-builder`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-app-builder) | End-to-end scaffolding and implementation of Polkadot apps using `@parity/product-sdk` packages. Foundational orchestrator skill; reach for this when starting a new project or scaffolding chain interactions. |
| [`product-sdk-cloud-storage`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-cloud-storage)             | Upload and retrieve data on the Polkadot Bulletin Chain. Covers CID-based content-addressed storage, the `CloudStorageClient` SDK, chunked uploads, CID computation, and the authorization pre-flight. Reads are container-only: the SDK does not fall back to a public IPFS gateway. |
| [`product-sdk-chain-connection`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-chain-connection) | Typed access to Polkadot chains via `@parity/product-sdk-chain-client` and `@parity/product-sdk-descriptors`. Covers preset and BYOD (bring-your-own-descriptors) paths, state queries, and storage subscriptions. |
| [`product-sdk-contracts`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-contracts)           | Smart contract interaction (PolkaVM / Solidity) on Asset Hub. Covers `ContractManager` with `cdm.json` manifests, ad-hoc `createContract`, `ContractRuntime`, and contract type codegen. |
| [`product-sdk-statement-store`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-statement-store) | Publish and subscribe to ephemeral messages on the Polkadot Statement Store. Covers `StatementStoreClient` lifecycle, host and local connection modes, channels with last-write-wins semantics, and the 512-byte size limit. |
| [`product-sdk-transactions`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-transactions)     | Submit transactions, manage signers, and derive keys. Covers `@parity/product-sdk-tx`, `@parity/product-sdk-signer`, and `@parity/product-sdk-keys`: multi-provider wallet accounts, Host API signing (Desktop/Mobile), and dev signers for testnet. |
| [`product-sdk-utilities`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-utilities)           | Foundational utilities: SS58/H160 address encoding, AES/ChaCha/NaCl crypto, HKDF key derivation, byte encoding, token (planck) formatting, key-value storage, and structured logging. |
| [`product-sdk-individuality`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/product-sdk-individuality) | Personhood and membership standing on the Individuality chain. Covers `readPersonhoodState` from a username or an account, `lookupUsername`, the seven-state `PersonhoodState` union, and `withAsPerson` for dispatching under a person origin. |
| [`migrating-to-product-sdk`](https://github.com/paritytech/product-sdk/tree/main/product-sdk/skills/migrating-to-product-sdk) | Migrating an existing codebase onto `@parity/product-sdk`. Detects legacy stacks (`polkadot-apps`, `novasamatech`, hand-rolled crypto or IPFS, `polkadot-api` 1.x) and writes a per-repo migration spec. Discovery and planning only; it does not edit code. |

### Claude Code

The repo ships a plugin marketplace, so no clone is needed. Add the marketplace, then install the plugin, which installs every skill in the table above at once:

```text
/plugin marketplace add paritytech/product-sdk
/plugin install product-sdk@paritytech
```

### Other Agents

For Cursor, Codex, Cline, Windsurf, or a custom harness, clone the repo:

```bash
git clone https://github.com/paritytech/product-sdk.git
```

Skills live under `product-sdk/skills/<skill-name>/` in the cloned directory. Register whichever skills are relevant using your agent's skill or context-loading mechanism.

### Invoke a Skill

Skills trigger based on context. You can also invoke a skill directly by name before asking a question, so the agent loads the SDK's API patterns before it answers. For example, before asking for help chunking a large upload to the Bulletin Chain:

```text
/product-sdk-cloud-storage how do I chunk a 50 MB file across multiple transactions?
```

### Keep Skills Current

Product surfaces are in active development. A stale skill teaches stale API patterns, which is worse than no skill, so refresh after major releases. In Claude Code, update the plugin:

```text
/plugin update product-sdk@paritytech
```

From a clone, pull the latest changes and re-register any updated skills with your agent:

```bash
git pull
```

## Start From a Template

The official Playground starter template ships agent-instruction files, so an agent picks up the conventions automatically:

- `CLAUDE.md` for Claude Code, `AGENTS.md` for Codex and other `AGENTS.md`-aware tools.
- `.clinerules`, `.windsurfrules`, and `.github/copilot-instructions.md` for Cline, Windsurf, and GitHub Copilot.

Coverage varies by app: a moddable app you clone with [`pg mod`](/apps/playground-cli/#mod) may ship all of these, only `CLAUDE.md`, or none, so check the project. If it includes a `setup.sh`, `pg mod` runs it for you to install dependencies and fetch the latest skills; many apps do not ship one.

## Give Your Agent the Docs

This documentation publishes machine-readable versions for AI tools:

- **`/llms.txt`**: An index of the documentation, following the [llms.txt convention](https://llmstxt.org/).
- **`/ai/llms-full.jsonl`**: Every page, pre-chunked by section, one JSON object per line.
- **Per-page markdown**: Append `.md` to any page path (for example, `/apps/quick-start.md`) to get its raw markdown, or use the **Markdown for LLMs** button on any page to copy it or hand the URL to your agent.

The [AI Resources](/ai-resources/) page collects these downloads.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> **Quick Start**

    ---

    With your agent set up, deploy your first Product from the terminal.

    [:octicons-arrow-right-24: Quick Start](/apps/quick-start/)

-   <span class="badge guide">Guide</span> **Build Guides**

    ---

    The capability recipes your agent will lean on, one per SDK package.

    [:octicons-arrow-right-24: Open Build Guides](/apps/build/)

-   <span class="badge external">External</span> **Product SDK Repository**

    ---

    The source of truth for the skills, including per-skill README files and updates.

    [:octicons-arrow-right-24: Visit Repo](https://github.com/paritytech/product-sdk)

</div>
