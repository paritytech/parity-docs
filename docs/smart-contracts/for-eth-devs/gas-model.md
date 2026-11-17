---
title: Gas Model
description: Learn how pallet-revive meters contract execution, maps weight to Ethereum gas, estimates gas, and charges fees.
short_description: Gas vs. weight, proof size, storage deposits, and fees.
categories: Smart Contracts, Protocol Internals
---

# Gas Model

## Introduction

pallet-revive implements a gas model that bridges Ethereum's familiar gas concept with the multi-dimensional resource metering of the Polkadot SDK. This page explains how gas, weight, and fees work on a chain running pallet-revive and what you need to know when building smart contracts.

## Understanding Resources in pallet-revive

Unlike Ethereum, which uses a single gas value to measure everything, pallet-revive tracks three separate resources:

- **`ref_time`**: Measures computational time. It is the closest equivalent to traditional gas and represents the amount of CPU time your contract execution consumes.
- **`proof_size`**: Measures the amount of state data that validators need to verify. When your contract reads from storage or makes state queries, this metric tracks the size of the proofs needed to validate those operations.
- **`storage_deposit`**: Is a native balance that gets temporarily locked when your contract creates new storage entries. This prevents state bloat by requiring contracts to pay for the storage they use. The deposit is returned when the storage is freed.

For Ethereum wallet compatibility, these three dimensions are mapped into a single gas value that wallets can understand and display to users.

These resources can be limited at both the transaction and the contract level, similar to Ethereum's gas limits. Gas costs are based on benchmarks of actual execution rather than Ethereum's fixed values, which makes computation cheaper relative to I/O operations. Do not hardcode gas values in your contracts or scripts; estimate them instead. For how the gas-related opcodes and cross-contract call limits behave on PVM, see [EVM vs PVM](/smart-contracts/for-eth-devs/evm-vs-pvm/#gas-model).

## Gas vs Weight

When you interact with a chain running pallet-revive through an Ethereum wallet, you see familiar gas values. Internally, the runtime works with `weight`, a two-dimensional metric that combines `ref_time` and `proof_size`.

The system continuously translates between these representations: converting `weight` to gas when estimating costs, and converting gas back to `weight` when executing transactions.

## How Gas Estimation Works

When your wallet requests a gas estimate ([`eth_estimateGas`](/smart-contracts/for-eth-devs/json-rpc-apis/#eth_estimategas)), pallet-revive performs a dry-run of your transaction. This test execution discovers:

- How much computational time the contract will consume (`ref_time`).
- How much state data needs to be verified (`proof_size`).
- Whether any storage deposits are required (`storage_deposit`).

The system then calculates a gas estimate that covers all these costs, including:

- Base transaction overhead (intrinsic costs like signature verification, nonce/account checks, and dispatch setup).
- Transaction length fees (charges for the transaction size in bytes).
- The actual contract execution costs.
- Any storage deposits.

The gas estimate also includes a small safety buffer to account for slight differences between the test run and actual execution.

## Dynamic Gas Pricing

pallet-revive uses dynamic pricing through a "fee multiplier" that adjusts based on network congestion:

- When blocks are full, the multiplier increases, making transactions more expensive.
- When blocks are empty, the multiplier decreases, making transactions cheaper.
- The multiplier updates after every block based on utilization.

This creates a market-based pricing mechanism similar to Ethereum's base fee, helping to manage network resources efficiently.

The gas price returned during estimation is the current fee multiplier value.

!!! warning "Important for Users"
    Because the fee multiplier can change between when you estimate gas and when your transaction executes, you can add a safety buffer (10-20%) to both your gas limit and gas price. This ensures your transaction will execute successfully even if network conditions change slightly.

## Transaction Execution Flow

The following diagram illustrates the complete lifecycle of a transaction from submission to settlement:

```mermaid
graph TD
    U[User/Wallet] --> M[Transaction pool]
    M --> P[Pre-dispatch convert gas to weight and create hold]
    P --> C{Sufficient funds}
    C -->|No| R[Rejected]
    C -->|Yes| X[Execute contract within limits]
    X --> S[Settle fee from actual weight and length; refund]
    S --> B[Included in block]
```

The transaction execution flow is as follows:

- **Pool and pre-dispatch**: The transaction enters the pool, `gas` is mapped to `weight`, and a temporary hold is created for the maximum fee exposure. Weight is a two-dimensional tuple (`ref_time`, `proof_size`). Each dimension is tracked independently. The [`WeightToFee`](https://docs.rs/pallet-transaction-payment/latest/pallet_transaction_payment/pallet/trait.Config.html#associatedtype.WeightToFee) conversion takes the maximum of the two dimensions (after applying their respective coefficients) to determine the fee.
- **Funds check**: If the hold is insufficient, the transaction is rejected before any execution.
- **Execution**: If sufficient, the contract runs within the derived weight limits; a `storage_deposit` may be reserved when new storage is created.
- **Settlement**: Fees are charged from the actual `weight` used plus the length fee; any unused hold is refunded.
- **Inclusion**: After settlement, the transaction is included in the block.

## Fee Components

The total fee for a contract transaction combines the following parts:

- **Execution fee**: Charged from the actual `weight` consumed, converted to the native currency through the [`WeightToFee`](https://docs.rs/pallet-transaction-payment/latest/pallet_transaction_payment/pallet/trait.Config.html#associatedtype.WeightToFee) conversion and the current fee multiplier.
- **Length fee**: Charged for the size of the transaction in bytes.
- **Storage deposit**: Held, not spent, when a transaction creates new storage, including the code storage for a contract deployment. The deposit is refunded when the storage is freed.

Because the storage deposit is part of the estimate but is refundable, gas estimates for deployments can be much higher than the fee you end up paying. See [Gas Estimation vs Actual Consumption](/smart-contracts/for-eth-devs/contract-deployment/#gas-estimation-vs-actual-consumption).

## Blocks and Transactions

Blocks on a chain running pallet-revive are standard Polkadot SDK blocks. pallet-revive adds an Ethereum view of each block, represented by its [`Block`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/evm/struct.Block.html) type, so Ethereum JSON-RPC methods such as `eth_getBlockByNumber` can return Ethereum-shaped block data. Ethereum transactions reach pallet-revive through the `eth-rpc` adapter. For the supported transaction formats, see [Supported Transaction Types](/smart-contracts/for-eth-devs/json-rpc-apis/#supported-transaction-types).

## Conclusion

The pallet-revive gas model is compatible with Ethereum tooling while keeping the multi-dimensional resource metering of the Polkadot SDK. You can keep using Ethereum wallets and tools, as long as you estimate gas rather than hardcode it and allow for storage deposits.
