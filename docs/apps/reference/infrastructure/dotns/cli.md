---
title: dotNS CLI Reference
description: Reference for the dotNS command-line tool, covering name registration, contenthash updates, record management, and ownership transfers.
categories: Apps, Reference
---

# CLI

## Introduction

[`@parity/dotns-cli`](https://www.npmjs.com/package/@parity/dotns-cli) is the command-line tool for interacting with the dotNS registry: registering a `.dot` name, updating its `contenthash`, and transferring ownership. It is the canonical tool for these operations; the higher-level [Register and Publish](/apps/deploy-your-app.md) flow wraps it for the Polkadot Product setup track.

Names carry the TLD of the deployment the CLI targets, for example `.paseo` on the Paseo TestNet and `.dot` on Polkadot. Registrations do not expire and there is no renewal step. A name stays with its owner until it is transferred or deliberately released.

A Product developer building a typical publishing pipeline rarely calls the CLI directly, because the setup track handles the common path. The CLI is the right tool when you need a fine-grained, scriptable interaction (CI publishing, batch operations across multiple names, debugging a registration failure).

!!! info "CLI version"
    This page targets `@parity/dotns-cli` `{{ dependencies.javascript_packages.dotns_cli.version }}`, which supports dotNS protocol releases `0.8.0`, `1.0.0`, and `1.1.0`. Before it reads or signs, the CLI checks the protocol release the network declares, and it refuses a network that declares any other release, or declares none. Set `DOTNS_SKIP_VERSION_CHECK=1` to skip the check. The CLI is in active development and breaking changes between versions are expected.

## Command Families

The CLI groups its commands into six families that map onto the dotNS contract responsibilities:

- **Registration**: Commands that create a name record:

    - Register a name on the public path. Registration checks `PopRules` eligibility for the label and account before it commits, so an ineligible name fails before any transaction is sent.
    - Register a subname beneath a name you own.
    - Resume, inspect, or discard a commitment left by an interrupted registration.
    - Register a reserved name with `register domain --governance` once a whitelist grant names it for the owner.

- **Governance grants**: Commands that ask governance for a name the public path does not sell, such as a name of five characters or fewer:

    - Submit a referendum that puts a whitelist grant for a name on chain. Before it signs, `governance propose` checks that the name can be granted, dry-runs the grant as Root, shows every deposit and fee, and checks the balance. It signs only from an interactive terminal; `--dry-run` prints the call and its costs instead, for a multisig or proxy to submit.
    - Follow a grant by name, or a referendum by its index.
    - Request a name while the whitelist's request window is open, for governance to accept.

- **Records management**: Commands that read or change an existing name's records:

    - Set the `contenthash` to a new CID. This is what a Product owner runs after publishing a new bundle, so the `.dot` name points at the new version.
    - Set text records, and choose the primary name shown for an account.
    - Look up a name's owner and records, and check an account's personhood status.
    - Show a personhood name's reservation queue, and clear expired reservations from its front. Anyone can clear a queue; see [PopRules and Pricing](/apps/reference/infrastructure/dotns/poprules-pricing.md) for how reservations work.

- **Lifecycle**: Commands that change who controls a name or its escrow position:

    - Transfer a name that is not soulbound to another account.
    - Let another account manage and transfer a name on your behalf, and revoke that delegation.
    - Release a name into escrow, inspect its position, redeem it, withdraw the deposit, and claim it. See [Escrow and Deposits](/apps/reference/infrastructure/dotns/escrow.md).

- **Accounts**: Commands that manage the signing account and its on-chain mapping:

    - Manage the encrypted keystore the CLI signs with.
    - Show balances, map a Substrate account to its EVM address, and check a name's governance grant.

- **Bulletin storage**: Commands that upload a Product bundle to the [Bulletin Chain](/apps/reference/infrastructure/bulletin-chain/index.md) and track what was uploaded, producing the CID a `contenthash` points at.

## Installing and Authenticating

The CLI is distributed as an npm package and installs the `dotns` command. Install it globally or run it with `npx`. It requires Node.js 22.3 or later, or Bun. It targets one environment at a time, set with `--env` (alias `--network`) or the `DOTNS_ENV` environment variable: `paseo-v2` (Paseo TestNet), `previewnet`, or `polkadot`. Each environment is a stack descriptor bundled with the CLI, listing the chain endpoints, genesis hashes, and dotNS contract addresses, so a newer CLI release carries newer copies. Set the environment explicitly for every command or pipeline. To target an instance you deployed yourself, pass a stack descriptor with `--stack` instead, as shown in [Point the dotNS CLI at Your Deployment](/apps/reference/infrastructure/dotns/deploy-your-own.md#point-the-dotns-cli-at-your-deployment).

!!! warning "Default Environment Spends Real Funds"
    When neither `--env` nor `DOTNS_ENV` is set, the CLI targets `polkadot`, where registrations, transfers, and Bulletin uploads spend real DOT. Pass `--env paseo-v2` to work on the TestNet.

Operations that mutate state (registration, record updates, transfers) need an account that can sign the resulting Polkadot Hub transaction. The CLI signs with one of the following:

- **An encrypted keystore**: The default. `dotns auth` creates and manages keystore accounts, and `--account` selects one.
- **A mnemonic or key URI**: Passed with `--mnemonic` or `--key-uri`, or through the matching environment variables, which suits CI pipelines.
- **The Polkadot App**: `--signer qr` pairs the CLI with the Polkadot App by QR code, so the App signs each transaction. This signer is experimental.

With none of these configured, a command that signs stops before it connects. The CLI signs with the shared development account only when you pass `--allow-dev-account` or set `DOTNS_ALLOW_DEV_ACCOUNT=1`.

The `PopRules` check reads the personhood status of the account the name is registered to: the signing account, or the account passed with `register domain --owner`.

## Command Surface

The table lists every command in `{{ dependencies.javascript_packages.dotns_cli.version }}`. Run `dotns <command> --help` for each command's flags.

| Command | Subcommands | Purpose |
|:--------|:------------|:--------|
| `register` | `domain`, `subname`, `retry`, `clear`, `list` | Register a name or subname, and manage commitments cached by an interrupted registration. |
| `governance` | `propose`, `status` | Submit a referendum for a name's whitelist grant, and follow it. |
| `whitelist` | `request` | Request a name through the whitelist's request window. |
| `lookup` | `name`, `owner-of`, `transfer` | Show a name's records and owner, and transfer a name. |
| `content` | `view`, `set` | Read and set a name's `contenthash`. |
| `text` | `view`, `set` | Read and set a name's text records. |
| `primary` | `set`, `status` | Set and show the primary name for an account. |
| `delegate` | `set`, `revoke`, `status`, `records`, `records-status` | Let another account manage a name, or edit records on all of an owner's names. |
| `escrow` | `status`, `positions`, `release`, `redeem`, `withdraw`, `claim-withdrawal`, `balance`, `refunds list`, `refunds claim`, `refunds claim-batch` | Manage deposits, the release lifecycle, the pull-payment balance, and the time-locked refund ledger. |
| `pop` | `info` (alias `status`), `queue`, `expire` | Show an account's personhood status and pending names, show a personhood name's reservation queue, and clear expired reservations from it. There is no command to set a tier. |
| `store` | `claim`, `info`, `list`, `names`, `cids`, `get`, `set`, `delete`, `sync` | Manage the account's User Store and the names in its label store. |
| `account` | `address`, `info`, `map`, `is-mapped`, `grant` | Show account details and balances, map a Substrate account to its EVM address, and show a name's governance grant. |
| `auth` | `set`, `list`, `use`, `remove`, `clear` | Manage the encrypted keystore. |
| `bulletin` | `authorize`, `refresh`, `upload`, `status`, `history`, `history:remove`, `history:clear`, `verify` | Upload content to the Bulletin Chain and track the uploads. |

## Use the CLI from TypeScript

The package also exports the operations the CLI runs as typed functions, from `@parity/dotns-cli/core`. You build a context with `createDotnsContext` from a chain client, an SS58 origin, a `PolkadotSigner` you supply, and an environment; every operation, such as `registerName`, `setContentHash`, or `transferName`, takes that context. Contract addresses come from the bundled stacks, so callers never pass an ABI, address, or calldata.

Unlike the CLI, the functions do not check the network's protocol release on their own: call `checkProtocolVersion(ctx)` once after creating the context. To run against another stack, load it with `loadDotnsStack`, which takes the same link, URL, or file as `--stack`, and compare the chain's genesis hash with the stack's before using it. The [`@parity/dotns-cli` README](https://www.npmjs.com/package/@parity/dotns-cli) has a complete example.

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge guide">Guide</span> **Register and Publish**

    ---

    The higher-level publishing flow that wraps the most common CLI operations.

    [:octicons-arrow-right-24: Get Started](/apps/deploy-your-app.md)
    
- <span class="badge learn">Learn</span> **MainNet Contracts**

    ---

    The contract addresses the CLI targets with `--env polkadot`.

    [:octicons-arrow-right-24: Reference](https://developers.polkadot.com/reference/apps/infrastructure/dotns/mainnet-contracts/)

- <span class="badge learn">Learn</span> **TestNet Contracts**

    ---

    The Paseo TestNet contract addresses the CLI targets with `--env paseo-v2`.

    [:octicons-arrow-right-24: Reference](https://developers.polkadot.com/reference/apps/infrastructure/dotns/testnet-contracts/)

</div>
