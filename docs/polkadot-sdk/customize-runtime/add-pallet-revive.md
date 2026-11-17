---
title: Add pallet-revive to Your Runtime
description: Add pallet-revive to a Polkadot SDK runtime step by step, covering dependencies, fees, transaction extensions, Config, chain ID, runtime APIs, and precompiles.
short_description: Integrate pallet-revive and its runtime APIs into a runtime.
categories: Polkadot SDK, Smart Contracts
---

# Add pallet-revive to Your Runtime

## Introduction

[`pallet-revive`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/index.html) adds smart contract support to a Polkadot SDK runtime, and the [Ethereum JSON-RPC adapter](/smart-contracts/eth-rpc-adapter/) (`eth-rpc`) lets Ethereum tools talk to a chain that runs it. For an overview of what the pallet provides, see [Add Smart Contract Functionality](/polkadot-sdk/customize-runtime/add-smart-contract-functionality/). This guide walks through the changes you make to your runtime:

- Add the `pallet-revive` dependency.
- Configure transaction fees in the way the pallet requires.
- Define the transaction extension and extrinsic types that let the runtime accept Ethereum transactions.
- Implement the pallet's `Config` trait, including the chain ID and precompiles.
- Add the pallet to the runtime and implement the runtime APIs that `eth-rpc` calls.

The code in this guide comes from runtimes in the Polkadot SDK at release `{{dependencies.repositories.polkadot_sdk.version}}`:

- **[Revive dev runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs)**: A minimal solochain runtime used by the Revive dev node.
- **[Kitchensink runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs)**: The Substrate node runtime, which configures every `Config` item explicitly.
- **[Asset Hub Westend runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs)**: A parachain runtime with asset, XCM, and asset conversion precompiles.

!!! note
    The `pallet-revive` `Config` trait changes between Polkadot SDK releases. For example, the Polkadot SDK `master` branch adds a `MaxOutsideFrameLogs` item that `{{dependencies.repositories.polkadot_sdk.version}}` does not have. Compare the code in this guide with the reference runtimes of the release your runtime uses.

## Prerequisites

Before you begin, make sure you have:

- [Rust and the Polkadot SDK dependencies](/polkadot-sdk/install-polkadot-sdk/) installed.
- A FRAME runtime that already includes `frame_system`, `pallet_balances`, `pallet_timestamp`, and `pallet_transaction_payment`. `pallet-revive` uses these pallets for accounts, balances, timestamps, and fees.
- Familiarity with adding a pallet to a runtime, as described in [Add an Existing Pallet to the Runtime](/polkadot-sdk/customize-runtime/add-existing-pallets/).

## Add the Dependency

How you add the dependency depends on how your runtime imports Polkadot SDK crates.

=== "polkadot-sdk umbrella crate"

    If your runtime depends on the `polkadot-sdk` umbrella crate, add `pallet-revive` to its features in `runtime/Cargo.toml`. The [Revive dev runtime's `Cargo.toml`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/Cargo.toml) uses this approach:

    ```toml title="runtime/Cargo.toml"
    polkadot-sdk = { workspace = true, features = [
        "pallet-balances",
        "pallet-revive",
        "pallet-sudo",
        "pallet-timestamp",
        "pallet-transaction-payment",
        # ... other features
    ] }
    ```

    The `polkadot-sdk/std` entry in your `std` feature list also enables the standard library features of `pallet-revive`.

=== "Individual crates"

    If your runtime depends on individual crates, add `pallet-revive` and propagate its features. The [Asset Hub Westend `Cargo.toml`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/Cargo.toml) uses this approach:

    ```toml title="runtime/Cargo.toml"
    [dependencies]
    pallet-revive = { workspace = true }

    [features]
    runtime-benchmarks = [
        # ... other features
        "pallet-revive/runtime-benchmarks",
    ]
    try-runtime = [
        # ... other features
        "pallet-revive/try-runtime",
    ]
    std = [
        # ... other features
        "pallet-revive/std",
    ]
    ```

## Configure Transaction Fees

`pallet-revive` maps Ethereum gas to Polkadot SDK weight and fees, and this mapping depends on the fee structure. The pallet supports only one [`WeightToFee`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/evm/fees/struct.BlockRatioFee.html) implementation for `pallet_transaction_payment`: `BlockRatioFee`, which enforces a ratio between `ref_time` and `proof_size` fees that matches their distribution in the block limits.

Set `WeightToFee` in your `pallet_transaction_payment::Config` implementation. The [kitchensink runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs#L609-L613) uses:

```rust title="runtime/src/lib.rs"
impl pallet_transaction_payment::Config for Runtime {
    // ... other configuration
    type WeightToFee = pallet_revive::evm::fees::BlockRatioFee<1, 1, Self, Balance>;
}
```

The two numeric parameters define the rational number that maps `ref_time` to fees, and neither can be zero.

## Accept Ethereum Transactions

The `eth-rpc` adapter submits Ethereum transactions to the chain through the `eth_transact` call of `pallet-revive`. The runtime needs three types to accept them: a transaction extension pipeline that includes `SetOrigin`, an `EthExtra` implementation, and the `pallet-revive` extrinsic type.

1. Add `pallet_revive::evm::tx_extension::SetOrigin` to your transaction extension tuple, after all extensions that rely on a signed origin. The [Revive dev runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs#L163-L188) uses the following pipeline:

    ```rust title="runtime/src/lib.rs"
    type TxExtension = (
        frame_system::CheckNonZeroSender<Runtime>,
        frame_system::CheckSpecVersion<Runtime>,
        frame_system::CheckTxVersion<Runtime>,
        frame_system::CheckGenesis<Runtime>,
        frame_system::CheckEra<Runtime>,
        frame_system::CheckNonce<Runtime>,
        frame_system::CheckWeight<Runtime>,
        pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
        // Needs to be done after all extensions that rely on a signed origin.
        pallet_revive::evm::tx_extension::SetOrigin<Runtime>,
        // Must be last to catch the refund in previous transaction extensions.
        frame_system::WeightReclaim<Runtime>,
    );
    ```

2. Implement the [`EthExtra`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/evm/runtime/trait.EthExtra.html) trait. Its `get_eth_extension` function builds the extension values that the runtime applies to an Ethereum transaction, using the nonce and tip taken from that transaction. Return one value per entry in your `TxExtension` tuple, in the same order, and construct `SetOrigin` with `new_from_eth_transaction()`. The Revive dev runtime's [implementation](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs#L190-L213) is:

    ```rust title="runtime/src/lib.rs"
    use pallet_revive::evm::runtime::EthExtra;

    /// Default extensions applied to Ethereum transactions.
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct EthExtraImpl;

    impl EthExtra for EthExtraImpl {
        type Config = Runtime;
        type ExtensionV0 = TxExtension;
        type ExtensionOtherVersions = sp_runtime::traits::InvalidVersion;

        fn get_eth_extension(nonce: u32, tip: Balance) -> Self::ExtensionV0 {
            (
                frame_system::CheckNonZeroSender::<Runtime>::new(),
                frame_system::CheckSpecVersion::<Runtime>::new(),
                frame_system::CheckTxVersion::<Runtime>::new(),
                frame_system::CheckGenesis::<Runtime>::new(),
                frame_system::CheckMortality::from(sp_runtime::generic::Era::Immortal),
                frame_system::CheckNonce::<Runtime>::from(nonce),
                frame_system::CheckWeight::<Runtime>::new(),
                pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
                pallet_revive::evm::tx_extension::SetOrigin::<Runtime>::new_from_eth_transaction(),
                frame_system::WeightReclaim::<Runtime>::new(),
            )
        }
    }
    ```

    The extension must include `frame_system::CheckNonce` so that the runtime checks the nonce of the Ethereum transaction.

3. Replace your runtime's `UncheckedExtrinsic` type with the `pallet-revive` [`UncheckedExtrinsic`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/evm/runtime/struct.UncheckedExtrinsic.html), which wraps the generic extrinsic type and adds support for `eth_transact`:

    ```rust title="runtime/src/lib.rs"
    pub type UncheckedExtrinsic =
        pallet_revive::evm::runtime::UncheckedExtrinsic<Address, Signature, EthExtraImpl>;
    ```

If your runtime already uses a wrapping extension, such as `cumulus_pallet_weight_reclaim::StorageWeightReclaim` in a parachain, keep it and add `SetOrigin` inside the wrapped tuple. The [Asset Hub Westend runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs#L1762-L1818) shows this pattern.

## Implement the Config Trait

Implement [`pallet_revive::Config`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/pallet/trait.Config.html) for your runtime. The following implementation combines the kitchensink runtime's [deposit parameters](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs#L1504-L1512) and [`Config` implementation](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs#L1556-L1590), with its precompiles removed. `deposit` is a helper in the kitchensink runtime that prices storage items and bytes in the native token; use your runtime's equivalent.

```rust title="runtime/src/lib.rs"
parameter_types! {
    pub const DepositPerItem: Balance = deposit(1, 0);
    pub const DepositPerChildTrieItem: Balance = deposit(1, 0) / 100;
    pub const DepositPerByte: Balance = deposit(0, 1);
    pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(30);
    pub const MaxEthExtrinsicWeight: FixedU128 = FixedU128::from_rational(9, 10);
}

impl pallet_revive::Config for Runtime {
    type Time = Timestamp;
    type Balance = Balance;
    type Currency = Balances;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type RuntimeOrigin = RuntimeOrigin;
    type DepositPerItem = DepositPerItem;
    type DepositPerChildTrieItem = DepositPerChildTrieItem;
    type DepositPerByte = DepositPerByte;
    type WeightInfo = pallet_revive::weights::SubstrateWeight<Self>;
    type Precompiles = ();
    type AddressMapper = pallet_revive::AccountId32Mapper<Self>;
    type RuntimeMemory = ConstU32<{ 128 * 1024 * 1024 }>;
    type PVFMemory = ConstU32<{ 512 * 1024 * 1024 }>;
    type UploadOrigin = EnsureSigned<Self::AccountId>;
    type InstantiateOrigin = EnsureSigned<Self::AccountId>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
    type ChainId = ConstU64<420_420_420>;
    type NativeToEthRatio = ConstU32<1_000_000>; // 10^(18 - 12) Eth is 10^18, Native is 10^12.
    type FindAuthor = <Runtime as pallet_authorship::Config>::FindAuthor;
    type AllowEVMBytecode = ConstBool<true>;
    type FeeInfo = pallet_revive::evm::fees::Info<Address, Signature, EthExtraImpl>;
    type MaxEthExtrinsicWeight = MaxEthExtrinsicWeight;
    type DebugEnabled = ConstBool<false>;
    type AutoMap = ConstBool<false>;
    type GasScale = ConstU32<1000>;
    type OnBurn = ();
    type Deposit = ();
}
```

The `Config` items fall into the following groups. For the full description of each item, see the [`Config` trait source](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/src/lib.rs#L176-L400).

- **Runtime types**: `RuntimeEvent`, `RuntimeCall`, `RuntimeOrigin`, and `RuntimeHoldReason` link the pallet to the runtime's aggregated types.
- **Currency and time**: `Currency` is the fungible token that pays fees and holds contract balances, `Balance` is its balance type, and `Time` supplies timestamps to contracts.
- **Storage deposits**: `DepositPerByte`, `DepositPerItem`, and `DepositPerChildTrieItem` set what callers pay for storage. `CodeHashLockupDepositPercent` sets the share of the code deposit held when a contract is instantiated from a code hash. `Deposit` selects the payment backend for deposits; `()` always uses the native currency.
- **Ethereum compatibility**: `ChainId` is the chain ID reported to Ethereum tools. `NativeToEthRatio` is the ratio between the decimal representation of the native token and ETH's 18 decimals. `FeeInfo` must be `pallet_revive::evm::fees::Info` in a production runtime. `AddressMapper` maps between native accounts and Ethereum addresses, and must be `AccountId32Mapper` or `H160Mapper`.
- **Gas and weight**: `GasScale` scales the gas price and gas estimates, so that gas costs are closer to standard EVM values, and cannot be `0`. `MaxEthExtrinsicWeight` caps the weight of `eth_transact` extrinsics as a fraction of the maximum extrinsic weight.
- **Code and origins**: `AllowEVMBytecode` allows EVM bytecode to be uploaded and instantiated, in addition to PVM bytecode. `UploadOrigin` and `InstantiateOrigin` control who can upload and instantiate contracts; `EnsureSigned` allows anyone.
- **Memory**: `RuntimeMemory` and `PVFMemory` are the memory available to the runtime and to the PoV validation function. The pallet's integrity test uses them to check that the runtime has enough memory.
- **Other**: `Precompiles` registers additional precompiles, `FindAuthor` finds the block author, `WeightInfo` provides benchmarked weights, `OnBurn` receives native currency burned through gas rounding, `DebugEnabled` enables debug settings such as unlimited contract size, and `AutoMap` enables automatic account mapping.

The Asset Hub Westend runtime differs from this configuration in several items. For example, it registers asset, XCM, asset conversion, and vesting precompiles, sets `AutoMap` to `true`, and uses a different chain ID. See its [`Config` implementation](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs#L1379-L1435).

!!! note
    The Revive dev runtime uses `#[derive_impl(pallet_revive::config_preludes::TestDefaultConfig)]` to fill in most items. That prelude provides default configurations for testing environments, for example, `WeightInfo = ()` and `FeeInfo = ()`. Set every item explicitly in a production runtime, as the kitchensink and Asset Hub Westend runtimes do.

### Set the Chain ID

`ChainId` is the [EIP-155](https://eips.ethereum.org/EIPS/eip-155) chain ID, which identifies your chain to Ethereum wallets and tools and prevents transactions signed for one chain from being replayed on another. The `eth-rpc` adapter reads this value from the runtime and returns it for `eth_chainId`.

Choose a value that no other network uses. The reference runtimes use `420_420_420` (kitchensink and Revive dev runtime) and `420_420_421` (Asset Hub Westend), so do not reuse those values for your own network.

### Enable Automatic Account Mapping

Native `AccountId32` accounts must be mapped to Ethereum addresses before they can interact with contracts. By default, accounts map themselves with the `map_account` call. If you set `AutoMap` to `true`, the pallet maps accounts automatically when they are created and unmaps them when they are removed, and disables the `map_account` and `unmap_account` calls.

`AutoMap` requires you to set `pallet_revive::AutoMapper` as the account creation and removal hooks in `frame_system::Config`. The [Revive dev runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs#L305-L316) does this:

```rust title="runtime/src/lib.rs"
impl frame_system::Config for Runtime {
    // ... other configuration
    type OnNewAccount = pallet_revive::AutoMapper<Runtime>;
    type OnKilledAccount = pallet_revive::AutoMapper<Runtime>;
}
```

## Add the Pallet to the Runtime

Add `pallet-revive` to your runtime under the name `Revive`. The runtime API macro in the next section takes this name as a parameter, and the `eth-rpc` adapter's client looks up the pallet's constants by it.

=== "#[runtime] macro"

    The [Revive dev runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs#L265-L267) declares the pallet as follows. Use a pallet index that is unused in your runtime.

    ```rust title="runtime/src/lib.rs"
    #[runtime::pallet_index(5)]
    pub type Revive = pallet_revive::Pallet<Runtime>;
    ```

=== "construct_runtime macro"

    The [Asset Hub Westend runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/assets/asset-hub-westend/src/lib.rs#L1704) declares the pallet as follows. Use a pallet index that is unused in your runtime.

    ```rust title="runtime/src/lib.rs"
    construct_runtime!(
        pub enum Runtime {
            // ... other pallets
            Revive: pallet_revive = 60,
        }
    );
    ```

## Implement the Runtime APIs

The `eth-rpc` adapter calls the [`ReviveApi`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/trait.ReviveApi.html) runtime API for balances, nonces, calls, gas estimation, gas price, block information, and tracing. `pallet-revive` provides the [`impl_runtime_apis_plus_revive_traits!`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/macro.impl_runtime_apis_plus_revive_traits.html) macro, which wraps `impl_runtime_apis!`, implements `ReviveApi`, and implements the `SetWeightLimit` trait for the runtime call type.

Replace `impl_runtime_apis!` with this macro and pass four parameters before your existing runtime API implementations:

- **`Runtime`**: The runtime type.
- **`Revive`**: The name under which the pallet is declared in the runtime.
- **`Executive`**: The runtime's `Executive` type.
- **`EthExtraImpl`**: Your `EthExtra` implementation.

The [Revive dev runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs#L376-L497) uses it as follows:

```rust title="runtime/src/apis.rs"
pallet_revive::impl_runtime_apis_plus_revive_traits!(
    Runtime,
    Revive,
    Executive,
    EthExtraImpl,

    impl apis::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }
        // ... other functions
    }

    // ... other runtime API implementations
);
```

Keep all the runtime API implementations you already have inside the macro call; do not implement `ReviveApi` yourself.

## Register Precompiles

`pallet-revive` includes the standard Ethereum precompiles and its own system precompiles. To expose more runtime functionality to contracts, pass a tuple of types that implement the [`Precompile`](https://paritytech.github.io/polkadot-sdk/master/pallet_revive/precompiles/trait.Precompile.html) trait to `Config::Precompiles`. The type is usually provided by another pallet.

For example, the [kitchensink runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/bin/node/runtime/src/lib.rs#L1567-L1571) registers ERC-20 precompiles for two instances of the Assets pallet and a vesting precompile:

```rust title="runtime/src/lib.rs"
use pallet_assets_precompiles::{InlineIdConfig, ERC20};
use pallet_vesting_precompiles::Vesting as VestingPrecompile;

impl pallet_revive::Config for Runtime {
    // ... other configuration
    type Precompiles = (
        ERC20<Self, InlineIdConfig<0x1>, Instance1>,
        ERC20<Self, InlineIdConfig<0x2>, Instance2>,
        VestingPrecompile<Self>,
    );
}
```

Each precompile crate is a separate dependency of your runtime, and some, such as the vesting precompile, also require their own pallet in the runtime. The Asset Hub Westend runtime additionally registers `pallet_xcm_precompiles::XcmPrecompile` and the asset conversion precompile. For the precompiles that contracts can call, see [Precompiles](/smart-contracts/precompiles/).

## Configure Genesis

The `pallet-revive` genesis configuration has a `mapped_accounts` field that lists native accounts to map at genesis, so they can interact with contracts from the first block. The Revive dev runtime's [development preset](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/substrate/frame/revive/dev-node/runtime/src/lib.rs#L117-L123) maps its endowed accounts:

```rust title="runtime/src/genesis_config_presets.rs"
revive: ReviveConfig {
    mapped_accounts: endowed_accounts
        .iter()
        .filter(|x| !<Runtime as pallet_revive::Config>::AddressMapper::is_mapped(x))
        .cloned()
        .collect(),
},
```

## Verify the Integration

1. Build your runtime and node, then start the node locally with its development configuration.
2. Build and start the `eth-rpc` adapter against your node, as described in [Run the Ethereum JSON-RPC Adapter](/smart-contracts/eth-rpc-adapter/).
3. Request the chain ID through the adapter:

    ```bash
    curl -s -X POST {{ networks.local.eth_rpc_url }} \
      -H "Content-Type: application/json" \
      -d '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}'
    ```

    The result is your `ChainId` value in hexadecimal. If the adapter fails to start, check its logs for missing runtime APIs or constants.

## Where to Go Next

<div class="grid cards" markdown>

-   <span class="badge guide">Guide</span> __Run the Ethereum JSON-RPC Adapter__

    ---

    Build, run, and configure the `eth-rpc` adapter next to your node.

    [:octicons-arrow-right-24: Get Started](/smart-contracts/eth-rpc-adapter/)

-   <span class="badge learn">Learn</span> __Smart Contracts Overview__

    ---

    Learn how `pallet-revive`, its execution backends, and its tooling fit together.

    [:octicons-arrow-right-24: Learn More](/smart-contracts/overview/)

</div>
