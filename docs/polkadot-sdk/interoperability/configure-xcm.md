---
title: Configure XCM in Your Runtime
description: Configure the XCM executor, pallet-xcm, XCMP and message queues, and the XCM dry-run and fee runtime APIs in a Polkadot SDK parachain runtime.
categories: Polkadot SDK, Interoperability
---

# Configure XCM in Your Runtime

## Introduction

[Get Started with XCM](/polkadot-sdk/interoperability/get-started/) describes the Cross-Consensus Messaging (XCM) format. To send and receive XCM messages, a Polkadot SDK runtime also needs configuration: it decides which messages it executes, how it converts locations into accounts and origins, which assets it holds and trusts, how it charges for execution, and how messages travel to the relay chain and to other parachains.

This guide walks through that configuration as it appears in the parachain template in the Polkadot SDK, at the `{{dependencies.repositories.polkadot_sdk.version}}` release. It covers the following:

- The pallets that handle XCM in a parachain runtime.
- The `xcm_config.rs` file and the items of the XCM executor's `Config` trait.
- The `pallet_xcm::Config` implementation.
- The XCMP and message queue wiring that carries messages to and from other chains.
- The `DryRunApi` and `XcmPaymentApi` runtime APIs, which wallets and tools use to simulate messages and estimate fees.

Code excerpts come from the parachain template runtime ([`templates/parachain/runtime`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime)) and, where the template does not include a feature, from the People Westend system parachain runtime ([`cumulus/parachains/runtimes/people/people-westend`](https://github.com/paritytech/polkadot-sdk/tree/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/people/people-westend)). For the full reference, see the [XCM docs](https://paritytech.github.io/polkadot-sdk/master/xcm_docs/index.html) and the rustdocs linked in each section.

## Check Prerequisites

Before you begin, make sure you have:

- A parachain runtime based on the [Polkadot SDK parachain template](https://github.com/paritytech/polkadot-sdk-parachain-template). [Set Up the Parachain Template](/polkadot-sdk/launch-a-parachain/set-up-the-parachain-template/) walks through building it.
- Familiarity with configuring pallets in a runtime, as shown in [Add an Existing Pallet to the Runtime](/polkadot-sdk/customize-runtime/add-existing-pallets/).
- A basic understanding of XCM locations, assets, and instructions from [Get Started with XCM](/polkadot-sdk/interoperability/get-started/).

## XCM Pallets in a Parachain Runtime

A parachain runtime uses four pallets for XCM. The parachain template declares them together in [`runtime/src/lib.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/src/lib.rs):

```rust title="runtime/src/lib.rs"
// XCM helpers.
#[runtime::pallet_index(30)]
pub type XcmpQueue = cumulus_pallet_xcmp_queue;
#[runtime::pallet_index(31)]
pub type PolkadotXcm = pallet_xcm;
#[runtime::pallet_index(32)]
pub type CumulusXcm = cumulus_pallet_xcm;
#[runtime::pallet_index(33)]
pub type MessageQueue = pallet_message_queue;
```

Each pallet has the following role:

- **[`pallet-xcm`](https://paritytech.github.io/polkadot-sdk/master/pallet_xcm/index.html)**: Exposes extrinsics to send and execute XCM, handles XCM version negotiation, query responses, and asset traps, and provides the helpers behind the XCM runtime APIs.
- **[`cumulus-pallet-xcmp-queue`](https://paritytech.github.io/polkadot-sdk/master/cumulus_pallet_xcmp_queue/index.html)**: Sends messages to sibling parachains and receives messages from them over XCMP channels.
- **[`pallet-message-queue`](https://paritytech.github.io/polkadot-sdk/master/pallet_message_queue/index.html)**: Stores incoming messages from the relay chain and from siblings, and executes them with the XCM executor within a weight budget.
- **[`cumulus-pallet-xcm`](https://paritytech.github.io/polkadot-sdk/master/cumulus_pallet_xcm/index.html)**: Provides the `Relay` and `SiblingParachain` origins that the origin converters use.

The template also includes [`cumulus-pallet-parachain-system`](https://paritytech.github.io/polkadot-sdk/master/cumulus_pallet_parachain_system/index.html), which is required for any parachain and which connects the queues to the relay chain.

The template's runtime enables the XCM crates through features of the `polkadot-sdk` dependency in [`runtime/Cargo.toml`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/Cargo.toml): `pallet-xcm`, `staging-xcm`, `staging-xcm-builder`, `staging-xcm-executor`, `cumulus-pallet-xcm`, `cumulus-pallet-xcmp-queue`, `pallet-message-queue`, and `cumulus-primitives-utility`.

## Structure of xcm_config.rs

The XCM configuration lives in [`runtime/src/configs/xcm_config.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/src/configs/xcm_config.rs). The file defines, in order:

1. Parameter types for the chain's own location and the relay chain's location.
2. The location-to-account converter.
3. The asset transactor.
4. The origin converter.
5. The barrier.
6. The `XcmConfig` struct, which implements [`xcm_executor::Config`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_executor/trait.Config.html).
7. The XCM router, which sends outgoing messages.
8. The `pallet_xcm::Config` and `cumulus_pallet_xcm::Config` implementations.

The queue pallets are configured in [`runtime/src/configs/mod.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/src/configs/mod.rs), next to the other pallet configurations.

Most of the types used in `xcm_config.rs` are ready-made building blocks from [`staging-xcm-builder`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/index.html). You configure XCM by combining these blocks, usually as tuples that are tried in order.

### Locations

The template defines the relay chain location and the chain's universal location, which is the chain's own location as seen from the global consensus:

```rust title="runtime/src/configs/xcm_config.rs"
parameter_types! {
	pub const RelayLocation: Location = Location::parent();
	pub const RelayNetwork: Option<NetworkId> = None;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	// For the real deployment, it is recommended to set `RelayNetwork` according to the relay chain
	// and prepend `UniversalLocation` with `GlobalConsensus(RelayNetwork::get())`.
	pub UniversalLocation: InteriorLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}
```

As the comment in the source says, a runtime meant for a live network sets `RelayNetwork` to that relay chain's network ID and adds a `GlobalConsensus` junction to `UniversalLocation`.

### Location Converters

The `LocationToAccountId` type converts an XCM location into a local account ID. The executor uses it to find the account that owns assets and to derive the dispatch origin for `Transact`:

```rust title="runtime/src/configs/xcm_config.rs"
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);
```

The converters are tried in order. The first two produce the sovereign accounts of the relay chain and of sibling parachains on this chain. Add converters to this tuple if your chain must recognize other kinds of locations, for example, accounts on other chains.

### Asset Transactors

The asset transactor withdraws and deposits assets when the executor processes instructions such as `WithdrawAsset` and `DepositAsset`. The template uses a [`FungibleAdapter`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.FungibleAdapter.html) that maps the relay chain's native token to the local `Balances` pallet:

```rust title="runtime/src/configs/xcm_config.rs"
pub type LocalAssetTransactor = FungibleAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<RelayLocation>,
	// Do a simple punn to convert an AccountId32 Location into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;
```

To support more assets, such as assets from a `pallet-assets` instance, combine several transactors in a tuple. `staging-xcm-builder` provides [`FungiblesAdapter`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.FungiblesAdapter.html) for multi-asset pallets.

### Origin Converters

The origin converter turns the origin of an incoming XCM into a local `RuntimeOrigin` when the message contains a `Transact` instruction. The `OriginKind` in the instruction selects which converter applies:

```rust title="runtime/src/configs/xcm_config.rs"
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `RuntimeOrigin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);
```

### Barriers

The barrier decides whether the executor runs a message at all. It is the first check applied to every incoming message, so it is the main security boundary of your XCM configuration:

```rust title="runtime/src/configs/xcm_config.rs"
pub type Barrier = TrailingSetTopicAsId<
	DenyThenTry<
		DenyRecursively<DenyReserveTransferToRelayChain>,
		(
			TakeWeightCredit,
			WithComputedOrigin<
				(
					AllowTopLevelPaidExecutionFrom<Everything>,
					AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
					// ^^^ Parent and its exec plurality get free execution
				),
				UniversalLocation,
				ConstU32<8>,
			>,
		),
	>,
>;
```

This barrier works as follows:

- [`DenyThenTry`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.DenyThenTry.html) first applies the deny filter, which rejects messages that try a reserve transfer to the relay chain, and then accepts the message if any of the allow filters in the tuple passes.
- `TakeWeightCredit` allows messages whose weight is covered by weight credit, which lets local users execute XCM through extrinsics such as the transfer extrinsics of `pallet-xcm`.
- `AllowTopLevelPaidExecutionFrom<Everything>` allows messages from any origin that pay for their execution with `BuyExecution`.
- `AllowExplicitUnpaidExecutionFrom` allows the relay chain and its executive body to execute without paying, if the message starts with `UnpaidExecution`.
- `WithComputedOrigin` lets the allow filters evaluate the origin that results from origin-changing instructions such as `DescendOrigin` at the start of a message.

The template's barrier is minimal. System parachain runtimes add filters such as `AllowKnownQueryResponses<PolkadotXcm>` for expected query responses, `AllowSubscriptionsFrom` for XCM version subscriptions, and `AllowHrmpNotificationsFromRelayChain` for channel notifications. See the [People Westend `xcm_config.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/people/people-westend/src/xcm_config.rs) for an example.

### Weigher and Trader

The weigher calculates the weight of a message, and the trader charges the sender for that weight in an asset the chain accepts. The template uses the following types:

- [`FixedWeightBounds`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.FixedWeightBounds.html) as the weigher, which assigns the same weight, `UnitWeightCost`, to every instruction and limits messages to `MaxInstructions` instructions.
- [`UsingComponents`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.UsingComponents.html) as the trader, which converts weight to a fee in the relay chain token with the runtime's `WeightToFee` and pays the fee to the block author through `ToAuthor`.

```rust title="runtime/src/configs/xcm_config.rs"
parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}
```

Fixed weights are a placeholder for development. Production runtimes use [`WeightInfoBounds`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.WeightInfoBounds.html) with benchmarked weights for each instruction, as the People Westend runtime does:

```rust title="people-westend/src/xcm_config.rs"
type Weigher = WeightInfoBounds<
	crate::weights::xcm::PeopleWestendXcmWeight<RuntimeCall>,
	RuntimeCall,
	MaxInstructions,
>;
```

### Reserve and Teleport Filters

`IsReserve` and `IsTeleporter` decide which chains this chain trusts for which assets:

- **`IsReserve`**: The (asset, location) pairs that this chain accepts as reserves. Reserve-backed transfers from untrusted pairs are rejected.
- **`IsTeleporter`**: The (asset, location) pairs that this chain accepts teleports from. A teleport burns the asset on one chain and mints it on the other, so only enable it between chains that trust each other's runtimes.

The template sets `IsReserve = NativeAsset`, which accepts an asset only when the sending location is that asset's native location, and disables teleports with `IsTeleporter = ()`. System parachains trust the relay chain and other system parachains for teleports of the relay chain token, for example:

```rust title="people-westend/src/xcm_config.rs"
pub type TrustedTeleporters = ConcreteAssetFromSystem<RelayLocation>;
```

### The XcmConfig Struct

`XcmConfig` collects the preceding types and the remaining executor settings. The template's implementation is:

```rust title="runtime/src/configs/xcm_config.rs"
pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type XcmEventEmitter = PolkadotXcm;
	// How to withdraw and deposit an asset.
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = (); // Teleporting is disabled.
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader =
		UsingComponents<WeightToFee, RelayLocation, AccountId, Balances, ToAuthor<Runtime>>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
	type XcmRecorder = PolkadotXcm;
}
```

The items not covered in earlier sections are described in the [`xcm_executor::Config`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_executor/trait.Config.html) rustdoc. The following are the ones you are most likely to change:

- **`XcmSender`**: The router that sends messages to other chains. See [Message Routing](#message-routing).
- **`ResponseHandler`, `AssetTrap`, and `SubscriptionService`**: Set to `PolkadotXcm` so that `pallet-xcm` handles query responses, traps assets left over after execution, and manages version subscriptions.
- **`SafeCallFilter`**: The calls that `Transact` can dispatch. The template allows every call.
- **`Aliasers`**: The (origin, target) location pairs for which the `AliasOrigin` instruction is allowed. The template disables aliasing with `Nothing`.
- **`AssetExchanger`**: Swaps assets during execution, which also lets the executor pay delivery fees in a different asset. The template has no exchanger.
- **`XcmRecorder`**: Records the last executed XCM. The dry-run runtime API uses it, so keep it set to `PolkadotXcm`.

## Message Routing

`XcmRouter` sends outgoing messages over the right transport. Messages to the relay chain go through Upward Message Passing (UMP), and messages to sibling parachains go through the XCMP queue:

```rust title="runtime/src/configs/xcm_config.rs"
pub type XcmRouter = WithUniqueTopic<(
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, PriceForParentDelivery>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
)>;
```

`WithUniqueTopic` appends a `SetTopic` instruction with a universally unique ID to each message that does not already end with one, so the message can be traced across chains. `PriceForParentDelivery` sets the delivery fee for upward messages. It is defined in `configs/mod.rs` and described in [Configure the XCMP and Message Queues](#configure-the-xcmp-and-message-queues).

## Configure pallet-xcm

The `pallet_xcm::Config` implementation connects `pallet-xcm` to the executor and router, and controls which of the pallet's extrinsics are allowed:

```rust title="runtime/src/configs/xcm_config.rs"
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	// ^ Disable dispatchable execute on the XCM pallet.
	// Needs to be `Everything` for local testing.
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Nothing;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;

	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
	// Aliasing is disabled: xcm_executor::Config::Aliasers is set to `Nothing`.
	type AuthorizedAliasConsideration = Disabled;
}
```

The items that most affect what users can do are:

- **`SendXcmOrigin` and `ExecuteXcmOrigin`**: The origins allowed to call `send` and `execute`, and how those origins are converted into XCM locations. `LocalOriginToLocation` converts a signed account into an `AccountId32` location.
- **`XcmExecuteFilter`**: The messages that the `execute` extrinsic accepts. The template disables `execute` with `Nothing`; as the source comment says, set it to `Everything` for local testing.
- **`XcmTeleportFilter` and `XcmReserveTransferFilter`**: The (origin, assets) pairs that the teleport and reserve transfer extrinsics accept. These filters apply to outgoing transfers started on this chain, while `IsTeleporter` and `IsReserve` apply to incoming messages.
- **`Weigher`**: Usually the same weigher as in `XcmConfig`.
- **`AdminOrigin`**: The origin allowed to call the pallet's privileged operations.
- **`WeightInfo`**: The template uses `pallet_xcm::TestWeightInfo`. Replace it with benchmarked weights for a production runtime.

The full list of items is in the [`pallet_xcm::Config`](https://paritytech.github.io/polkadot-sdk/master/pallet_xcm/pallet/trait.Config.html) rustdoc.

The template also implements `cumulus_pallet_xcm::Config`, which only needs the executor:

```rust title="runtime/src/configs/xcm_config.rs"
impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}
```

## Configure the XCMP and Message Queues

Messages between a parachain and other chains pass through three pallets. `cumulus-pallet-parachain-system` receives downward messages from the relay chain and inbound XCMP messages, `cumulus-pallet-xcmp-queue` manages the XCMP channels, and `pallet-message-queue` stores every inbound message and runs it through the XCM executor. The template wires them together in [`runtime/src/configs/mod.rs`](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/templates/parachain/runtime/src/configs/mod.rs).

`cumulus_pallet_parachain_system::Config` hands outbound XCMP messages from the XCMP queue to the collator, enqueues downward messages in `MessageQueue` with the relay chain as their origin, and passes inbound XCMP messages to the XCMP queue:

```rust title="runtime/src/configs/mod.rs"
parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	// ...
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	// ...
}
```

`pallet_message_queue::Config` processes queued messages with the XCM executor through [`ProcessXcmMessage`](https://paritytech.github.io/polkadot-sdk/master/staging_xcm_builder/struct.ProcessXcmMessage.html). `ServiceWeight` limits how much of each block is spent processing messages:

```rust title="runtime/src/configs/mod.rs"
parameter_types! {
	pub MessageQueueServiceWeight: Weight = Perbill::from_percent(35) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
		cumulus_primitives_core::AggregateMessageOrigin,
	>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor = xcm_builder::ProcessXcmMessage<
		AggregateMessageOrigin,
		xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
		RuntimeCall,
	>;
	type Size = u32;
	// The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
	type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
	type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
	type HeapSize = sp_core::ConstU32<{ 103 * 1024 }>;
	type MaxStale = sp_core::ConstU32<8>;
	type ServiceWeight = MessageQueueServiceWeight;
	type IdleMaxServiceWeight = ();
}
```

`cumulus_pallet_xcmp_queue::Config` enqueues messages from siblings in `MessageQueue`, reads channel information from `ParachainSystem`, and sets the delivery fee for messages to siblings. The delivery fee types use [`ExponentialPrice`](https://paritytech.github.io/polkadot-sdk/master/polkadot_runtime_common/xcm_sender/struct.ExponentialPrice.html), which charges a base fee plus a fee for each byte of the encoded message, multiplied by a fee factor that the queue provides. The fees are paid in the relay chain token:

```rust title="runtime/src/configs/mod.rs"
parameter_types! {
	/// The asset ID for the asset that we use to pay for message delivery fees.
	pub FeeAssetId: AssetId = AssetId(xcm_config::RelayLocation::get());
	/// The base fee for the message delivery fees.
	pub const ToSiblingBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
	pub const ToParentBaseDeliveryFee: u128 = CENTS.saturating_mul(3);
}

/// The price for delivering XCM messages to sibling parachains.
pub type PriceForSiblingParachainDelivery =
	ExponentialPrice<FeeAssetId, ToSiblingBaseDeliveryFee, TransactionByteFee, XcmpQueue>;

/// The price for delivering XCM messages to relay chain.
pub type PriceForParentDelivery =
	ExponentialPrice<FeeAssetId, ToParentBaseDeliveryFee, TransactionByteFee, ParachainSystem>;

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	// Enqueue XCMP messages from siblings for later processing.
	type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
	type MaxInboundSuspended = sp_core::ConstU32<1_000>;
	type MaxActiveOutboundChannels = ConstU32<128>;
	type MaxPageSize = ConstU32<{ 1 << 16 }>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
	type PriceForSiblingDelivery = PriceForSiblingParachainDelivery;
}
```

The template sets `VersionWrapper = ()`, so outgoing XCMP messages are not converted to the recipient's XCM version. System parachains set it to `PolkadotXcm`, which wraps messages in the version the destination supports.

The XCMP queue sends to a sibling only over an open HRMP channel; opening channels is a relay chain operation described in [Channels Between Parachains](https://developers.polkadot.com/parachains/interoperability/channels-between-parachains/).

## Add the XCM Runtime APIs

Wallets, SDKs, and tools such as the [XCM fee estimation flow](/tools/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/) rely on two runtime APIs from the [`xcm-runtime-apis`](https://paritytech.github.io/polkadot-sdk/master/xcm_runtime_apis/index.html) crate:

- **[`DryRunApi`](https://paritytech.github.io/polkadot-sdk/master/xcm_runtime_apis/dry_run/trait.DryRunApi.html)**: Simulates an extrinsic (`dry_run_call`) or an XCM (`dry_run_xcm`) and returns its result, emitted events, and the messages it would send to other chains.
- **[`XcmPaymentApi`](https://paritytech.github.io/polkadot-sdk/master/xcm_runtime_apis/fees/trait.XcmPaymentApi.html)**: Returns the assets the chain accepts for execution fees, the weight of an XCM, the fee for a weight in a given asset, and the delivery fee for sending a message to a destination.

The parachain template does not implement these APIs. To add them, enable the `xcm-runtime-apis` feature on the `polkadot-sdk` dependency in `runtime/Cargo.toml`, and then implement both APIs in the runtime's `impl_runtime_apis!` block. The following implementation is from the [People Westend runtime](https://github.com/paritytech/polkadot-sdk/blob/{{dependencies.repositories.polkadot_sdk.version}}/cumulus/parachains/runtimes/people/people-westend/src/lib.rs). Each method delegates to a helper in `pallet-xcm`:

```rust title="runtime/src/apis.rs"
use xcm::{prelude::*, Version as XcmVersion};
use xcm_runtime_apis::{
	dry_run::{CallDryRunEffects, Error as XcmDryRunApiError, XcmDryRunEffects},
	fees::Error as XcmPaymentApiError,
};

impl_runtime_apis! {
	// ...

	impl xcm_runtime_apis::fees::XcmPaymentApi<Block> for Runtime {
		fn query_acceptable_payment_assets(xcm_version: xcm::Version) -> Result<Vec<VersionedAssetId>, XcmPaymentApiError> {
			let acceptable_assets = vec![AssetId(xcm_config::RelayLocation::get())];
			PolkadotXcm::query_acceptable_payment_assets(xcm_version, acceptable_assets)
		}

		fn query_weight_to_asset_fee(weight: Weight, asset: VersionedAssetId) -> Result<u128, XcmPaymentApiError> {
			type Trader = <XcmConfig as xcm_executor::Config>::Trader;
			PolkadotXcm::query_weight_to_asset_fee::<Trader>(weight, asset)
		}

		fn query_xcm_weight(message: VersionedXcm<()>) -> Result<Weight, XcmPaymentApiError> {
			PolkadotXcm::query_xcm_weight(message)
		}

		fn query_delivery_fees(destination: VersionedLocation, message: VersionedXcm<()>, asset_id: VersionedAssetId) -> Result<VersionedAssets, XcmPaymentApiError> {
			type AssetExchanger = <XcmConfig as xcm_executor::Config>::AssetExchanger;
			PolkadotXcm::query_delivery_fees::<AssetExchanger>(destination, message, asset_id)
		}
	}

	impl xcm_runtime_apis::dry_run::DryRunApi<Block, RuntimeCall, RuntimeEvent, OriginCaller> for Runtime {
		fn dry_run_call(origin: OriginCaller, call: RuntimeCall, result_xcms_version: XcmVersion) -> Result<CallDryRunEffects<RuntimeEvent>, XcmDryRunApiError> {
			PolkadotXcm::dry_run_call::<Runtime, xcm_config::XcmRouter, OriginCaller, RuntimeCall>(origin, call, result_xcms_version)
		}

		fn dry_run_xcm(origin_location: VersionedLocation, xcm: VersionedXcm<RuntimeCall>) -> Result<XcmDryRunEffects<RuntimeEvent>, XcmDryRunApiError> {
			PolkadotXcm::dry_run_xcm::<xcm_config::XcmRouter>(origin_location, xcm)
		}
	}
}
```

Adapt the following parts to your runtime:

- `query_acceptable_payment_assets` lists the assets your `Trader` accepts. Keep it in sync with the trader; the template's trader accepts only the relay chain token.
- The dry-run helpers need `XcmRouter` and `XcmConfig` from `xcm_config.rs`. Make them public if they are not, and import them into the file that holds `impl_runtime_apis!`.
- The dry-run API depends on the executor's `XcmRecorder`, which must be set to `PolkadotXcm`, as it is in the template.

The same runtime also implements `LocationToAccountApi`, `TrustedQueryApi`, and `AuthorizedAliasersApi` from `xcm-runtime-apis`. They are optional, and you can add them in the same way.

After you add the APIs, rebuild the runtime and confirm that they appear in its metadata. A runtime upgrade is required to make them available on a running chain. See [Runtime Upgrades](/polkadot-sdk/runtime-maintenance/runtime-upgrades/).

## Test Your Configuration Locally

To test XCM between chains, run your parachain with a local relay chain and a second parachain. [Run a Parachain Network](/polkadot-sdk/testing/run-a-parachain-network/) shows how to start such a network with [Zombienet](/tools/zombienet/). Once the APIs from the previous section are in place, use [Estimate XCM Fees](/tools/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/) to dry-run transfers and estimate fees against your local chains.

## Where to Go Next

- [XCM docs](https://paritytech.github.io/polkadot-sdk/master/xcm_docs/index.html): Read the Polkadot SDK reference for XCM, including the executor, the builder types, and worked examples.
- [Estimate XCM Fees](/tools/chain-interactions/send-transactions/interoperability/estimate-xcm-fees/): Use `DryRunApi` and `XcmPaymentApi` from a client.
- [XCM Precompile](/smart-contracts/precompiles/xcm/): Send and execute XCM from smart contracts.
