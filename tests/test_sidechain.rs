use litentry_test_suit::{sidechain::SidechainRpc, ApiClient};
use sp_core::{sr25519, Pair};

#[test]
fn tc_sidechain_rpc_methods_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let methods = api_client.rpc_methods().unwrap();
    println!("Sidechain supported methods: {:?}", methods);
}
