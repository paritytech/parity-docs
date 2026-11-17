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
    // Initialize the Subxt client
    let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL).await?;

    // Anchor to the current block
    let at_block = api.at_current_block().await?;

    println!("Connected to {}", NODE_URL);

    // Use the Alice and Bob development accounts
    let sender_keypair = dev::alice();
    let sender_address = AccountId32::from(sender_keypair.public_key());
    let dest = AccountId32::from(dev::bob().public_key());

    println!("Sender address: {}", sender_address);
    println!("Recipient address: {}", dest);
    println!("Amount: {}\n", AMOUNT);

    // Get sender's account info to check balance
    let account_storage = at_block
        .storage()
        .entry(runtime::storage().system().account())?;
    let account_info = account_storage
        .fetch((sender_address,))
        .await?
        .decode()?;

    println!("Sender balance: {}", account_info.data.free);

    // Build the balance transfer extrinsic
    let balance_transfer_tx = runtime::transactions()
        .balances()
        .transfer_keep_alive(dest.into(), AMOUNT);

    // Sign and submit the extrinsic, then wait for it to be finalized
    println!("\nSigning and submitting transaction...");
    let events = at_block
        .transactions()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &sender_keypair)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Check for a successful transfer event
    if let Some(event) = events.find_first::<runtime::balances::events::Transfer>() {
        println!("\nTransaction successful!");
        println!("Transfer event: {:?}", event);
    }

    Ok(())
}
