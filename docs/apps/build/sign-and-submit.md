---
title: Sign and Submit Transactions
description: Derive product-scoped accounts and sign transactions from a Polkadot Product using the Product SDK, with all approvals routed to the Polkadot App.
categories: Apps
page_badges:
  tutorial_badge: Beginner
---

# Sign and Submit Transactions

## Introduction

This guide covers the `@parity/product-sdk-signer` package, which gives your Product a typed, host-aware signing interface for deriving accounts and signing payloads (raw bytes or full transactions) from inside a Polkadot host container. The SDK exposes a single orchestrator, `SignerManager`, that wraps both the production host path ([Polkadot Desktop](https://developers.polkadot.com/reference/apps/hosts/polkadot-desktop/)) and a deterministic dev path behind the same `Result`-typed API.

## Prerequisites

Before starting, ensure you have:

- A Polkadot Product project running locally (see [Set Up Your Project](/apps/build/index.md#set-up-your-project)) with a TypeScript toolchain
- Node.js 20 or later with ESM support (`@parity/product-sdk-signer` is ESM only)
- Polkadot Desktop to run your Product inside a host container (see [Run a Host for Local Development](/apps/get-started/index.md))

!!! note
    To test signing without a host container, use `manager.connect('dev')`. This loads the standard Substrate dev accounts (Alice, Bob, and others) locally and does not require Polkadot Desktop. See [Test Without a Host](#test-without-a-host).

!!! note "Code examples"
    The snippets in this guide are standalone; each one illustrates one concept and can be pasted into your Product independently.

!!! warning "Provisional"
    The signer snippets were last confirmed with Polkadot Desktop and `@parity/product-sdk` v0.9.0, while the rest of this section targets v0.30.0. Check them against the [Product SDK API reference](https://paritytech.github.io/product-sdk/) for the version you install.

## Install the SDK

--8<-- 'text/apps/install-packages.md'

- **Umbrella package**: The whole SDK in one dependency.

    ```bash
    npm install @parity/product-sdk
    ```

- **Individual package**: Only the signer.

    ```bash
    npm install @parity/product-sdk-signer
    ```

The import specifiers differ between the two. The snippets in this guide use the standalone specifier `@parity/product-sdk-signer`; on the umbrella, the same exports come from the `@parity/product-sdk/wallet` subpath, which keeps the older name. `SignerManager` is also re-exported from the umbrella root.

## How Product Accounts Work

Every product-scoped account is identified by a `ProductAccountId` tuple:

- **`dotNsIdentifier`**: The dotNS identifier of the Product requesting the account, for example the `.dot` name your Product is loaded under.
- **`derivationIndex`**: A non-negative integer chosen by the Product. Index `0` is the conventional default; higher indices give you additional sub-accounts within the same Product.

The same `(dotNsIdentifier, derivationIndex)` pair always yields the same public key, so your Product can reproduce the same address across sessions without persisting it.

!!! note "Per-Product isolation is a privacy feature"
    `app-a.dot` and `app-b.dot` produce different addresses for the same user. This prevents passive on-chain correlation across Products. Sharing an account across Products requires explicit permission; it is never the default.

## Set Up Signer Manager

Construct `SignerManager` once and hold it for the lifetime of your Product. Always set `ss58Prefix` and `dappName` explicitly. Use the `onConnect` callback to request any resources your Product needs immediately after connection; the callback fires exactly once per connect and re-fires automatically after any auto-reconnect.

```typescript
import { SignerManager } from '@parity/product-sdk-signer';

const manager = new SignerManager({
  ss58Prefix: 0,
  dappName: 'my-product',
  onConnect: async (_account, { requestResourceAllocation, signal }) => {
    try {
      const outcomes = await requestResourceAllocation([
        { tag: 'BulletinAllowance', value: undefined },
      ]);
      if (signal.aborted) return;
      if (outcomes.some((outcome) => outcome !== 'Allocated')) {
        // Degrade gracefully: the capability is unavailable, not fatal.
      }
    } catch (cause) {
      // Typed host error; the connection itself is unaffected.
    }
  },
});
```

- **`ss58Prefix`**: The SS58 address-format prefix for the target network. Use `0` for the Polkadot relay chain.
- **`dappName`**: A human-readable name for your Product, shown in Desktop UI.
- **`onConnect`**: Fires once per connection transition. Request the resources your Product needs here, before any signing call is made.
- **`ctx.requestResourceAllocation`**: The one call in this package that **throws** instead of returning a `Result`, so wrap it in `try`/`catch`. Each outcome is `'Allocated'`, `'Rejected'`, or `'NotAvailable'`; treat anything other than allocated as the capability being unavailable.
- **`ctx.signal`**: Aborts if the user disconnects while the request is in flight. Check it before acting on the outcomes.

!!! note "`AutoSigning` is not grantable yet"
    It is the resource most worth requesting and the one you cannot depend on: both the Android and iOS wallets return `NotAvailable` for it today. Keep per-transaction signing as the real path. See [Allowances and Permissions](/apps/concepts/allowances.md).

## Connect to the Host

Call `connect()` to establish a session with the host and discover available accounts. `connect()`, `selectAccount()`, `getProductAccount()`, and `signRaw()` all return a `Result`; always check `.ok` before accessing `.value`. (`getSigner()` returns a nullable directly, not a `Result`.)

Each step below is a self-contained function over the `manager` you built above, so you can paste them one at a time.

```typescript
async function connectAndSelect() {
  const connectResult = await manager.connect();
  if (!connectResult.ok) {
    // HostUnavailableError when running outside Desktop
    console.error(connectResult.error.message);
    return null;
  }

  const accounts = connectResult.value;
  if (accounts.length === 0) {
    // Connected, but the host derived no accounts for this Product.
    return null;
  }

  // Auto-select the first account if none is already selected
  if (!manager.getState().selectedAccount) {
    const selectResult = manager.selectAccount(accounts[0].address);
    if (!selectResult.ok) {
      console.error(selectResult.error.message);
      return null;
    }
  }

  return manager.getState().selectedAccount;
}
```

!!! warning "A successful connect can still yield zero accounts"
    `connect()` resolves with `ok([])` — not an error — when `dappName` is unset or the host rejects the derivation, typically because the `.dot` identifier is not registered for that user. Check the length before indexing, or `accounts[0].address` throws on a path the SDK treats as normal.

`connect()` defaults to the host provider. Outside a host container it returns `HostUnavailableError`; use `connect('dev')` for local testing instead. See [Test Without a Host](#test-without-a-host).

## Get a Product Account

`getProductAccount` requests the product-scoped account for a given `dotNsIdentifier` from the host. This is a host-only API; it returns `HostUnavailableError` when the active provider is `'dev'`.

```typescript
async function loadProductAccount() {
  const accountResult = await manager.getProductAccount('my-product.dot', 0);
  if (!accountResult.ok) {
    console.error(accountResult.error.message);
    return null;
  }

  return accountResult.value; // SignerAccount
}
```

The returned `SignerAccount` exposes:

- **`address`**: The SS58-encoded address for the current network.
- **`publicKey`**: The raw 32-byte public key.
- **`name`**: An optional display name, or `null`.

## Show a Display Name

There is no built-in primitive for an in-app username, so most Products need to choose a display identity themselves. [Usernames in Your Product](/apps/concepts/identity.md#usernames-in-your-product) covers the pattern to follow; this section covers the call it depends on.

When the user holds a personhood tier, the platform already associates an earned name with their account. Read it with `getUserId`:

```typescript
async function loadDisplayName() {
  const idResult = await manager.getUserId();
  if (!idResult.ok) {
    return null; // No host, no permission, or no personhood tier
  }

  return idResult.value.primaryUsername; // e.g. 'joseph.42'
}
```

When there is no earned name to read, fall back to a per-Product display name the user sets, stored in [local storage](/apps/product-sdk/local-storage.md) (device-local) or [cloud storage](/apps/product-sdk/cloud-storage.md) (shared) and keyed to their per-app account. Either way, treat the result as a label rather than as identity: authorization and uniqueness come from the per-app account and Proof of Personhood.

!!! warning "`getUserId` is host-only and prompts the user"
    `getUserId` returns `HostUnavailableError` when the active provider is `'dev'`, so a Product that renders a display name works in a Host and fails under the dev provider. It also triggers a host identity-permission prompt on first use. If your Product has its own display-name chain and does not need the platform name, skip the call rather than requesting a permission you will not use.

## Sign Arbitrary Bytes

Use `signRaw` to sign an arbitrary byte payload with the currently selected account. This is useful for off-chain authentication, message proofs, and any use case that does not require a full transaction.

`signRaw` returns a `Result<Uint8Array, SignerError>`; it never throws. Always check `.ok` before accessing `.value`.

```typescript
async function signMessage() {
  const account = await loadProductAccount();
  if (!account) return null;

  const selectResult = manager.selectAccount(account.address);
  if (!selectResult.ok) {
    console.error(selectResult.error.message);
    return null;
  }

  const payload = new TextEncoder().encode('hello polkadot');
  const result = await manager.signRaw(payload);

  if (!result.ok) {
    console.error(result.error.message);
    return null;
  }

  return result.value; // Uint8Array - the raw signature
}
```

## Sign and Submit a Transaction

To submit a transaction, combine three packages: the [chain client](/apps/product-sdk/chain-client.md) builds the transaction from a typed API, `SignerManager` supplies a `PolkadotSigner` for the selected account, and [`tx`](/apps/product-sdk/tx.md) signs, broadcasts, and follows it. The Host owns the chain connection, so your Product never opens a WebSocket or picks an RPC endpoint. The signing request routes to Desktop, which renders a signing modal and forwards it to the [Polkadot App](https://developers.polkadot.com/reference/apps/hosts/polkadot-app/).

Install the two extra packages. `tx` has no umbrella subpath, so install it on its own even if you use the umbrella package:

```bash
npm install @parity/product-sdk-chain-client @parity/product-sdk-tx
```

Get a typed client for the environment's preset chains with `getChainAPI`. Pass the environment name your Product targets, such as `'paseo'`:

```typescript
import { getChainAPI } from '@parity/product-sdk-chain-client';

const chain = await getChainAPI('paseo');
```

Then build a transaction from the typed API and submit it with `submitAndWatch`. This example posts a `System.remark` on Asset Hub:

```typescript
import { submitAndWatch } from '@parity/product-sdk-tx';
import { Binary } from 'polkadot-api';

async function postRemark() {
  const account = await loadProductAccount();
  if (!account) return null;

  const selectResult = manager.selectAccount(account.address);
  if (!selectResult.ok) return null;

  // getSigner() returns null if no account is selected
  const signer = manager.getSigner();
  if (!signer) return null;

  const tx = chain.assetHub.tx.System.remark({ remark: Binary.fromText('hello') });

  const result = await submitAndWatch(tx, signer, {
    onStatus: (status) => console.log(status.type),
  });

  if (!result.ok) {
    console.error(result.error.message); // TxError
    return null;
  }

  return result.value.block.number;
}
```

`onStatus` reports each transition (`signing`, `broadcasting`, `in-block`, and `finalized`), which you can drive a progress indicator from. `result.ok` means the transaction was included and its dispatch succeeded. Call `chain.destroy()` when your Product no longer needs the client.

## Handle Signing Errors

`submitAndWatch` returns a `Result` and resolves expected failures on the error channel rather than throwing. Two outcomes are worth handling explicitly:

- **The user declines**: The user dismissed the signing prompt on the Polkadot App, reported as `TxSigningRejectedError`. Return the user to a stable state and let them retry.
- **The submission times out**: The transaction did not reach the requested state in time, reported as `TxTimeoutError`. The default timeout is 300 seconds.

A dispatch that fails on chain arrives as `TxDispatchError`, and a transaction rejected before inclusion as `TxValidityError`. All of them extend `TxError`; see [Transactions](/apps/product-sdk/tx.md) for the full hierarchy.

!!! tip "Design for async latency"
    Signing is asynchronous because the Polkadot App runs on a separate device. Show a non-blocking pending state rather than freezing the interface, and make sure your retry path is idempotent in case the user attempts the same action twice. There is no push notification when the phone is waiting, so a stalled-looking action is often an [unanswered prompt](/apps/troubleshooting.md#the-app-seems-frozen-after-an-action).

## Test Without a Host

During development, use `connect('dev')` to load the standard Substrate dev accounts (Alice, Bob, Charlie, Dave, Eve, and Ferdie) without needing Polkadot Desktop. The signing API is identical — only the provider changes.

```typescript
async function devSignRaw() {
  const result = await manager.connect('dev');
  if (!result.ok) return null;

  const [alice] = result.value;
  if (!alice) return null;

  const selectResult = manager.selectAccount(alice.address);
  if (!selectResult.ok) return null;

  const signResult = await manager.signRaw(new TextEncoder().encode('test'));
  if (!signResult.ok) return null;

  return signResult.value; // Uint8Array - the raw signature
}
```

!!! warning "Four methods are host-only"
    `getProductAccount`, `getProductAccountAlias`, `createRingVRFProof`, and `getUserId` return `HostUnavailableError` when the active provider is `'dev'`. `getUserId` is easy to miss: [Show a Display Name](#show-a-display-name) uses it to read the user's earned name, so a Product that does that renders fine in a Host and fails under the dev provider.

## Limitations

- The package is ESM only; your Product's build pipeline must support ESM imports.
- `SignerManager.destroy()` is terminal. After calling it, all subsequent method calls return `DestroyedError`. Use `disconnect()` for a reversible reset.
- Account persistence across page reloads requires the host to expose `localStorage`. Outside a host container, persistence silently no-ops.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> **Store Data on Chain**

    ---

    Your Product can sign; next, use that signer to store content on the Bulletin Chain and fetch it from anywhere by CID.

    [:octicons-arrow-right-24: Store Data on Chain](/apps/build/store-data-on-chain.md)

-   <span class="badge external">External</span> **PAPI Docs**

    ---

    Reference for the typed PAPI signer interface that `@parity/product-sdk-signer` exposes.

    [:octicons-arrow-right-24: Visit Site](https://papi.how)

-   <span class="badge external">External</span> **Product SDK API Reference**

    ---

    The full `product-sdk` surface beyond this recipe: every package, class, and method.

    [:octicons-arrow-right-24: Visit Site](https://paritytech.github.io/product-sdk/)

</div>
