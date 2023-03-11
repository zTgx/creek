mod index;
mod lit_vc;

use substrate_api_client::{rpc::WsRpcClient, Api, AssetTipExtrinsicParams, Metadata};
use std::env;
use sp_core::sr25519;

const NODE_SERVER_URL: &str = "NODE_SERVER_URL";
const NODE_PORT: &str = "NODE_PORT";
const DEFAULT_NODE_SERVER_URL: &str = "ws://127.0.0.1";
const DEFAULT_NODE_PORT: &str = "9944";

pub fn print_metadata() {
    let url = get_node_url();
	let client = WsRpcClient::new(&url);
	let api = Api::<sr25519::Pair, _, AssetTipExtrinsicParams>::new(client).unwrap();

	let meta = Metadata::try_from(api.get_metadata().unwrap()).unwrap();
	meta.print_overview();
}

pub fn get_node_url() -> String {
    let node_server = env::var(NODE_SERVER_URL).unwrap_or(DEFAULT_NODE_SERVER_URL.to_string());
    let node_port = env::var(NODE_PORT).unwrap_or(DEFAULT_NODE_PORT.to_string());
	let url = format!("{}:{}", node_server, node_port);
	println!("Interacting with node on {}\n", url);
	url
}