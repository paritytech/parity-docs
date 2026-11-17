use std::str::FromStr;
use subxt::dynamic::Value;
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
    println!("Querying runtime APIs for: {}\n", ADDRESS);

    // Parse the address
    let account = AccountId32::from_str(ADDRESS)?;

    // Call AccountNonceApi using static interface
    let nonce_call = runtime::runtime_apis()
        .account_nonce_api()
        .account_nonce(account.clone());
    let nonce = at_block.runtime_apis().call(nonce_call).await?;
    println!("AccountNonceApi Results:");
    println!("  Account Nonce: {}", nonce);

    // Call Metadata API to get supported versions using dynamic call
    let metadata_versions_call =
        subxt::dynamic::runtime_api_call::<_, Value>("Metadata", "metadata_versions", ());
    let versions_result = at_block
        .runtime_apis()
        .call(metadata_versions_call)
        .await?;
    println!("\nMetadata API Results:");
    println!(
        "  Supported Metadata Versions: {}",
        versions_result
    );

    Ok(())
}
