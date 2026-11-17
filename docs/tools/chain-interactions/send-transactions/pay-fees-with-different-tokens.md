---
title: Pay Transaction Fees with Different Tokens
description: Learn how the ChargeAssetTxPayment transaction extension lets accounts pay transaction fees in an asset other than the native token, and how to use it with Subxt.
short_description: Pay transaction fees in non-native assets with the ChargeAssetTxPayment extension.
categories: Chain Interactions, Tooling
page_badges:
  tutorial_badge: Intermediate
---

# Send a Transaction While Paying the Fee with a Different Token

## Introduction

Polkadot SDK runtimes can let accounts pay transaction fees in an asset other than the chain's native token. This capability comes from the `ChargeAssetTxPayment` transaction extension, which adds an optional asset ID to each transaction. When the asset ID is set, the runtime charges the fee in that asset. When it is not set, the fee falls back to the native token.

This tutorial explains how the extension works and uses [Subxt](/tools/subxt/), Parity's Rust library for Polkadot SDK-based chains, to send a native token transfer while paying the fee in another asset. The same pattern applies to any runtime that includes the extension.

Third-party client libraries also support this extension, for example [Polkadot API (PAPI)](https://developers.polkadot.com/reference/tools/papi/) and [Polkadot.js API](https://developers.polkadot.com/reference/tools/polkadot-js-api/). For a walkthrough on Polkadot Hub, see [Pay Transaction Fees with Different Tokens](https://developers.polkadot.com/chain-interactions/send-transactions/pay-fees-with-different-tokens/) on developers.polkadot.com.

## How the Extension Works

The Polkadot SDK provides two pallets that implement a `ChargeAssetTxPayment` extension. Both wrap the [Transaction Payment pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_transaction_payment/index.html) and replace its transaction extension in the runtime:

- **[Asset Conversion Transaction Payment pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_asset_conversion_tx_payment/index.html)**: Converts the fee calculated in the native asset into the amount required of the chosen asset by swapping through the Asset Conversion pallet. The asset needs a liquidity pool with the native token.
- **[Asset Transaction Payment pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_asset_tx_payment/index.html)**: Converts the fee calculated by the Transaction Payment pallet into the chosen asset with a fungibles adapter.

To support this feature in your own runtime, include one of these pallets alongside the Transaction Payment pallet in `construct_runtime` and use its `ChargeAssetTxPayment` extension in place of the Transaction Payment extension.

On the client side, you set the asset ID in the transaction's extension parameters. The asset ID type depends on the runtime. The example in this tutorial targets a runtime whose asset ID is an XCM `Location`.

## Prerequisites

This tutorial needs a running chain that includes the extension and an account that holds a fee asset with a liquidity pool. The steps below use [Chopsticks](https://developers.polkadot.com/reference/tools/chopsticks/), a third-party tool, to fork a chain that meets these requirements locally. If you run your own runtime with the extension, point the example at your node instead and adjust the asset ID, the Assets pallet index, and the funded account.

- [Rust](https://rustup.rs/) toolchain (latest stable) and Cargo.
- [Chopsticks](https://developers.polkadot.com/reference/tools/chopsticks/), or a node of your own runtime that includes the extension.

## Start a Local Chain

Fork a chain that includes the extension with Chopsticks:

```bash
chopsticks -c polkadot-asset-hub
```

This command starts a local fork at `ws://localhost:8000` using the [`polkadot-asset-hub.yml`](https://github.com/AcalaNetwork/chopsticks/blob/master/configs/polkadot-asset-hub.yml) configuration, which gives Alice's account a balance of the asset that the example uses for fees. If you use a different chain or configuration, make sure the account you sign with holds the fee asset.

## Set Up Your Project

1. Create a new Rust project:

    ```bash
    cargo new subxt-fee-payment-example && cd subxt-fee-payment-example
    ```

2. Install the Subxt CLI to download chain metadata:

    ```bash
    cargo install subxt-cli@{{ dependencies.crates.subxt_cli.version }}
    ```

3. Download the metadata from the running chain:

    ```bash
    mkdir metadata && \
    subxt metadata --url ws://localhost:8000 -o metadata/runtime.scale
    ```

    !!! note
        Ensure your chain is running at `ws://localhost:8000` before downloading the metadata. If you use your own node, pass its endpoint instead, for example `{{ networks.local.ws_url }}`.

4. Update `Cargo.toml` with the required dependencies:

    ```toml title="Cargo.toml"
    --8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/Cargo.toml"
    ```

5. Create the source file:

    ```bash
    mkdir -p src/bin && touch src/bin/fee_payment_transaction.rs
    ```

## Implementation

The following sections cover how to set up imports and constants, create a transaction signer, connect to the chain, and send a native token transfer while paying fees in another asset.

### Import Dependencies and Define Constants

First, generate types from the chain metadata using the `#[subxt::subxt()]` macro. The `derive_for_type` attribute ensures the `Location` type implements the traits needed for encoding. Then define a custom `AssetPaymentConfig` where `type AssetId = Location`, which lets you pass an XCM location as the fee payment asset. Finally, define the endpoint, transfer amount, Assets pallet index, and fee asset ID:

```rust title="src/bin/fee_payment_transaction.rs"
--8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/src/bin/fee_payment_transaction.rs:1:45"
```

### Create a Signer and Connect

Create a signer using Alice's development account and connect to the chain. The `OnlineClient` is parameterized with `AssetPaymentConfig` instead of the default `PolkadotConfig`:

```rust title="src/bin/fee_payment_transaction.rs"
--8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/src/bin/fee_payment_transaction.rs:47:58"
```

### Create the Transaction

Create a standard transfer that sends native tokens to Bob's development account while keeping Alice's account alive:

```rust title="src/bin/fee_payment_transaction.rs"
--8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/src/bin/fee_payment_transaction.rs:60:64"
```

### Sign and Submit with Alternative Fee Payment

The key step is setting the asset ID in the `ChargeAssetTxPayment` extension parameters. In this runtime, the fee asset is identified by its XCM location relative to the chain: `PalletInstance` holds the index of the Assets pallet and `GeneralIndex` holds the asset's ID in that pallet.

Use `DefaultExtrinsicParamsBuilder` with `tip_of(0, asset_location)` to specify the fee asset. The first argument is the tip amount (0), and the second is the XCM `Location`. Instead of calling `sign_and_submit_then_watch_default`, pass the custom `tx_params` to `sign_and_submit_then_watch`:

```rust title="src/bin/fee_payment_transaction.rs"
--8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/src/bin/fee_payment_transaction.rs:66:102"
```

### Full Code

??? code "Complete Code"

    ```rust title="src/bin/fee_payment_transaction.rs"
    --8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/src/bin/fee_payment_transaction.rs"
    ```

### Run the Script

```bash
cargo run --bin fee_payment_transaction
```

### Expected Output

When you run the script successfully, you should see output similar to the following. The exact list of events depends on the runtime and on the state of the accounts involved.

--8<-- "code/chain-interactions/send-transactions/pay-fees-with-different-tokens/subxt/fee-payment-transaction-rs.html"

The key events to look for are:

- **`AssetTxPayment`**: Confirms the fees were paid using the alternative asset.
- **`AssetConversion`**: The alternative asset was swapped to cover native fees.
- **`Balances`**: The transfer was executed and deposit and withdrawal events were emitted.
- **`System`**: The transaction completed successfully.

## Conclusion

The `ChargeAssetTxPayment` extension lets users and applications pay fees in the assets they hold, on any runtime that includes it. On the client side, the only change is setting the fee asset in the transaction's extension parameters, using the asset ID type that the runtime expects.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Send Transactions with SDKs__

    ---

    Learn how to build, sign, and submit transactions with Subxt.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/with-sdks/)

-   <span class="badge guide">Guide</span> __Calculate Transaction Fees__

    ---

    Understand how to estimate transaction fees before submitting.

    [:octicons-arrow-right-24: Get Started](/tools/chain-interactions/send-transactions/calculate-transaction-fees/)

</div>
