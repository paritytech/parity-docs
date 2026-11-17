---
title: Register a .dot Domain
description: Register a .dot domain with dotNS, covering name rules, the commit-reveal flow, the three CLIs that can register a name, and managing your name.
categories: Apps
page_badges:
  tutorial_badge: Intermediate
---

# Register a `.dot` Domain

## Introduction

Every published Polkadot Product is reached by a dotNS name, such as `awesome.dot`. That name is registered with [dotNS](/apps/reference/infrastructure/dotns/), Polkadot's decentralized, on-chain name service. dotNS turns a human-readable name into the Product bundle it points at, and it is the lookup every Host runs when a user navigates to it.

--8<-- 'text/apps/network-tld.md'

The registry (names, owners, and the content record each name points at) lives as contract state on Polkadot Hub. Resolution runs `name → namehash → contenthash → CID`: the name hashes to a deterministic key, the record's `contenthash` points at your bundle's CID, and the Host fetches and content-verifies the bundle before loading it.

Registration happens on chain, so the name rules below hold whichever tool you use. Three CLIs can register a name: the [`playground` CLI](/apps/quick-start/) folds it into a deploy, `pad` publishes a built bundle and registers its name in one command, and the [dotNS CLI](/apps/reference/infrastructure/dotns/cli/) drives the registry directly. This guide explains how to choose a name your account is allowed to register, which path to take, and how to manage the name afterward.

## Prerequisites

Before registering, ensure you have:

- Completed [Run a Host for Local Development](/apps/get-started/). Your account needs funds on the target environment to pay fees and any name deposit; for TestNet funds, see [Get TestNet Tokens](https://developers.polkadot.com/apps/get-started/get-testnet-tokens/) on developers.polkadot.com.
- One of the CLIs in [Ways to Register](#ways-to-register) installed. On the `playground` path that means `pg login` has paired it with your signer.
- A Product project ready to deploy, if you are registering as part of a deploy with `playground` or `pad`. Registering with the dotNS CLI needs no bundle. See [Deploy Your App](/apps/deploy-your-app/).

## Choose a Name

A name is a bare label plus the network's TLD, as in `johnsmith57.paseo`. How you supply it depends on the tool: `playground` and the dotNS CLI take the bare label (`johnsmith57`) and append the TLD for you, rejecting a label that already carries a different one, while `pad` takes the full name.

### Label Rules

A label must satisfy all of these, or registration is rejected before anything is submitted on chain:

- **Length**: 3 to 63 characters.
- **Character set**: lowercase letters, digits, and dashes (`a-z`, `0-9`, `-`) only.
- **Dashes**: cannot start or end with a dash.
- **Digits**: allowed anywhere, in any quantity. A trailing run of digits no longer carries any special meaning.

!!! note "Out-of-date tooling is stricter"
    Releases built against the previous label rules cap a trailing run of digits at two and reject a longer one, which the chain no longer does. `@parity/dotns-cli` lifted the cap in `0.9.0`. If a label with three or more trailing digits is refused before anything is submitted on chain, update your tooling rather than changing the name.

### Name Length

Which names an account may register, and the deposit it pays, depend on the label's length (counted as written, digits included) and on the `PopRules` configuration of the dotNS deployment you target. On the configuration these guides target, a label of nine characters or more, such as `johnsmith57`, registers with no personhood check; shorter labels fail with [`Short names are not for sale`](/apps/troubleshooting/#short-names-are-not-for-sale). Proof of Personhood is obtained in the Polkadot App, not through any of these CLIs. See the [PopRules and Pricing reference](/apps/reference/infrastructure/dotns/poprules-pricing/) for the length bands, the deposit, and personhood names.

## Ways to Register

All three CLIs write to the same registry, so the name rules above apply identically to each. They differ in how much of the deploy they own:

|                 Tool                  |                    Best for                     |                                      What it does                                       |
|:-------------------------------------:|:-----------------------------------------------:|:---------------------------------------------------------------------------------------:|
| [`playground`](/apps/quick-start/)    | A first Product, and hackathons                  | Registers the name as one step of an interactive `playground deploy`                     |
| `pad`                                 | Scripted or CI deploys of a built bundle         | Uploads the bundle, registers the name if needed, and writes the content record in one command |
| [`dotns`](/apps/reference/infrastructure/dotns/cli/) | Managing names on their own, with no deploy | Registration, lookups, content records, transfers, subnames, and escrow                  |

The choice is not permanent. A name registered by any of them is an ordinary name owned by your account, and the others operate on it afterward.

## Register with the `playground` CLI

When you run `playground deploy` and reach the `domain` prompt, enter the name you want:

<div class="termynal" data-termynal>
<span data-ty><pre>  domain
    › johnsmith57█</pre></span>
</div>

In a scripted deploy, pass the same bare label with `--domain` instead, as in `pg deploy -y --signer dev --suri //Alice --domain johnsmith57`. See the [`playground` CLI reference](/apps/playground-cli/#deploy) for the deploy flags.

From there, the CLI registers the name on chain. If you deploy with the phone signer, each on-chain step is a separate approval in the Polkadot App, in this order:

1. **Reserve domain**: Submits a dotNS commitment for the name without revealing it in the clear.
2. **Finalize domain**: Claims the name for your account.
3. **Link content**: Points the name's `contenthash` at your uploaded bundle's CID, so the name now resolves to your Product.

!!! note "The ~60-second pause is expected"
    Between reserve and finalize, the deploy pauses for about 60 seconds. Most of that is the tooling waiting for the commitment to settle: the contract requires only that the commitment be six seconds old before the name is claimed. The two-step handshake is what stops a watcher seeing your desired name and racing to register it ahead of you. The deploy is not stuck.

!!! note "An abandoned commitment expires after a day"
    A commitment is valid for one day. If a deploy fails between reserve and finalize and you come back later than that, the commitment is no longer claimable and the handshake starts over from the beginning.

Names are first come, first served. If the CLI reports that a name is [already registered](/apps/troubleshooting/#the-name-is-already-registered), choose another; if it reports that [short names are not for sale](/apps/troubleshooting/#short-names-are-not-for-sale), pick a longer name. With the dev signer, these steps run without phone prompts; the deployed name is owned by the shared dev account rather than by you.

## Register with `pad`

!!! warning "Provisional"
    Ownership of `pad` is not settled in these docs: this page installs `@parity/polkadot-app-deploy`, while PCF also publishes `pad` tooling that publishes to Browse. Confirm which build you installed with `pad --help` before scripting around it.

[`polkadot-app-deploy`](https://github.com/paritytech/polkadot-app-deploy) publishes a built bundle and registers its name in a single command, which suits a scripted or CI deploy:

```bash
npm i -g @parity/polkadot-app-deploy
pad ./dist johnsmith57.paseo --mnemonic "$MNEMONIC"
```

`pad` uploads the bundle to the Bulletin Chain, skipping unchanged blocks on repeat deploys, registers the name if you do not already own it, and writes the content record on Polkadot Hub. It also has a `login` mode: on TestNet, a local worker registers the name and then transfers it to the signed-in account, so the deploy runs without phone taps unless you pass `--no-transfer-to-signedin-user`. `pad` does not read the dotNS keystore, so supply the key explicitly when you are not using `login`.

!!! warning "Keep the mnemonic out of your shell history"
    Pass the phrase through an environment variable, never as a literal on the command line, and never commit it. A deploy key that registers names controls them.

!!! note "Node 22 or newer"
    `pad` requires Node 22+ and fails at startup on older versions with an unrelated-looking error. It also describes itself as a prototype reference implementation, so check `pad --help` against the version you installed before scripting around it.

## Register with the dotNS CLI

The [dotNS CLI](/apps/reference/infrastructure/dotns/cli/) registers a name on its own, with no bundle and no deploy involved. Reach for it when you want to hold a name before the Product is ready, script a batch of names, or debug a registration that failed.

From `1.0.0`, the CLI targets Polkadot by default, where a registration spends DOT. To register on the TestNet this guide uses, set `DOTNS_ENV=paseo-v2` or pass `--env paseo-v2` on each command:

```bash
npm i -g @parity/dotns-cli
export DOTNS_ENV=paseo-v2
dotns register domain --help
```

Registration runs the same commit-reveal handshake described above, so expect a comparable pause whichever tool drives it. The CLI can also preview `PopRules` eligibility for a proposed name and account, showing the band and the deposit before you submit.

!!! warning "Provisional"
    The dotNS CLI's per-command flags are still being confirmed against the published package, which is why the [CLI reference](/apps/reference/infrastructure/dotns/cli/) lists command families rather than a flag table. Run `--help` on the version you installed rather than copying flags from elsewhere; the PCF publishes its own build of this tool under a different scope, and the two are not interchangeable.

## Update the Bundle a Name Points At

Registration binds the name to your account once. Publishing a new version of your Product does not re-register the name; it updates the name's `contenthash` to the new CID. Every path handles this the same way, and re-running against a name you already own skips the reservation steps, so later deploys need fewer approvals:

- **`playground`**: Re-run `playground deploy` against the same name.
- **`pad`**: Re-run the same command; it uploads only the changed blocks and rewrites the content record.
- **`dotns`**: Set the record directly with the CLI's `content set` command once you have the new CID, targeting the same network you registered the name on.

Because a name's content record is mutable and the owner can transfer or repoint it, treat a name as a pointer rather than a permanent identity. Code that consumes another Product should verify the `contenthash` it resolves at use time, not assume a name maps to the same bundle forever.

## Manage Your Name

Everything past registration is the [dotNS CLI](/apps/reference/infrastructure/dotns/cli/)'s job. A deploy tool repoints only the name it published; `dotns` operates on any name your account owns:

- **Transfer**: A name registered on this path is owned by a Polkadot Hub account and can be transferred to another account. A transfer changes only the owner; the name and its current content record are unchanged, so users keep seeing the same bundle until the new owner updates it. Proof of Personhood status and reservations do not transfer with the name. A device or personhood name cannot be transferred at all, and some moves carry a fee. See the [transfer reference](/apps/reference/infrastructure/dotns/transfer/).
- **Subnames**: The dotNS CLI can register subnames under a name you own.
- **Content records**: The dotNS CLI can view and set a name's content record outside a deploy.

!!! warning "Provisional"
    `@parity/dotns-cli` is in active development with breaking changes expected between versions, and `pad` describes itself as a prototype reference implementation. Both work today; treat the commands here as a snapshot and confirm them with `--help` before depending on them in a pipeline.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> **Deploy Your App**

    ---

    The full deploy flow that registers your name and uploads your bundle in one pass.

    [:octicons-arrow-right-24: Deploy Your App](/apps/deploy-your-app/)

-   <span class="badge guide">Guide</span> **Store Data on Chain**

    ---

    Keep the bundle your name points at available by renewing it on the Bulletin Chain.

    [:octicons-arrow-right-24: Store Data on Chain](/apps/build/store-data-on-chain/)

-   <span class="badge learn">Learn</span> **dotNS Reference**

    ---

    The name mechanism, PopRules pricing, contract architecture, and transfer model in depth.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/)

</div>
