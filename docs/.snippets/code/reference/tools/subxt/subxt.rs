use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "./polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the node URL (a local development node by default).
    const NODE_URL: &str = "ws://127.0.0.1:9944";

    // Initialize the Subxt client to interact with the blockchain.
    let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL).await?;

    // Anchor to the current block for all subsequent queries.
    let at_block = api.at_current_block().await?;

    // A query to obtain some constant.
    let constant_query = polkadot::constants().balances().existential_deposit();

    // Obtain the value.
    let value = at_block.constants().entry(constant_query)?;

    println!("Existential deposit: {:?}", value);

    // Define the target account address.
    const ADDRESS: &str = "INSERT_ADDRESS";
    let account = AccountId32::from_str(ADDRESS).unwrap();

    // Build a storage query to access account information.
    let account_storage = at_block
        .storage()
        .entry(polkadot::storage().system().account())?;

    // Fetch the latest state for the account.
    let result = account_storage
        .fetch((account.into(),))
        .await?
        .decode()?;

    println!("Account info: {:?}", result);

    // Use the Bob development account as the recipient.
    let dest = dev::bob().public_key();

    // Define the transfer amount.
    const AMOUNT: u128 = INSERT_AMOUNT;

    // Build the balance transfer extrinsic.
    let balance_transfer_tx = polkadot::transactions()
        .balances()
        .transfer_allow_death(dest.into(), AMOUNT);

    // Sign with the Alice development account. Its secret key is public,
    // so only use it on local development chains.
    let sender_keypair = dev::alice();

    // Sign and submit the extrinsic, then wait for it to be finalized.
    let events = at_block
        .transactions()
        .sign_and_submit_then_watch_default(&balance_transfer_tx, &sender_keypair)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Check for a successful transfer event.
    if let Some(event) = events.find_first::<polkadot::balances::events::Transfer>() {
        println!("Balance transfer successful: {:?}", event);
    }

    Ok(())
}
