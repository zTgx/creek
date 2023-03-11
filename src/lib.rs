mod index;
mod lit_vc;

#[macro_use]
extern crate lazy_static;

use sp_core::sr25519;
use std::env;
use substrate_api_client::{rpc::WsRpcClient, Api, AssetTipExtrinsicParams, Metadata};

const NODE_SERVER_URL: &str = "NODE_SERVER_URL";
const NODE_PORT: &str = "NODE_PORT";
const DEFAULT_NODE_SERVER_URL: &str = "ws://127.0.0.1";
const DEFAULT_NODE_PORT: &str = "9944";

lazy_static! {
    static ref API: Api::<sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams> = {
        let node_server = env::var(NODE_SERVER_URL).unwrap_or(DEFAULT_NODE_SERVER_URL.to_string());
        let node_port = env::var(NODE_PORT).unwrap_or(DEFAULT_NODE_PORT.to_string());
        let url = format!("{}:{}", node_server, node_port);
        let client = WsRpcClient::new(&url);

        Api::<sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>::new(client).unwrap()
    };
}

pub fn print_metadata() {
    let meta = Metadata::try_from(API.get_metadata().unwrap()).unwrap();
    meta.print_overview();
}
