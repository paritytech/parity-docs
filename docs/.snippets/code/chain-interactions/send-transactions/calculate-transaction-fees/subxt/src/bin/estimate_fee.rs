use subxt::config::DefaultExtrinsicParamsBuilder;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

// Generate an interface from the node's metadata
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod runtime {}

// WebSocket endpoint of your local development node
const NODE_URL: &str = "ws://127.0.0.1:9944";

// Amount to transfer, in the chain's smallest unit
const AMOUNT: u128 = 1_000_000_000_000;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Subxt client and anchor to the current block
    let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL).await?;
    let at_block = api.at_current_block().await?;

    // Use the Alice and Bob development accounts
    let alice = dev::alice();
    let bob = AccountId32::from(dev::bob().public_key());

    // Build the balance transfer call
    let tx = runtime::transactions()
        .balances()
        .transfer_keep_alive(bob.into(), AMOUNT);

    // Sign the transaction without submitting it, then ask the node for a fee estimate
    let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new().build();
    let fee = at_block
        .transactions()
        .create_signed(&tx, &alice, params)
        .await?
        .partial_fee_estimate()
        .await?;

    println!("Estimated fee: {} (smallest unit)", fee);

    Ok(())
}
