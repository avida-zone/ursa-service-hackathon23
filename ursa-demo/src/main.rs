use std::fs;
use ursa_demo::{create_sub_proof_request, issuer_set_up};

fn main() {
    env_logger::init();

    // Issuers set up schema and credential def
    // - Issuer qualified for DApp
    let issuers_str = fs::read_to_string("./setup_data/issuers.json").unwrap();
    let issuers: Vec<String> = serde_json::from_str(&issuers_str).unwrap();

    for issuer in issuers {
        _ = issuer_set_up(&issuer);
        create_sub_proof_request(&issuer)
    }
}
