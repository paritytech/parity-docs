---
title: Deploy Your Own dotNS Instance
description: Build the dotNS contracts and deploy them to a local pallet-revive dev node, set the TLD and deposit, and point the dotNS CLI at your deployment.
categories: Apps, Reference
---

# Deploy Your Own dotNS Instance

## Introduction

dotNS is published as source in the [`paritytech/dotns`](https://github.com/paritytech/dotns) repository. Parity does not deploy it: every dotNS registry, including the ones on public networks, is an instance someone deployed from that source with their own parameters. See [One Deployment per Network](/apps/reference/infrastructure/dotns/index.md#one-deployment-per-network) for what that means for names and deposits.

This page deploys the full contract set to a local [Revive Dev node](/smart-contracts/dev-environments/local-dev-node.md) through its [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter.md), explains which values you choose as the deployer, and shows how to point the dotNS CLI at the result. The same pipeline targets any chain that runs `pallet-revive` and exposes an ETH-RPC adapter; the repository's [`DEPLOYMENT_CHECKLIST.md`](https://github.com/paritytech/dotns/blob/{{ dependencies.repositories.dotns.version }}/DEPLOYMENT_CHECKLIST.md) covers remote chains.

!!! warning "Unaudited reference code"
    The dotNS repository describes its code as unaudited and experimental, provided for research, experimentation, and developer education. Before you deploy it for real use, review the code, check its dependencies, and secure your own keys and environment, as the repository's [Security section](https://github.com/paritytech/dotns/blob/{{ dependencies.repositories.dotns.version }}/README.md#security) describes.

## Prerequisites

Before you start, make sure you have the following:

- A running Revive Dev node and ETH-RPC adapter, with the adapter listening at `{{ networks.local.eth_rpc_url }}`. Follow [Local Development Node](/smart-contracts/dev-environments/local-dev-node.md) to build and start both.
- [Foundry](https://getfoundry.sh/), for `forge` and `cast`.
- [Bun](https://bun.sh/) 1.2.6 or later, which runs the deploy pipeline.
- Git.

The repository's own checklist also lists Docker. Docker runs a containerized ETH-RPC adapter for remote chains; with the local adapter from the dev node guide you do not need it.

## Get the Source

Clone the repository and check out a release tag. This page uses `{{ dependencies.repositories.dotns.version }}`:

```bash
git clone https://github.com/paritytech/dotns.git
cd dotns
git checkout {{ dependencies.repositories.dotns.version }}
bun install
```

`bun install` also runs `setup.bash`, which fetches the pinned Solidity dependencies (`forge-std` and the OpenZeppelin libraries) into `lib/`.

Deploying from a release tag matters: the last pipeline stage declares the release on chain as `protocolVersion()` on the protocol registry, and the runner reads it from the tag the checkout sits on. From any other commit, export `DOTNS_RELEASE_TAG` (bare semver such as `1.1.0`), or the run aborts before deploying anything.

## Choose the Deployment Parameters

A dotNS instance has a handful of values its deployer decides. Some are fixed at deploy time, some can be changed afterwards by the contract owner, and some need a Substrate Root origin:

| Parameter | Where it is set | Default in `{{ dependencies.repositories.dotns.version }}` | Changing it later |
|:----------|:----------------|:--------------------|:------------------|
| TLD | `DOTNS_TLD` in `.env` | `dot` in `.env.example` | Not possible. The protocol registry is initialized with it and has no setter. |
| Registration deposit | `BASE_DEPOSIT` in `contracts/utils/DotnsConstants.sol`, passed to the `DotnsFlatPricing` constructor | `10 ether` (10 tokens at the 18-decimal EVM scale) | Deploy a new pricing model and register it on `DotnsCostModelRegistry` with `register`, or switch versions with `setCurrentVersion`. Both are owner-only. |
| Release cooldown | `ESCROW_COOLDOWN` in `DotnsConstants.sol` | 15 minutes | `updateCooldown` on `DotnsNameEscrow`. |
| Redeem window | `ESCROW_REDEEM_WINDOW` in `DotnsConstants.sol` | 1 day | `updateRedeemWindow` on `DotnsNameEscrow`. |
| Short-name switch | `PopRules` state | Off | `setShortNamesEnabled(bool)` on `PopRules`, Root origin only. |
| Name whitelist limits | Defaults in `DotnsConstants.sol` | See the source | Setters on `DotnsNameWhitelist`, Root origin only. |

Pick the TLD before you deploy, because a wrong TLD can only be fixed by deploying again. The TLD is a bare label without the leading dot, such as `dot`, `paseo`, or `test`. To deploy with a different deposit amount, edit `BASE_DEPOSIT` before you build. For what the deposit, the length bands, and the short-name switch do, see [PopRules and Pricing](/apps/reference/infrastructure/dotns/poprules-pricing.md) and [Escrow and Deposits](/apps/reference/infrastructure/dotns/escrow.md).

## Configure the Deployer

The pipeline signs with a Foundry keystore account. On the first run it imports a private key from a one-off `.env` file into the keystore, then deletes `.env` after a successful deploy.

1. Create the bootstrap file:

    ```bash
    cp .env.example .env
    ```

2. Set these values in `.env`:

    - **`PRIVATE_KEY`**: A funded key on the target chain. The Revive Dev node's development genesis funds and maps the Alith development account, whose private key is `0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133` (the `alith` key in the [`subxt-signer`](https://docs.rs/subxt-signer) crate's `eth::dev` module). Use it on a local dev node only.
    - **`ACCOUNT_PASSWORD`**: Any password. It encrypts the imported keystore account.
    - **`RPC_URL`**: Leave it as `paseo_local`. The alias resolves to `http://127.0.0.1:8545` in `foundry.toml`, which is the local adapter.
    - **`DOTNS_TLD`**: The bare TLD label for your instance.

The Revive Dev node reports chain ID `420420420`, which the pipeline maps to the manifest folder `deployments/paseo-local/`. Set `DEPLOYMENT_NETWORK` in `.env` to write the manifest to a different folder.

## Build, Test, and Deploy

1. Build the contracts and run the tests that do not need a live chain:

    ```bash
    forge clean && forge build
    forge test --no-match-path 'test/fork/**'
    ```

2. Run the deploy pipeline:

    ```bash
    bun run deploy
    ```

    The runner imports the key, then runs five stages in order, each as a separate `forge script` invocation: `DeployCore`, `DeployRecords`, `DeployPolicy`, `DeployPopSystem`, and `WireDeployments`. After each stage it checks that every address in the manifest has bytecode; on a failure it restores the previous manifest and stops. A successful run ends with `=== Pipeline complete ===` and `Deleted one-off env file: .env`.

    If transactions are dropped or a stage runs out of gas, rerun the runner with slower submission and a longer timeout:

    ```bash
    ./scripts/deploy/run.sh '--slow --timeout 1000'
    ```

3. Read the addresses from the manifest:

    ```bash
    cat deployments/paseo-local/420420420.json
    ```

    The manifest lists every contract, including `DotnsProtocolRegistry`, `DotnsRegistrar`, `DotnsRegistry`, `DotnsRegistrarController`, `DotnsPopController`, `PopRules`, the four resolvers, `DotnsNameEscrow`, `DotnsNameWhitelist`, `StoreFactory`, `Multicall3`, `DotnsCostModelRegistry`, and `DotnsFlatPricing`. For their roles, see [Architecture](/apps/reference/infrastructure/dotns/architecture.md).

4. Confirm the escrow's redeem window is set, since a zero value makes every release revert:

    ```bash
    cast call INSERT_ESCROW_ADDRESS 'redeemWindow()(uint256)' \
      --rpc-url {{ networks.local.eth_rpc_url }}
    ```

    With the default constants, the call returns `86400`.

For the remaining post-deploy checks, including the release metadata verification, see [`DEPLOYMENT_CHECKLIST.md`](https://github.com/paritytech/dotns/blob/{{ dependencies.repositories.dotns.version }}/DEPLOYMENT_CHECKLIST.md).

## Dispatch Root-Only Calls

Opening the short-name band, issuing name grants, and changing whitelist settings check for a Substrate Root origin rather than an EVM caller. The contracts read it from the `pallet-revive` System precompile, so a signed EVM transaction reverts with `NotRoot` or `NotGovernance`, the deployer included.

A Root call is a `Revive.call` extrinsic, carrying the contract address and the EVM calldata, dispatched with a Root origin. The Revive Dev node's development genesis sets `//Alice` as the sudo key, so on a dev node you wrap the call in `Sudo.sudo`. Encode the calldata with `cast`. For example, to open the short-name band on `PopRules`:

```bash
cast calldata "setShortNamesEnabled(bool)" true
```

Then submit `Sudo.sudo(Revive.call { dest: INSERT_POP_RULES_ADDRESS, value: 0, data: INSERT_CALLDATA, ... })` signed by `//Alice`, for example from the Polkadot.js Apps **Sudo** page connected to `{{ networks.local.ws_url }}`. The gas and storage-deposit limits belong to the `Revive.call` extrinsic, not to the contract call; dry-run the call to size them. The repository's [Name Grants section](https://github.com/paritytech/dotns/blob/{{ dependencies.repositories.dotns.version }}/DEPLOYMENTS.md#name-grants-whitelisting) gives the calldata for grants. On a network with governance instead of sudo, the same call is put up as a referendum.

## Known Limits on a Dev Node

`PopRules` reads each account's personhood status from a Proof of Personhood precompile at `0x000000000000000000000000000000000a010000`, backed by the alias-accounts pallet in the Asset Hub runtime. The Revive Dev node runtime does not include that pallet, so anything that reads personhood status, including pricing a public registration, does not get the answer it would get on Asset Hub. A dev node is suited to testing the deployment pipeline, record management, and Root dispatches. To exercise the registration paths, deploy to a chain whose runtime provides the precompile.

The personhood gateway path also needs the dotNS gateway pallet in the chain's runtime; see [`DotnsPopController`](https://github.com/paritytech/dotns/blob/{{ dependencies.repositories.dotns.version }}/README.md#dotnspopcontroller) in the repository README.

## Point the dotNS CLI at Your Deployment

The [dotNS CLI](/apps/reference/infrastructure/dotns/cli.md) resolves a network from a stack descriptor: a document listing the network's chain endpoints, genesis hashes, and dotNS contract addresses. Its `--env` values select descriptors bundled with the CLI, so they never reach your instance. To use your own instance, write a descriptor for it and pass it with `--stack`, or set `DOTNS_STACK`:

```bash
dotns lookup name INSERT_LABEL --stack ./INSERT_STACK_FILE.json
```

The descriptor follows the [Trinity stack descriptor format](https://github.com/paritytech/trinity-user-agents/blob/main/docs/rfcs/trinity-stack-descriptor.md). For a custom stack, the CLI (`@parity/dotns-cli` `{{ dependencies.javascript_packages.dotns_cli.version }}`) requires:

- An `AssetHub` chain entry, the chain where you deployed the contracts.
- A `dotns` address for each of these keys: `registrar`, `controller`, `registry`, `resolver`, `reverseResolver`, `popResolver`, `contentResolver`, `storeFactory`, `popRules`, `popController`, `nameEscrow`, and `multicall3`. Take them from your deployment manifest.

A stack without a Bulletin or People chain entry still runs the other commands; commands that need the missing chain fail and name it. A descriptor file is used as is, without a signature check. `--stack` cannot be combined with `--env`, and `--rpc` changes only the endpoint, not the contract addresses.

Before it reads or signs, the CLI checks the release the instance declares through `protocolVersion()` and refuses an unsupported one; `{{ dependencies.javascript_packages.dotns_cli.version }}` accepts `0.8.0`, `1.0.0`, and `1.1.0`. It also compares the chain's genesis hash with the descriptor. Set `DOTNS_SKIP_VERSION_CHECK=1` or `DOTNS_SKIP_CHAIN_CHECK=1` to skip either check. For the full environment variable list, see the [`@parity/dotns-cli` README](https://www.npmjs.com/package/@parity/dotns-cli).

## Where to Go Next

<div class="grid cards" markdown>

- <span class="badge learn">Learn</span> **Architecture**

    ---

    What each deployed contract is responsible for and how the protocol registry ties them together.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/architecture.md)

- <span class="badge learn">Learn</span> **PopRules and Pricing**

    ---

    The length bands, the deposit, and the short-name switch your instance enforces.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/poprules-pricing.md)

- <span class="badge learn">Learn</span> **CLI**

    ---

    The commands you can run against your instance once the CLI points at it.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/cli.md)

- <span class="badge learn">Learn</span> **dotNS Overview**

    ---

    How dotNS resolves `.dot` names and why each network has its own deployment.

    [:octicons-arrow-right-24: Reference](/apps/reference/infrastructure/dotns/index.md)

</div>
