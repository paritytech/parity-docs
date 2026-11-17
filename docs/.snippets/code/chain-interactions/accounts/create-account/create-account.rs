use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::{sr25519, Pair};

// SS58 address prefix of your target chain (42 is the generic Substrate prefix)
const SS58_PREFIX: u16 = 42;

fn main() {
    let (pair, phrase, _) = sr25519::Pair::generate_with_phrase(None);
    let address = pair
        .public()
        .to_ss58check_with_version(Ss58AddressFormat::custom(SS58_PREFIX));

    println!("Address: {}", address);
    println!("Mnemonic: {}", phrase);
}
