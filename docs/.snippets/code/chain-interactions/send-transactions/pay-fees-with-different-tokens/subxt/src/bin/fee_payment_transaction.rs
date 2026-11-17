use subxt::config::{Config, DefaultExtrinsicParamsBuilder, DefaultTransactionExtensions, PolkadotConfig};
use subxt::utils::AccountId32;
use subxt::{OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::dev;

// Generate types from the chain metadata with Location trait derives
#[subxt::subxt(
    runtime_metadata_path = "metadata/runtime.scale",
    derive_for_type(
        path = "staging_xcm::v5::location::Location",
        derive = "Clone, Eq, PartialEq, codec::Encode",
        recursive
    )
)]
pub mod runtime {}

// Import XCM location types from the generated metadata module
use runtime::runtime_types::staging_xcm::v5::{
    junction::Junction,
    junctions::Junctions,
    location::Location,
};

// Define a custom config where AssetId is an XCM Location
#[derive(Debug, Default, Clone)]
pub struct AssetPaymentConfig;

impl Config for AssetPaymentConfig {
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type TransactionExtensions = DefaultTransactionExtensions<AssetPaymentConfig>;
    type AssetId = Location;
}

// WebSocket endpoint of the node (the Chopsticks fork in this tutorial)
const NODE_URL: &str = "ws://localhost:8000";
// Amount to transfer, in the chain's smallest unit
const TRANSFER_AMOUNT: u128 = 3_000_000_000;
// Index of the Assets pallet in the runtime
const ASSETS_PALLET_INDEX: u8 = 50;
// ID of the asset used to pay fees (an asset Alice holds in the Chopsticks config)
const FEE_ASSET_ID: u128 = 1984;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the node
    let api = OnlineClient::<AssetPaymentConfig>::from_url(NODE_URL).await?;
    println!("Connected to {}", NODE_URL);

    // Anchor to the current block
    let at_block = api.at_current_block().await?;

    // Create Alice's dev keypair
    let alice = dev::alice();
    println!("Sender (Alice): {}", AccountId32::from(alice.public_key()));

    // Create the balance transfer transaction to Bob
    let dest = AccountId32::from(dev::bob().public_key());
    let tx = runtime::transactions()
        .balances()
        .transfer_keep_alive(dest.into(), TRANSFER_AMOUNT);

    // Define the fee asset location in XCM format
    let asset_location = Location {
        parents: 0,
        interior: Junctions::X2([
            Junction::PalletInstance(ASSETS_PALLET_INDEX),
            Junction::GeneralIndex(FEE_ASSET_ID),
        ]),
    };

    // Build transaction params to pay fees with the alternative asset
    let tx_params = DefaultExtrinsicParamsBuilder::<AssetPaymentConfig>::new()
        .tip_of(0, asset_location)
        .build();

    // Sign, submit, and watch for finalization
    println!("Signing and submitting transaction...");
    let progress = at_block
        .transactions()
        .sign_and_submit_then_watch(&tx, &alice, tx_params)
        .await?;

    let in_block = progress.wait_for_finalized().await?;
    let block_hash = in_block.block_hash();
    let events = in_block.wait_for_success().await?;

    // Display transaction results
    println!("\nTransaction finalized in block: {:?}", block_hash);

    println!("\nEvents:");
    for event in events.iter() {
        let event = event?;
        println!(
            "  {}.{}",
            event.pallet_name(),
            event.event_name()
        );
    }

    Ok(())
}
