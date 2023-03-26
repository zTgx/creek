use litentry_test_suit::{sidechain::SidechainRpc, ApiClient};
use sp_core::{sr25519, Pair};

#[test]
fn tc_sidechain_rpc_methods_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let methods = api_client.rpc_methods().unwrap();
    println!("Sidechain supported methods: {:?}", methods);
}

#[test]
fn tc_sidechain_system_version_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let system_version = api_client.system_version().unwrap();
    println!("Sidechain system_version: {}", system_version);
}

#[test]
fn tc_sidechain_system_name_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let system_name = api_client.system_name().unwrap();
    println!("Sidechain system_name: {}", system_name);
}

#[test]
fn tc_sidechain_system_health_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let system_health = api_client.system_health().unwrap();
    println!("Sidechain system_health: {}", system_health);
}

#[test]
fn tc_sidechain_state_get_runtime_version_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let runtime_version = api_client.state_get_runtime_version().unwrap();
    println!("Sidechain runtime_version: {}", runtime_version);
}

#[test]
fn tc_sidechain_state_get_metadata_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let metadata = api_client.state_get_metadata().unwrap();
    println!("Sidechain metadata: {:?}", metadata);
}