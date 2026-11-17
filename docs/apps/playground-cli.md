---
title: playground CLI Reference
description: Reference for the playground CLI (pg), covering install, every command and flag, target environments, local files, environment variables, and exit codes.
categories: Apps, Reference
---

# playground CLI Reference

## Introduction

The [`playground` CLI](https://github.com/paritytech/playground-cli) builds a Polkadot Product, uploads it to the Bulletin Chain, registers a dotNS name for it, and optionally lists it in the Playground registry. The binary installs as `playground`, with `pg` as a short alias that runs the same binary, so `playground deploy` and `pg deploy` are interchangeable.

This page lists every command and flag, the environments the CLI can target, the files it reads and writes, and how it reports failure. For a guided first deploy, see the [Quick Start](/apps/quick-start.md). For a walkthrough of the interactive deploy prompts, see [Deploy Your App](/apps/deploy-your-app.md).

!!! info "CLI version"
    This page targets playground-cli `v0.48.5`. The upstream README describes the CLI as a prototype reference implementation, and breaking changes between versions are expected. Run `pg <command> --help` on the version you installed, and compare with the [playground-cli releases](https://github.com/paritytech/playground-cli/releases) and [`CHANGELOG.md`](https://github.com/paritytech/playground-cli/blob/main/CHANGELOG.md) before scripting around a flag.

## Install and Update

The installer supports macOS and Linux. On Windows, run it inside [WSL](https://learn.microsoft.com/windows/wsl/install) and follow the Debian or Ubuntu steps. On a fresh Debian or Ubuntu system, install the base prerequisites first:

```bash
sudo apt update && sudo apt install -y build-essential curl
```

Install the latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/paritytech/playground-cli/main/install.sh | bash
```

Set `VERSION` to install a specific release tag:

```bash
curl -fsSL https://raw.githubusercontent.com/paritytech/playground-cli/main/install.sh | VERSION=v0.48.5 bash
```

The [install script](https://github.com/paritytech/playground-cli/blob/main/install.sh) does the following:

1. Downloads the release binary for your OS and architecture to `~/.polkadot/bin/playground`.
2. Symlinks `playground` and `pg` into both `~/.polkadot/bin/` and `~/.local/bin/`, and adds the install directory to your shell profile.
3. Removes the legacy `dot` command left by earlier installs.
4. Runs `playground login` so you can finish setup without a second command.

To move an existing install to the newest release, run [`pg update`](#update). Print the installed version with `pg --version`.

## Global Behavior

### Target Environment

The commands that write on chain (`deploy`, `deploy-all`, and `decentralize`) take an `--env` flag. It accepts the following values:

- Environment IDs: `preview`, `paseo-next`, `paseo-review`, `paseo-next-v2`, `polkadot`, and `kusama`.
- Legacy aliases: `testnet`, which maps to `paseo-next-v2`, and `mainnet`, which maps to `polkadot`.

Only `paseo-next-v2` (Paseo Next v2) is wired in the [CLI's configuration module](https://github.com/paritytech/playground-cli/blob/main/src/config.ts), and it is the default. Any other value fails immediately with a "not yet supported" error and a non-zero exit code. On `paseo-next-v2`, dotNS names use the `.paseo` TLD and fees are paid in PAS. The CLI has no local-network mode. For network details, see [developers.polkadot.com](https://developers.polkadot.com/).

The Asset Hub, Bulletin, and People chain endpoints for each environment are compiled into the binary. The only runtime override is `DOT_BULLETIN_RPC`, described in [Environment Variables](#environment-variables).

### Builder Identity Check

Before doing any work, `deploy`, `deploy-all`, `decentralize`, `mod`, and `init` check whether the acting account is a verified builder, using the verifier contract that the Playground registry consults. The acting account is one of the following:

- The account set by `--suri`, when you pass it.
- The shared development publish account, when you pass `--signer dev` without `--suri`.
- Otherwise, the product account paired by `playground login`.

If no session is paired, the account is not verified, or the check cannot be completed, the command prints a notice explaining how to become a builder through `playground.dot` in Polkadot Desktop and exits with code `0` without changing anything.

### Signer Modes

Commands that sign take `--signer <mode>` with one of these values:

- `dev`: Signs with shared development keys, so no phone is needed. Pair it with `--suri` to pin a known keypair, such as `//Alice`. Anything registered this way is owned by that shared or development account, not by you.
- `phone`: Signs with the account paired by `playground login`. Each on-chain step waits for an approval in the Polkadot App.

`--suri <suri>` accepts a development name such as `//Alice` or a BIP-39 mnemonic.

### Update Notice

After a command finishes, the CLI checks for a newer release and prints an update notice to standard error if one exists. The check is skipped when standard output is not a terminal, for `pg update`, for `--version` and `--help`, and when `DOT_NO_UPDATE_CHECK=1` is set.

## Commands

### `login`

Runs first-time setup. Pairing and toolchain installation run at the same time, and account setup runs once both succeed.

```bash
pg login [-y]
```

1. **Pairing**: Prints a QR code to scan with the Polkadot App. Skipped when a session already exists in `~/.polkadot-apps/`.
2. **Toolchain**: Installs `git`, `curl`, a C linker (`build-essential`), `rustup`, the nightly toolchain, `rust-src`, `cargo-pvm-contract`, the Kubo `ipfs` binary, and `wget`. Existing installs are detected and skipped.
3. **Account setup**: Requests the `BulletInAllowance` and `SmartContractAllowance` allowances in a single approval on your phone. Granting `SmartContractAllowance` mints PGAS to your `playground.dot/0` product account, which creates and maps the account on chain.

Login never sends native tokens. Use [`pg drip`](#drip) for a TestNet top-up.

| Flag        | Description                                                                                      |
|-------------|--------------------------------------------------------------------------------------------------|
| `-y, --yes` | Skips QR pairing. The toolchain still installs, and account setup is skipped because no session exists. |

### `status`

Prints the signed-in product account in SS58 and H160 form, its PAS and PGAS balances, and the state of the Bulletin and smart-contract allowances. It is read-only and takes no flags.

```bash
pg status
```

### `drip`

Sends 1 PAS from a shared development funder account to your own signed-in product account, up to a total of 10 PAS. Once the balance is at or above 10 PAS, it skips. It takes no flags and has no address option.

```bash
pg drip
```

### `init`

Creates a new project from the Playground starter template. It is shorthand for `pg mod playground-template` and takes no flags.

```bash
pg init
```

### `mod`

Clones a moddable app's source into a new local project so you can customize and redeploy it. The interactive picker lists only apps published with `--moddable`. Targeting a non-moddable app by domain fails with a "this app is not moddable" error.

```bash
pg mod [domain] [--suri <suri>]
```

The source is downloaded as a tarball from `codeload.github.com`, so neither `git` nor `gh` is required. The new directory is named `<slug>-<6 hex chars>` and is initialized as an empty Git repository when `git` is on your `PATH`. If the source repository has a `quests.json` at its root, an interactive terminal shows a quest browser before cloning.

| Argument or flag | Description                                                                                          |
|------------------|------------------------------------------------------------------------------------------------------|
| `[domain]`       | App domain, with or without the TLD. Omit it to open the interactive picker.                         |
| `--suri <suri>`  | Checks builder status for this account instead of the paired session. `mod` itself never signs.      |

### `build`

Detects the package manager from the lockfile (pnpm, Yarn, Bun, or npm), installs dependencies when `package.json` declares any, and runs the project's `build` script. Without a `build` script, it falls back to `vite build`, `next build`, or `tsc`, depending on what the project has installed.

```bash
pg build [--dir <path>]
```

| Flag           | Description                                           |
|----------------|-------------------------------------------------------|
| `--dir <path>` | Project directory. Defaults to the current directory. |

### `deploy`

Builds the project, uploads the output to the Bulletin Chain, registers a dotNS name under the environment's TLD, and optionally publishes the app to the Playground registry.

```bash
pg deploy [options]
```

| Flag                              | Description                                                                                                                                                                 |
|-----------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `--signer <mode>`                 | `dev` or `phone`. See [Signer Modes](#signer-modes).                                                                                                                        |
| `--domain <name>`                 | dotNS label, with or without the TLD, such as `my-app` or `my-app.paseo`.                                                                                                   |
| `--buildDir <path>`               | Directory that holds the build output. Defaults to `dist`.                                                                                                                  |
| `--no-build`                      | Skips the build and deploys what is already in `--buildDir`.                                                                                                                |
| `--contracts` / `--no-contracts`  | Deploys and installs the project's contracts, then rebuilds the frontend, or skips that step.                                                                               |
| `--playground`                    | Publishes the app to the Playground registry.                                                                                                                               |
| `--private`                       | Publishes with owner-only visibility. Requires `--playground`.                                                                                                              |
| `--moddable` / `--no-moddable`    | Records the project's public GitHub `origin` URL so others can `pg mod` the app, or explicitly skips it. Requires `--playground`. Not moddable by default.                  |
| `--tag <tag>`                     | Category tag for the published app: `site`, `social`, `chat`, `utility`, `gaming`, `marketplace`, or `irl`. Requires `--playground`.                                       |
| `--suri <suri>`                   | Secret URI for the signer, such as `//Alice`.                                                                                                                               |
| `--env <env>`                     | Target environment. Defaults to `paseo-next-v2`. See [Target Environment](#target-environment).                                                                             |
| `--dir <path>`                    | Project directory. Defaults to the current directory.                                                                                                                       |
| `-y, --yes`                       | Runs non-interactively with defaults. Requires `--domain`. `--signer` defaults to `dev` and `--buildDir` to `dist`; contracts and Playground publishing are off unless passed. |

`deploy` runs in one of three modes:

- **Non-interactive**: When you pass `-y`, or pass all of `--signer`, `--domain`, `--buildDir`, `--playground`, and either `--contracts` or `--no-contracts`, no prompts are shown.
- **Interactive**: In a terminal, any value you did not pass is asked for in a prompt. [Deploy Your App](/apps/deploy-your-app.md) walks through each prompt.
- **Error**: Without a terminal (CI, a coding agent, or piped input) and without `-y` or the full flag set, the command exits with an error that names both options instead of prompting.

For example, a CI deploy with the development signer looks like this:

```bash
pg deploy -y --signer dev --suri //Alice --domain my-product --no-build
```

Passing `--moddable` or `--tag` without `--playground` is rejected before anything is uploaded. With `--moddable`, the deploy also fails if `origin` is unset, points to a private repository, or is not on GitHub. In a non-interactive deploy, `--no-build` with a missing `--buildDir` and `--signer phone` without a paired session both fail before any on-chain work.

`deploy` requires the Kubo `ipfs` binary on your `PATH`. `pg login` installs it; otherwise, install it from [docs.ipfs.tech/install](https://docs.ipfs.tech/install/). A memory watchdog aborts the deploy if the process exceeds 4 GB resident memory.

### `deploy-all`

Deploys several apps listed in a JSON manifest in one run. Builds run in parallel, while all on-chain work is serialized per signing account so concurrent deploys never reuse a nonce. The command is always non-interactive.

```bash
pg deploy-all --manifest apps.json --signer dev --playground --concurrency 3 --json
```

Each app entry takes `name`, `dir` (resolved relative to the manifest file), and `domain`, plus optional `buildDir` (relative to `dir`) and `skipBuild` overrides:

```json title="apps.json"
{
  "apps": [
    { "name": "arcade", "dir": "apps/arcade", "domain": "arcade" },
    { "name": "arcade-snake", "dir": "apps/arcade-snake", "domain": "arcade-snake" }
  ]
}
```

| Flag                  | Description                                                                                              |
|-----------------------|----------------------------------------------------------------------------------------------------------|
| `--manifest <path>`   | Required. Path to the JSON manifest.                                                                     |
| `--signer <mode>`     | `dev` or `phone`, applied to every app.                                                                  |
| `--buildDir <path>`   | Default build directory for apps that do not set their own.                                              |
| `--no-build`          | Skips the build for every app.                                                                           |
| `--playground`        | Publishes every app to the Playground registry.                                                          |
| `--private`           | Publishes with owner-only visibility. Requires `--playground`.                                           |
| `--suri <suri>`       | Secret URI for the signer, such as `//Alice`.                                                            |
| `--concurrency <n>`   | Maximum apps in flight at once. Defaults to `3`, capped at the number of apps.                           |
| `--json`              | Prints a per-app JSON summary (`name`, `status`, `domain`, `appUrl`, `appCid`, and more) to standard output. |
| `--env <env>`         | Target environment. Defaults to `paseo-next-v2`.                                                         |

One app's failure does not stop the others, but the command exits non-zero if any app failed.

### `decentralize`

Uploads an existing static site to the Bulletin Chain and registers a dotNS name pointing at it. The source is either a live URL to mirror or a local directory of built files. Pass `--site` or `--path` to run without prompts; omit both to start the interactive flow.

```bash
pg decentralize --site https://example.com
pg decentralize --path ./dist --dot my-site
```

| Flag              | Description                                                                                                   |
|-------------------|---------------------------------------------------------------------------------------------------------------|
| `--site <url>`    | HTTP or HTTPS URL of a live static site to mirror.                                                            |
| `--path <dir>`    | Local directory of built static files. Cannot be combined with `--site`.                                      |
| `--dot <name>`    | dotNS name, with or without the TLD. Omit it to generate a free random name.                                  |
| `--suri <suri>`   | Signs with this secret URI. Defaults to the session paired by `playground login`.                             |
| `--playground`    | Also publishes a minimal entry to the Playground registry. Off by default.                                    |
| `--tag <tag>`     | Category tag, from the same list as `deploy`. Requires `--playground`.                                        |
| `--moddable`      | Records the public GitHub origin of the `--path` repository. Requires `--path` and `--playground`.            |
| `--env <env>`     | Target environment. Defaults to `paseo-next-v2`.                                                              |

### `contract`

Runs the [Contract Dependency Manager](https://github.com/paritytech/contract-dependency-manager) (CDM) pipeline bundled in the CLI. See [Add a Smart Contract to Your Product](/apps/build/deploy-a-smart-contract.md) for the workflow and for when to run `cdm` directly.

`pg contract deploy` builds, deploys, and registers the project's contracts. It signs with the paired session by default.

| Flag                            | Description                                                                                                          |
|---------------------------------|----------------------------------------------------------------------------------------------------------------------|
| `--signer <mode>`               | `dev` or `phone`. With `dev` and no `--suri`, the signer falls back to `DOTNS_MNEMONIC`, then `MNEMONIC`, then a built-in development mnemonic. `--suri` cannot be combined with `phone`. |
| `--suri <suri>`                 | Secret URI for local signing.                                                                                        |
| `--features <features>`         | Cargo feature flags passed to the contract build.                                                                    |
| `--registry-address <address>`  | CDM registry contract address.                                                                                       |
| `--assethub-url <url>`          | Overrides the Asset Hub WebSocket URL.                                                                               |
| `--bulletin-url <url>`          | Overrides the Bulletin WebSocket URL.                                                                                |

`pg contract install [libraries...]` (alias `pg contract i`) installs CDM contract libraries, such as `@polkadot/reputation` or `@polkadot/reputation:3`, then writes `cdm.json` and CDM's generated outputs. Omit the libraries to install everything listed in `cdm.json`.

| Flag                            | Description                                                                 |
|---------------------------------|-----------------------------------------------------------------------------|
| `-n, --name <name>`             | Chain preset name: `polkadot`, `paseo`, `preview-net`, or `local`.          |
| `--assethub-url <url>`          | Asset Hub WebSocket URL.                                                    |
| `--ipfs-gateway-url <url>`      | IPFS gateway URL used to fetch contract metadata.                           |
| `--registry-address <address>`  | CDM registry contract address.                                              |

### `dotns`

Runs the [dotNS CLI](/apps/reference/infrastructure/dotns/cli.md) bundled in the binary (`@parity/dotns-cli` `0.8.0` in this release), passing every argument after `dotns` through unchanged and exiting with its exit code.

```bash
pg dotns --help
```

### `logout`

Signs out of the paired account. It tells the paired Polkadot App to drop the connection, then clears the session files under `~/.polkadot-apps/`. If the phone cannot be notified, the local files are still cleared and the phone shows a stale pairing until it reconnects. With no session, it does nothing.

```bash
pg logout
```

### `update`

Downloads the latest GitHub release for your OS and architecture and replaces the installed binary in place. It takes no flags.

```bash
pg update
```

## Files

The CLI reads and writes the following files:

- **`~/.polkadot/bin/`**: Install location of the `playground` binary and the `pg` alias. Symlinks also live in `~/.local/bin/`.
- **`~/.polkadot-apps/`**: Session files from `playground login` and the cached Bulletin allowance key. `pg logout` clears the session files.
- **`README.md`**: In the project root, becomes the app's detail page when you publish to the Playground registry. The interactive deploy asks you to confirm it first.
- **`cdm.json`**: Contract dependencies and resolved addresses, written by `pg contract install` and by `deploy --contracts`.
- **`apps.json`**: The manifest passed to `pg deploy-all --manifest`. Any filename works.
- **`quests.json`**: Optional, at the root of a moddable app's source repository. `pg mod` reads it to show the quest browser.

## Environment Variables

| Variable                | Effect                                                                                                                                           |
|-------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| `VERSION`               | Read by the install script. Installs the given release tag instead of the latest.                                                               |
| `DOT_NO_UPDATE_CHECK=1` | Disables the update notice.                                                                                                                      |
| `DOT_DEPLOY_VERBOSE=1`  | Prefixes deploy log lines with elapsed seconds.                                                                                                  |
| `DOT_MEMORY_TRACE=1`    | Prints a per-second memory sample (RSS, heap, external, and peak) to standard error. Combine it with `DOT_DEPLOY_VERBOSE=1` when reporting a memory issue. |
| `DOT_BULLETIN_RPC`      | Uses this Bulletin endpoint as the primary and keeps the built-in endpoint as a fallback. Used by the CLI's own failover tests.                   |
| `DOT_TELEMETRY`         | `1` opts in to telemetry and `0` opts out. Telemetry is off by default for external users.                                                       |
| `DOT_TAG`               | Marks a run as synthetic for telemetry, such as `e2e-local-smoke`.                                                                               |
| `SENTRY_DSN`            | Overrides the bundled Sentry DSN for local testing.                                                                                              |
| `DOTNS_MNEMONIC`, `MNEMONIC` | Fallback signer for `pg contract deploy --signer dev` when `--suri` is not passed.                                                          |

## Exit Codes

The CLI exits with `0` on success and `1` on failure. Some outcomes that stop a command are treated as soft and exit `0` with a notice instead of an error:

- `status` and `drip` with no paired session.
- `drip` when the shared funder account has no tokens left.
- A blocked [builder identity check](#builder-identity-check).
- Exiting the interactive deploy from the `README.md` confirmation or the moddable setup screen.
- `logout` with no session.

An unsupported `--env` value always exits `1`, before the builder identity check runs. `deploy-all` exits `1` if any app in the batch failed.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> **Quick Start with the CLI**

    ---

    Install the CLI, pair it with the Polkadot App, and deploy your first Product.

    [:octicons-arrow-right-24: Quick Start](/apps/quick-start.md)

-   <span class="badge guide">Guide</span> **Deploy Your App**

    ---

    Walk through each prompt of the interactive deploy and what it publishes.

    [:octicons-arrow-right-24: Deploy Your App](/apps/deploy-your-app.md)

-   <span class="badge guide">Guide</span> **Register a .dot Domain**

    ---

    Choose a name your account can register and manage it after the deploy.

    [:octicons-arrow-right-24: Register a .dot Domain](/apps/register-dot-domain.md)

</div>
