---
title: Test XCM with the Emulator and Simulator
description: Test cross-chain messages in Rust with xcm-simulator mock runtimes or xcm-emulator real runtimes, assert on destination events, and dry-run XCM in tests.
categories: Polkadot SDK, Interoperability
---

# Test XCM with the Emulator and Simulator

## Introduction

The Polkadot SDK ships two Rust test kits for Cross-Consensus Messaging (XCM). Both run a relay chain and several parachains inside one test process, pass messages between them in memory, and let you assert on the state and events of each chain. They differ in which runtimes they run:

- **[`xcm-simulator`](https://github.com/paritytech/polkadot-sdk/tree/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot/xcm/xcm-simulator)**: Runs small mock runtimes that you write for the test. Use it to exercise XCM instructions and your XCM configuration against the XCM executor with fast, unit-style tests.
- **[`xcm-emulator`](https://github.com/paritytech/polkadot-sdk/tree/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/xcm/xcm-emulator)**: Runs real runtimes, such as your parachain runtime, the Westend relay chain runtime, and Asset Hub, with their genesis configuration. Use it to verify cross-chain flows, fees, weights, and side effects as they would occur with production runtime code.

Neither tool runs consensus or networking. The emulator's [README](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/xcm/xcm-emulator/README.md) notes that message transport is mocked, so areas such as disputes and staking cannot be tested. For end-to-end tests with real nodes, use [Zombienet](/polkadot-sdk/testing/zombienet-tests.md).

This guide shows how each tool declares a test network, how to write a test that sends a message and asserts on the destination, and how to call the XCM dry-run and fee APIs from an emulator test. Code samples come from the Polkadot SDK at release `{{ dependencies.repositories.polkadot_sdk.version }}`.

## Prerequisites

Before you begin, ensure you have the following:

- [Rust](https://rust-lang.org/tools/install/) and Cargo.
- A runtime with an XCM configuration. See [Configure XCM in Your Runtime](/polkadot-sdk/interoperability/configure-xcm.md).
- Familiarity with FRAME runtime tests that use `sp_io::TestExternalities`.

## Choose a Tool

The following table compares the two test kits:

| | `xcm-simulator` | `xcm-emulator` |
|---|---|---|
| Runtimes | Mock runtimes defined in the test crate | Real runtime crates with their genesis config |
| Parachain message handling | `mock_message_queue` pallet from `xcm-simulator` | The runtime's own XCMP queue and message queue pallets |
| Declaration macros | `decl_test_relay_chain!`, `decl_test_parachain!`, `decl_test_network!` | `decl_test_relay_chains!`, `decl_test_parachains!`, `decl_test_networks!` |
| Crate version at `{{ dependencies.repositories.polkadot_sdk.version }}` | `{{ dependencies.crates.xcm_simulator.version }}` | `{{ dependencies.crates.xcm_emulator.version }}` |
| API docs | [`xcm_simulator`](https://paritytech.github.io/polkadot-sdk/master/xcm_simulator/index.html) | [`xcm_emulator`](https://paritytech.github.io/polkadot-sdk/master/xcm_emulator/index.html) |

Start with the simulator when you are developing an XCM configuration and want quick feedback on how the executor handles a message. Move to the emulator when you need to confirm that your real runtime interoperates with other real runtimes.

## Test with XCM Simulator

The reference for this section is the [`xcm-simulator-example`](https://github.com/paritytech/polkadot-sdk/tree/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot/xcm/xcm-simulator/example) crate. It defines a mock relay chain runtime in `src/relay_chain/` and a mock parachain runtime in `src/parachain/`, each with its own `xcm_config` module, and declares a network with two parachains in `src/lib.rs`.

### Add the Dependency

Add `xcm-simulator` to the crate that holds your tests, together with the FRAME and XCM crates your mock runtimes use:

```toml title="Cargo.toml"
[dev-dependencies]
xcm-simulator = "{{ dependencies.crates.xcm_simulator.version }}"
```

Use the crate versions that belong to the same Polkadot SDK release as the rest of your dependencies.

### Declare the Mock Network

Each mock parachain runtime includes the [`mock_message_queue`](https://paritytech.github.io/polkadot-sdk/master/xcm_simulator/mock_message_queue/index.html) pallet from `xcm-simulator`, which handles incoming XCMP and DMP messages. In the example, it is part of `construct_runtime!` as `MsgQueue: mock_message_queue`.

With the runtimes in place, declare the chains and the network with three macros:

- **`decl_test_parachain!`**: Wraps a parachain runtime. It takes the runtime, the pallets that handle XCMP and DMP messages, and a function that builds the chain's `TestExternalities`.
- **`decl_test_relay_chain!`**: Wraps the relay chain runtime. It takes the runtime and its `RuntimeCall`, `RuntimeEvent`, `XcmConfig`, `MessageQueue`, and `System` types, plus a `TestExternalities` builder.
- **`decl_test_network!`**: Combines the relay chain and a list of `(para_id, parachain)` pairs into one network, and generates the XCM routers that the mock runtimes use to send messages.

The following declarations are from the example's `src/lib.rs`:

```rust title="src/lib.rs"
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

decl_test_parachain! {
	pub struct ParaA {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(1),
	}
}

decl_test_parachain! {
	pub struct ParaB {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(2),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		RuntimeCall = relay_chain::RuntimeCall,
		RuntimeEvent = relay_chain::RuntimeEvent,
		XcmConfig = relay_chain::XcmConfig,
		MessageQueue = relay_chain::MessageQueue,
		System = relay_chain::System,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
			(2, ParaB),
		],
	}
}
```

The `para_ext` and `relay_ext` functions build genesis storage for each chain, for example, by funding test accounts through `pallet_balances::GenesisConfig`. The example's `para_ext` also calls `MsgQueue::set_para_id` so that each mock parachain knows its own ID.

### Write a Simulator Test

Each declared chain implements the simulator's `TestExt` trait. Its `execute_with` method runs a closure against that chain's storage and then delivers all queued messages to their destinations. To test a message, send it inside one chain's `execute_with` and assert on the result inside the destination's `execute_with`. Call `MockNet::reset()` at the start of each test to clear the message buses and rebuild every chain's storage.

The following test from the example's [`src/tests.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/polkadot/xcm/xcm-simulator/example/src/tests.rs) sends a `Transact` instruction from the relay chain to parachain 1 and asserts that the parachain emitted a `Remarked` event:

```rust title="src/tests.rs"
#[test]
fn dmp() {
	MockNet::reset();

	let remark = parachain::RuntimeCall::System(
		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	Relay::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::send_xcm(
			Here,
			Parachain(1),
			Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				call: remark.encode().into(),
				fallback_max_weight: None,
			}]),
		));
	});

	ParaA::execute_with(|| {
		assert!(system_contains_event!(parachain, System(frame_system::Event::Remarked { .. })));
	});
}
```

`RelayChainPalletXcm` is a type alias for `pallet_xcm::Pallet<relay_chain::Runtime>`, and `system_contains_event!` is a helper macro defined in the same file that checks `System::events()` for a matching event. The same file contains tests for upward messages (`ump`), messages between parachains (`xcmp`), reserve transfers, teleports, NFTs, and queries.

### Run the Simulator Example

To run the example tests, clone the Polkadot SDK at the pinned release and run them with Cargo:

```bash
git clone --branch {{ dependencies.repositories.polkadot_sdk.version }} --depth 1 https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk
cargo test -p xcm-simulator-example
```

## Test with XCM Emulator

The emulator runs real runtimes, so a test needs the runtime crates and a genesis configuration for every chain in the network. The Polkadot SDK organizes its own emulated tests under [`cumulus/parachains/integration-tests/emulated/`](https://github.com/paritytech/polkadot-sdk/tree/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated) in three layers, which you can follow for your own project:

- **`chains/`**: One crate per chain. Each declares a relay chain or parachain with its genesis, for example, `westend-emulated-chain` and `penpal-emulated-chain`.
- **`networks/`**: Crates that combine chains into a network, for example, `westend-system-emulated-network`.
- **`tests/`**: Test crates that use a network, for example, `asset-hub-westend-integration-tests`.

Shared helpers, such as test accounts, genesis builders, and event assertions, live in the [`emulated-integration-tests-common`](https://github.com/paritytech/polkadot-sdk/tree/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/common) crate, which re-exports `xcm_emulator`.

### Add the Emulator Dependencies

Add `xcm-emulator`, or `emulated-integration-tests-common` if you want its helpers, to the crates that declare your chains and tests, together with the runtime crates you emulate:

```toml title="Cargo.toml"
[dev-dependencies]
xcm-emulator = "{{ dependencies.crates.xcm_emulator.version }}"
emulated-integration-tests-common = "{{ dependencies.crates.emulated_integration_tests_common.version }}"
```

These are the versions in `{{ dependencies.repositories.polkadot_sdk.version }}`. The chain and network crates in the Polkadot SDK, such as `westend-emulated-chain` and `westend-system-emulated-network`, are not published to crates.io (`publish = false`). To reuse them, add them as Git dependencies on the Polkadot SDK repository at your release tag, or copy their declarations into your own crates.

### Declare Emulated Chains

Declare a relay chain with `decl_test_relay_chains!`. You provide the genesis storage, code to run when the chain initializes, the runtime crate, the location converter for sovereign accounts, and the pallets you want to access from tests. The following declaration is from [`chains/relays/westend/src/lib.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/chains/relays/westend/src/lib.rs):

```rust title="chains/relays/westend/src/lib.rs"
decl_test_relay_chains! {
	#[api_version(16)]
	pub struct Westend {
		genesis = genesis::genesis(),
		on_init = (),
		runtime = westend_runtime,
		core = {
			SovereignAccountOf: westend_runtime::xcm_config::LocationConverter,
		},
		pallets = {
			XcmPallet: westend_runtime::XcmPallet,
			Sudo: westend_runtime::Sudo,
			Balances: westend_runtime::Balances,
			AssetRate: westend_runtime::AssetRate,
			Hrmp: westend_runtime::Hrmp,
			Identity: westend_runtime::Identity,
			IdentityMigrator: westend_runtime::IdentityMigrator,
		}
	},
}
```

Declare parachains with `decl_test_parachains!`. The `core` block names the runtime's XCMP message handler, its location-to-account converter, the `ParachainInfo` pallet, and the message origin type. The following declaration of the Penpal test parachain is from [`chains/parachains/testing/penpal/src/lib.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/chains/parachains/testing/penpal/src/lib.rs). To emulate your own parachain, replace `penpal_runtime` with your runtime crate and the types with the ones your runtime defines:

```rust title="chains/parachains/testing/penpal/src/lib.rs"
decl_test_parachains! {
	pub struct PenpalA {
		genesis = genesis(PARA_ID_A),
		on_init = {
			penpal_runtime::AuraExt::on_initialize(1);
			frame_support::assert_ok!(penpal_runtime::System::set_storage(
				penpal_runtime::RuntimeOrigin::root(),
				vec![(PenpalRelayNetworkId::key().to_vec(), NetworkId::ByGenesis(ROCOCO_GENESIS_HASH).encode())],
			));
		},
		runtime = penpal_runtime,
		core = {
			XcmpMessageHandler: penpal_runtime::XcmpQueue,
			LocationToAccountId: penpal_runtime::xcm_config::LocationToAccountId,
			ParachainInfo: penpal_runtime::ParachainInfo,
			MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
		},
		pallets = {
			PolkadotXcm: penpal_runtime::PolkadotXcm,
			Assets: penpal_runtime::Assets,
			AssetConversion: penpal_runtime::AssetConversion,
			Balances: penpal_runtime::Balances,
		}
	},
}
```

The `genesis` function returns the chain's genesis `Storage`, built from the runtime's `RuntimeGenesisConfig` with the parachain ID, balances, collators, and XCM version you need. For each entry under `pallets`, the macro generates a trait named after the chain, such as `PenpalAParaPallet`, so tests can refer to `<PenpalA as PenpalAParaPallet>::PolkadotXcm`. The chain crates also call helper macros from `emulated-integration-tests-common`, such as `impl_assert_events_helpers_for_parachain!(PenpalA)`, which add methods like `assert_xcm_pallet_sent` and `assert_xcmp_queue_success` to the chain.

### Declare the Emulated Network

Combine the chains into a network with `decl_test_networks!`. The following declaration is from [`networks/westend-system/src/lib.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/networks/westend-system/src/lib.rs):

```rust title="networks/westend-system/src/lib.rs"
decl_test_networks! {
	pub struct WestendMockNet {
		relay_chain = Westend,
		parachains = vec![
			AssetHubWestend,
			BridgeHubWestend,
			CollectivesWestend,
			CoretimeWestend,
			PeopleWestend,
			PenpalA,
			PenpalB,
		],
		bridge = ()
	},
}

decl_test_sender_receiver_accounts_parameter_types! {
	WestendRelay { sender: ALICE, receiver: BOB },
	AssetHubWestendPara { sender: ALICE, receiver: BOB },
	PenpalAPara { sender: ALICE, receiver: BOB },
	PenpalBPara { sender: ALICE, receiver: BOB }
	// ...
}
```

Every chain type is generic over the network it belongs to. `decl_test_networks!` generates aliases that bind each chain to this network: `WestendRelay` for the relay chain and `<Name>Para`, such as `PenpalAPara`, for each parachain. The test crate imports these aliases under shorter names, for example, `PenpalAPara as PenpalA`. `decl_test_sender_receiver_accounts_parameter_types!` defines `PenpalAParaSender`, `PenpalAParaReceiver`, and similar getters for the development accounts `//Alice` and `//Bob` on each chain.

### Write an Emulator Test

Each emulated chain implements the emulator's `TestExt` trait. Its `execute_with` method runs a closure against that chain's storage, then lets the network deliver the messages the closure produced, and clears the chain's events. As a result, events you assert on inside a chain's `execute_with` are the events emitted while that chain processed the messages it received.

The following excerpt is from `penpal_register_foreign_asset_on_asset_hub` in [`tests/assets/asset-hub-westend/src/tests/send.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/tests/assets/asset-hub-westend/src/tests/send.rs). Penpal A sends an XCM program to Asset Hub with `pallet_xcm::send`, and the test then asserts on Asset Hub that the message was processed and the fee was withdrawn from Penpal A's sovereign account:

```rust title="tests/assets/asset-hub-westend/src/tests/send.rs"
PenpalA::execute_with(|| {
	assert_ok!(<PenpalA as PenpalAPallet>::PolkadotXcm::send(
		root_origin,
		bx!(system_para_destination),
		bx!(xcm),
	));

	PenpalA::assert_xcm_pallet_sent();
});

AssetHubWestend::execute_with(|| {
	type RuntimeEvent = <AssetHubWestend as Chain>::RuntimeEvent;
	AssetHubWestend::assert_xcmp_queue_success(None);
	assert_expected_events!(
		AssetHubWestend,
		vec![
			// Withdraw the fee
			RuntimeEvent::Balances(pallet_balances::Event::Withdraw { who, amount }) => {
				who: *who == penpal_sovereign_account,
				amount: *amount == fee_amount,
			},
			// Foreign Asset created
			RuntimeEvent::ForeignAssets(pallet_assets::Event::Created { asset_id, creator, owner }) => {
				asset_id: *asset_id == foreign_asset_at_asset_hub,
				creator: *creator == penpal_sovereign_account.clone(),
				owner: *owner == penpal_sovereign_account,
			},
		]
	);
});
```

The excerpt uses the following emulator features:

- **`assert_expected_events!`**: Takes a chain and a list of event patterns, each with conditions on the event's fields. It panics with the list of received events if a pattern is never matched or a condition fails.
- **`bx!`**: Wraps a value in a `Box`, as required by the `pallet_xcm` extrinsics.
- **`Chain`**: The trait that exposes each chain's `Runtime`, `RuntimeCall`, `RuntimeOrigin`, and `RuntimeEvent` types, and helpers such as `account_id_of` and `account_data_of`.

Before the excerpt, the test computes locations and accounts with `Parachain` trait methods, such as `sibling_location_of` and `sovereign_account_id_of`, builds the XCM program, and funds the sovereign account with `fund_accounts`, a method added by `impl_accounts_helpers_for_parachain!`.

For transfers that cross several chains, the emulator also provides the `Test` struct. You create it from a `TestContext` with the sender, the receiver, and `TestArgs`, register assertion functions for each chain with `set_assertion`, set the call to dispatch on the origin chain with `set_call` or a dispatchable with `set_dispatchable`, and then run everything in order with `assert`. The [`teleport.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/tests/assets/asset-hub-westend/src/tests/teleport.rs) and [`xcm_fee_estimation.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/tests/assets/asset-hub-westend/src/tests/xcm_fee_estimation.rs) tests use this pattern.

### Run the Emulated Tests

Each test crate under `tests/` is a regular Cargo package. To run the Asset Hub Westend tests from a Polkadot SDK checkout at the pinned release, use the package name from its `Cargo.toml`:

```bash
cargo test -p asset-hub-westend-integration-tests
```

When an `assert_expected_events!` assertion fails, the panic message lists every event the chain emitted, which is usually the fastest way to see why a message did not have the expected effect.

## Use the XCM Runtime APIs in Tests

Because the emulator runs real runtimes, an emulator test can call the [`DryRunApi`](https://paritytech.github.io/polkadot-sdk/master/xcm_runtime_apis/dry_run/trait.DryRunApi.html) and [`XcmPaymentApi`](https://paritytech.github.io/polkadot-sdk/master/xcm_runtime_apis/fees/trait.XcmPaymentApi.html) implementations of a runtime directly. This lets you check that the fees a wallet would estimate match what the transfer actually charges. The runtime must implement both APIs, as shown in [Configure XCM in Your Runtime](/polkadot-sdk/interoperability/configure-xcm.md#add-the-xcm-runtime-apis).

To call the API functions on a `Runtime` type, import the traits that `impl_runtime_apis!` generates for each API version:

```rust
use xcm_runtime_apis::{
	dry_run::runtime_decl_for_dry_run_api::DryRunApiV2,
	fees::runtime_decl_for_xcm_payment_api::XcmPaymentApiV2,
};
```

The following excerpt is from the `multi_hop_works` test in [`xcm_fee_estimation.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{ dependencies.repositories.polkadot_sdk.version }}/cumulus/parachains/integration-tests/emulated/tests/assets/asset-hub-westend/src/tests/xcm_fee_estimation.rs). Inside Penpal A, it dry-runs a transfer call, picks the message forwarded to Asset Hub (parachain 1000), and queries the delivery fee for that message:

```rust title="tests/assets/asset-hub-westend/src/tests/xcm_fee_estimation.rs"
<PenpalA as TestExt>::execute_with(|| {
	type Runtime = <PenpalA as Chain>::Runtime;
	type OriginCaller = <PenpalA as Chain>::OriginCaller;

	let call = transfer_assets_para_to_para_through_ah_call(test.clone());
	let origin = OriginCaller::system(RawOrigin::Signed(sender.clone()));
	let result = Runtime::dry_run_call(origin, call, xcm::prelude::XCM_VERSION).unwrap();
	// We filter the result to get only the messages we are interested in.
	let (destination_to_query, messages_to_query) = &result
		.forwarded_xcms
		.iter()
		.find(|(destination, _)| {
			*destination == VersionedLocation::from(Location::new(1, [Parachain(1000)]))
		})
		.unwrap();
	assert_eq!(messages_to_query.len(), 1);
	remote_message = messages_to_query[0].clone();
	let asset_id_for_delivery_fees = VersionedAssetId::from(Location::here());
	let delivery_fees = Runtime::query_delivery_fees(
		destination_to_query.clone(),
		remote_message.clone(),
		asset_id_for_delivery_fees,
	)
	.unwrap();
	delivery_fees_amount = get_amount_from_versioned_assets(delivery_fees);
});
```

The test then continues on the next hops:

1. Inside Asset Hub, it calls `query_xcm_weight` and `query_weight_to_asset_fee` to get the execution fee for the forwarded message, and `dry_run_xcm` followed by `query_delivery_fees` to get the delivery fee to the final destination.
2. Inside Penpal B, it calls `query_xcm_weight` and `query_weight_to_asset_fee` again to get the execution fee at the destination.
3. It calls `reset_ext` on each chain to discard any state the dry runs changed, then runs the real transfer with the `Test` struct and asserts that the sender and receiver balances changed by exactly the estimated fees.

## Where to Go Next

<div class="grid cards" markdown>

-   __Configure XCM in Your Runtime__

    ---

    Set up the XCM executor, `pallet-xcm`, the message queues, and the XCM runtime APIs that these tests exercise.

    [:octicons-arrow-right-24: Configure XCM](/polkadot-sdk/interoperability/configure-xcm.md)

-   __Write Network Tests with Zombienet__

    ---

    Test your chains end to end with real nodes, a relay chain, and collators.

    [:octicons-arrow-right-24: Zombienet tests](/polkadot-sdk/testing/zombienet-tests.md)

-   __XCM Fee Estimation__

    ---

    Call `DryRunApi` and `XcmPaymentApi` from a client to estimate the fees of a transfer.

    [:octicons-arrow-right-24: Estimate XCM fees](/tools/chain-interactions/send-transactions/interoperability/estimate-xcm-fees.md)

</div>
