use litentry_test_suit::{ApiClient, sidechain::EnclaveRpc};
use serde::Serialize;
use serde_json::{Value, json};
use sp_core::{sr25519, Pair};


fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
	json!({
		"method": method,
		"params": params,
		"jsonrpc": "2.0",
		"id": id.to_string(),
	})
}

#[test]
fn tc_sidechain_rpc_methods_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::with_worker(alice);

    // api_client.rpc_methods();

	let jsonreq: Value  = json_req("rpc_methods", "", 1);

    let x = api_client.api.get_request(jsonreq);
    println!("x: {:?}", x);
}