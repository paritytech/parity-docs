---
title: Write a Custom Precompile
description: Implement a pallet-revive precompile in Rust, define its Solidity interface, charge weight, register it in your runtime, and call it from Solidity.
short_description: Expose your own runtime logic to smart contracts through a precompile.
categories: Smart Contracts, Polkadot SDK
---

# Write a Custom Precompile

## Introduction

`pallet-revive` lets a runtime add its own precompiles next to the built-in ones. A custom precompile is Rust code that runs natively in the runtime when a contract calls a fixed address, so you can expose runtime functionality, such as a pallet's dispatchables or storage, to Solidity contracts through an ordinary Solidity interface. The XCM, ERC-20, and vesting precompiles described in [Precompiles](/smart-contracts/precompiles/index.md) are all built this way.

This guide covers:

- How `pallet-revive` matches addresses, decodes input, and reports errors for precompiles.
- How to implement the `Precompile` trait with a Solidity interface and weight charging.
- How to register the precompile in your runtime through `Config::Precompiles`.
- How to call the precompile from a Solidity contract.

The code in this guide is based on `pallet-revive` and the precompiles shipped in the Polkadot SDK at release `{{dependencies.repositories.polkadot_sdk.version}}`:

- **[`precompiles.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/precompiles.rs)**: Defines the `Precompile` trait, `AddressMatcher`, and `Error`.
- **[XCM precompile](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/polkadot/xcm/pallet-xcm/precompiles/src/lib.rs)**: The `pallet-xcm-precompiles` crate, a fixed-address precompile that dispatches into `pallet-xcm`.
- **[ERC-20 precompile](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/assets/precompiles/src/lib.rs)**: The `pallet-assets-precompiles` crate, a prefix-matched precompile that encodes the asset ID in the address.
- **[Vesting precompile](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/vesting/precompiles/src/lib.rs)**: The `pallet-vesting-precompiles` crate, a fixed-address precompile for `pallet-vesting`.

!!! note
    The precompile API is part of `pallet-revive` and changes between Polkadot SDK releases. Compare the code in this guide with the release your runtime uses.

## Prerequisites

Before you begin, make sure you have:

- A runtime that includes `pallet-revive`, as described in [Add pallet-revive to Your Runtime](/polkadot-sdk/customize-runtime/add-pallet-revive.md).
- Familiarity with FRAME pallets and weights, and with Solidity interfaces.

## How Precompiles Work

When a contract calls an address, `pallet-revive` first checks the built-in precompiles and then the precompiles registered in `Config::Precompiles`. If a precompile matches the address, the pallet decodes the call data against the precompile's Solidity interface and runs the precompile's Rust entry point instead of contract code.

### Address Matching

Each precompile declares where it lives with the [`AddressMatcher`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/precompiles.rs#L70-L102) constant `MATCHER`. The matcher takes a non-zero `u16` that is written big endian into bytes 16 and 17 of the 20-byte address. Bytes 18 and 19 are reserved for the built-in precompiles, so runtime precompiles cannot collide with them. The two variants are:

- **`AddressMatcher::Fixed(n)`**: The precompile answers at a single address, with all other bytes set to zero. For example, the XCM precompile uses `Fixed(10)`, which is `0x000A` in bytes 16 and 17 and gives the address `0x00000000000000000000000000000000000a0000`.
- **`AddressMatcher::Prefix(n)`**: The precompile answers at every address that has `n` in bytes 16 and 17 and zeros in bytes 4 to 15. The first four bytes are free, so the precompile can read data from them. The ERC-20 precompile uses this to encode a `u32` asset ID in the first four bytes of the address.

If two registered precompiles use the same matcher value, the runtime fails to compile.

### Input Decoding and Output Encoding

The `Interface` associated type is a Solidity ABI definition generated with the `sol!` macro from [Alloy](https://github.com/alloy-rs/core), which `pallet-revive` re-exports as `pallet_revive::precompiles::alloy`. Before calling your code, `pallet-revive` decodes and validates the call data as one of the interface's calls. If decoding fails, the call reverts with a Solidity `Panic` and your code does not run.

Your code receives the decoded call as a Rust enum and must return the ABI-encoded return value as `Vec<u8>`. `pallet-revive` does not check the returned bytes against the interface, so encode them with the generated helpers, such as `abi_encode_returns` on the call type.

### Errors

A precompile returns an [`Error`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/precompiles.rs#L114-L130), which maps to Solidity behavior as follows:

- **`Error::Revert`**: Reverts with an ABI-encoded `Error(string)` message, the same as `revert("message")` in Solidity. Callers can catch it with `try`/`catch Error(string memory)`.
- **`Error::Panic`**: Reverts with an ABI-encoded `Panic(uint256)` code from a `PanicKind`, such as `PanicKind::UnderOverflow`.
- **`Error::Error`**: Fails the call frame with an `ExecError` and returns no output. Use it for resource errors such as running out of weight. A `DispatchError` or a `pallet_revive::Error` converts into this variant.

### Weight Charging

Precompile code is not metered. The `Precompile` trait documentation states that a precompile has to charge appropriate weight itself, generally as its first line of code, before doing any work. The environment passed to the entry point provides the following methods:

- **`env.charge(weight)`**: Charges a `Weight` and returns a `ChargedAmount`, or an error if not enough weight is left.
- **`env.adjust_gas(charged, actual_weight)`**: Replaces an earlier charge with the weight that was actually used, which refunds the difference. The XCM precompile uses it after `execute`.
- **`env.frame_meter_mut().charge_weight_token(token)`**: Charges a `RuntimeCosts` token, such as `RuntimeCosts::DepositEvent` for emitting an event.

Use benchmarked weights for these charges. The ERC-20 precompile charges the `pallet-assets` weight of the operation it performs, and the vesting precompile ships its own `weights.rs` and a minimal pallet that exists only to run its benchmarks.

### Contract Info

The `HAS_CONTRACT_INFO` constant selects the entry point:

- **`false`**: `pallet-revive` creates no account or state for the precompile address and calls `call`. Most precompiles, including the XCM, ERC-20, and vesting precompiles, use this mode.
- **`true`**: `pallet-revive` creates an account and a contract info entry for the address on first call and calls `call_with_info`, whose environment adds APIs such as contract instantiation and the contract's key-value storage. Only `Fixed` matchers can use this mode, and each call incurs a storage read and write of the contract info.

## Implement a Precompile

The following example implements a precompile with two functions: `double`, which returns twice its input, and `remark`, which dispatches `frame_system::Pallet::remark_with_event` as the calling account. The example is a sketch that follows the patterns in the XCM and vesting precompiles; adapt the weights and checks to your use case.

### Create the Crate

Put the precompile in its own crate in your runtime workspace, as the Polkadot SDK does with `pallet-xcm-precompiles`, or in a module of your runtime crate. A separate crate needs `pallet-revive`, `frame-support`, and `frame-system`, with their `std` features propagated. The [`pallet-xcm-precompiles` `Cargo.toml`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/polkadot/xcm/pallet-xcm/precompiles/Cargo.toml) is a reference for the layout:

```toml title="example-precompile/Cargo.toml"
[package]
name = "example-precompile"
version = "0.1.0"
edition = "2021"

[dependencies]
frame-support = { workspace = true }
frame-system = { workspace = true }
pallet-revive = { workspace = true }

[features]
default = ["std"]
std = [
    "frame-support/std",
    "frame-system/std",
    "pallet-revive/std",
]
runtime-benchmarks = [
    "frame-support/runtime-benchmarks",
    "frame-system/runtime-benchmarks",
    "pallet-revive/runtime-benchmarks",
]
```

### Define the Solidity Interface

Write the interface in a `.sol` file in the crate. The same file serves as the Rust interface definition and as the import for contracts that call the precompile. The address constant matches the `Fixed(0x1000)` matcher used in the next step, which places `0x1000` in bytes 16 and 17:

```solidity title="example-precompile/src/IExample.sol"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @dev The address of the example precompile.
address constant EXAMPLE_PRECOMPILE_ADDRESS = address(0x10000000);

interface IExample {
    /// @notice Returns `value` multiplied by two.
    function double(uint64 value) external view returns (uint64);

    /// @notice Stores a remark on chain as the calling account.
    function remark(bytes calldata data) external;
}
```

Choose a matcher value that no other precompile in your runtime uses. For example, Asset Hub Westend uses `0x0120`, `0x0220`, `0x0320`, and `0x0420` for its asset and asset conversion precompiles, `0x0902` for vesting, and `10` for XCM.

### Implement the Precompile Trait

Implement [`Precompile`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/precompiles.rs#L157-L238) for a type that is generic over the runtime. The `sol!` macro reads the interface file relative to the crate root and generates the `IExample` module, including the `IExampleCalls` enum:

```rust title="example-precompile/src/lib.rs"
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::{marker::PhantomData, num::NonZero};
use frame_system::WeightInfo as _;
use pallet_revive::{
    precompiles::{
        alloy::{
            self,
            sol_types::{PanicKind, SolCall},
        },
        AddressMatcher, Error, Ext, Precompile,
    },
    Weight,
};

alloy::sol!("src/IExample.sol");
use IExample::IExampleCalls;

pub struct ExamplePrecompile<T>(PhantomData<T>);

impl<T: pallet_revive::Config> Precompile for ExamplePrecompile<T> {
    type T = T;
    type Interface = IExampleCalls;
    const MATCHER: AddressMatcher = AddressMatcher::Fixed(NonZero::new(0x1000).unwrap());
    const HAS_CONTRACT_INFO: bool = false;

    fn call(
        _address: &[u8; 20],
        input: &Self::Interface,
        env: &mut impl Ext<T = Self::T>,
    ) -> Result<Vec<u8>, Error> {
        match input {
            IExampleCalls::double(IExample::doubleCall { value }) => {
                // Charge first. Replace this constant with a benchmarked weight.
                env.charge(Weight::from_parts(10_000, 0))?;

                let doubled = value
                    .checked_mul(2)
                    .ok_or(Error::Panic(PanicKind::UnderOverflow))?;
                Ok(IExample::doubleCall::abi_encode_returns(&doubled))
            },
            IExampleCalls::remark(IExample::remarkCall { data }) => {
                env.charge(
                    <T as frame_system::Config>::SystemWeightInfo::remark_with_event(
                        data.len() as u32,
                    ),
                )?;

                // State changes are not allowed in a static call, and dispatching as the
                // caller must not be possible through a delegate call.
                if env.is_read_only() {
                    return Err(pallet_revive::Error::<T>::StateChangeDenied.into());
                }
                if env.is_delegate_call() {
                    return Err(pallet_revive::Error::<T>::PrecompileDelegateDenied.into());
                }

                let who = env
                    .caller()
                    .account_id()
                    .map_err(|_| Error::Revert("caller has no account id".into()))?
                    .clone();

                frame_system::Pallet::<T>::remark_with_event(
                    frame_system::RawOrigin::Signed(who).into(),
                    data.to_vec(),
                )
                .map_err(|e| Error::Revert(alloc::format!("remark failed: {:?}", e.error).into()))?;

                Ok(Vec::new())
            },
        }
    }
}
```

The example uses the following parts of the environment:

- **`env.caller()`**: The immediate caller, which is the calling contract when a contract calls the precompile. It returns an `Origin` that is either `Signed` with an account ID or `Root`. To get the account that signed the transaction, use `env.origin()`.
- **`env.is_read_only()`**: Returns `true` inside a static call, such as a call to a `view` function. The ERC-20, XCM, and vesting precompiles reject state-changing calls in this case with `StateChangeDenied`.
- **`env.is_delegate_call()`**: Returns `true` when a contract reaches the precompile through `delegatecall`. Precompiles that act on behalf of the caller reject this with `PrecompileDelegateDenied`.

Other methods on the environment, defined by the [`PrecompileExt`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/exec.rs#L323) trait (exported as `Ext`), include `to_account_id` to convert an `H160` address to an account ID, `balance_of`, `block_number`, `now`, and `deposit_event` to emit an Ethereum log. When you emit a log, charge `RuntimeCosts::DepositEvent` first, as the ERC-20 precompile does in its [`deposit_event`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/assets/precompiles/src/lib.rs#L249-L259) helper.

### Test the Precompile

`pallet-revive` exposes a `precompiles::run` module when the `runtime-benchmarks` feature is enabled. Its `precompile` function calls a precompile directly with an environment built by `CallSetup`, which you can use in benchmarks and tests. The [XCM precompile tests](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/polkadot/xcm/pallet-xcm/precompiles/src) and the [`pallet-revive` test precompiles](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/tests/precompiles.rs) show complete setups, including precompiles that revert, panic, and fail with an `ExecError`.

## Register the Precompile

Add the crate as a dependency of your runtime and propagate its features:

```toml title="runtime/Cargo.toml"
[dependencies]
example-precompile = { path = "../example-precompile", default-features = false }

[features]
std = [
    # ... other features
    "example-precompile/std",
]
runtime-benchmarks = [
    # ... other features
    "example-precompile/runtime-benchmarks",
]
```

Then add the type to the `Precompiles` tuple in your `pallet_revive::Config` implementation. The [Asset Hub Westend runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs#L1398-L1409) registers its precompiles the same way:

```rust title="runtime/src/lib.rs"
impl pallet_revive::Config for Runtime {
    // ... other configuration
    type Precompiles = (
        // ... other precompiles
        example_precompile::ExamplePrecompile<Self>,
    );
}
```

`Config::Precompiles` accepts a tuple of up to 20 types that implement `Precompile`. Built-in precompiles always take priority, and a runtime precompile that reuses a matcher value fails at compile time. Changing `Precompiles` changes the runtime, so a live chain needs a runtime upgrade to add a precompile.

## Call the Precompile from Solidity

Contracts call the precompile through its interface at the fixed address, like any other contract. Import the interface file and wrap the address:

```solidity title="UseExample.sol"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./IExample.sol";

contract UseExample {
    IExample constant EXAMPLE = IExample(EXAMPLE_PRECOMPILE_ADDRESS);

    function quadruple(uint64 value) external view returns (uint64) {
        return EXAMPLE.double(EXAMPLE.double(value));
    }

    function tryRemark(bytes calldata data) external returns (bool ok, string memory reason) {
        try EXAMPLE.remark(data) {
            return (true, "");
        } catch Error(string memory message) {
            return (false, message);
        }
    }
}
```

When this contract calls `remark`, `env.caller()` in the precompile is the contract's account, so the remark is stored as the contract. A wallet or script can also call the precompile directly through the [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter.md) at `{{ networks.local.eth_rpc_url }}` by sending a transaction or `eth_call` to `0x0000000000000000000000000000000010000000` with the ABI-encoded call data.

`pallet-revive` reports a small placeholder bytecode for precompile addresses that implement `Precompile`, so `EXTCODESIZE` checks in Solidity high-level calls succeed.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge learn">Learn</span> __Precompiles__

    ---

    See the built-in and runtime-registered precompiles and their addresses.

    [:octicons-arrow-right-24: Learn More](/smart-contracts/precompiles/index.md)

-   <span class="badge guide">Guide</span> __Add pallet-revive to Your Runtime__

    ---

    Configure `pallet-revive`, including its `Precompiles` type, in a Polkadot SDK runtime.

    [:octicons-arrow-right-24: Get Started](/polkadot-sdk/customize-runtime/add-pallet-revive.md)

</div>
