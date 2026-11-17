use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

// Generate an interface from the node's metadata
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod runtime {}

// WebSocket endpoint of your local development node
const NODE_URL: &str = "ws://127.0.0.1:9944";

// Address to query (Alice's development account by default)
const ADDRESS: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Subxt client
    let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL).await?;

    // Anchor to the current block
    let at_block = api.at_current_block().await?;

    println!("Connected to {}", NODE_URL);
    println!("Querying account: {}\n", ADDRESS);

    // Parse the address
    let account = AccountId32::from_str(ADDRESS)?;

    // Build storage entry for System.Account
    let account_storage = at_block
        .storage()
        .entry(runtime::storage().system().account())?;

    // Fetch the account information
    let info = account_storage
        .fetch((account,))
        .await?
        .decode()?;

    println!("Account Information:");
    println!("  Nonce: {}", info.nonce);
    println!("  Consumers: {}", info.consumers);
    println!("  Providers: {}", info.providers);
    println!("  Sufficients: {}", info.sufficients);

    println!("\nBalance Details:");
    println!("  Free: {}", info.data.free);
    println!("  Reserved: {}", info.data.reserved);
    println!("  Frozen: {}", info.data.frozen);
    println!("  Total (free + reserved): {}", info.data.free + info.data.reserved);

    Ok(())
}
