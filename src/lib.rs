#![recursion_limit = "256"]
#![feature(string_remove_matches)]

// pub mod api_client_patch;
// pub mod direct_call;
// pub mod primitives;

// pub mod sidechain;
// pub mod utils;

// pub mod vc_management;
// pub mod identity_management;

// #[cfg(target_arch = "x86_64")]
// pub mod ra;
use codec::{Decode, Encode, Error as CodecError};

use openssl::ssl::{SslStream, SslConnector, SslMethod, SslVerifyMode};
use rpc::{SidechainRpcClient, SidechainRpcClientTrait, RpcReturnValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sp_core::{sr25519, Pair, H256, OpaqueMetadata};
// use sidechain::rpc::SidechainRpcClient;
use substrate_api_client::{
	ac_primitives::{AssetRuntimeConfig, Bytes, RpcParams},
	rpc::{Request, TungsteniteRpcClient},
	Api, Error as ApiError, ExtrinsicReport, Result as ApiResult, SubmitAndWatch, XtStatus, ac_compose_macros::rpc_params,
};
use ws::{connect, CloseCode, util::TcpStream};

use crate::hex::FromHexPrefixed;
mod rpc;
mod hex;

pub struct Creek {
	pub api: Api<AssetRuntimeConfig, TungsteniteRpcClient>,
}

struct Client {
    out: ws::Sender,
}

impl ws::Handler for Client {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("msg = {}", msg);
        self.out.close(ws::CloseCode::Normal)
    }

    fn upgrade_ssl_client(
        &mut self,
        sock: TcpStream,
        _: &url::Url,
    ) -> ws::Result<SslStream<TcpStream>> {
        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| {
            ws::Error::new(
                ws::ErrorKind::Internal,
                format!("Failed to upgrade client to SSL: {}", e),
            )
        })?;
        builder.set_verify(SslVerifyMode::empty());

        let connector = builder.build();
        connector
            .configure()
            .unwrap()
            .use_server_name_indication(false)
            .verify_hostname(false)
            .connect("", sock)
            .map_err(From::from)
    }
}



impl Creek {
	pub fn x() {
		let url: &str = "wss://localhost:2600";

		let c = SidechainRpcClient::new(url);
		let jsonreq = json_req("author_getShard", [0_u8; 0], 1);
		let resp = c.request(jsonreq).unwrap();
		println!(">> resp1: {:?}", resp);

		let mut resp = json_resp(resp).unwrap();
		println!(">> resp: {:?}", resp);

		let mut x = RpcReturnValue::from_hex(&resp.result).unwrap();


		let resp = H256::decode(&mut x.value.as_slice()).unwrap();
			println!(">> MRenclave: {:?}", resp);

	}
	pub fn new() -> ApiResult<Self> {
		let url: &str = "wss://localhost:2600";
		// let url = "wss://tee-internal.litentry.io:443";
		let client = TungsteniteRpcClient::new(url, 100)?;
		println!(">>1");
		let mut api = Api::<AssetRuntimeConfig, TungsteniteRpcClient>::new(client)?;
		println!(">>2");

		let alice = sr25519::Pair::from_string("//Alice", None)
			.map_err(|e| ApiError::Other(format!("Generate KeyPair Failed: {:?}", e).into()))?;
		api.set_signer(alice.into());

		println!(">>3");

		Ok(Creek { api })
	}

	pub fn send_extrinsic(&self, xthex_prefixed: &Bytes) -> ApiResult<ExtrinsicReport<H256>> {
		self.api
			.submit_and_watch_opaque_extrinsic_until(xthex_prefixed, XtStatus::InBlock)
	}

	pub fn metatdata(&self) {
		let metadata = self.api.metadata();
		println!(">>> Sidechain Metadata: {:?}", metadata);
	}
}

pub trait WorkerPublicApi {
	fn system_name(&self);
	fn system_version(&self);
	fn get_metadata(&self);
}

impl WorkerPublicApi for Creek {
	fn get_metadata(&self) {
	// 	const METHOD_NAME: &str = "state_getMetadata";
	// 	let client = self.api.client();
	// 	let x: OpaqueMetadata = client.request(METHOD_NAME, RpcParams::new()).unwrap();
	// 	println!("x: {}", x);

	// 	let m = 

		
	// let metadata = match metadata.1 {
	// 	RuntimeMetadata::V14(meta) => meta,
	// 	_ => panic!("Invalid metadata"),
	// };
	let url: &str = "wss://localhost:2600";

	if let Err(error) = connect(url, |out| {

		let jsonreq = json_req("system_name", [0_u8; 0], 1);

        // Queue a message to be sent when the WebSocket is open
        if out.send(jsonreq.to_string()).is_err() {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }

        // The handler needs to take ownership of out, so we use move
        move |msg| {
            // Handle messages received on this connection
            println!("Client got message '{}'. ", msg);

            // Close the connection
            out.close(CloseCode::Normal)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to: {:?}", error);
    }


	}

	fn system_version(&self) {
		const SYSTEM_NAME: &str = "system_version";
		let client = self.api.client();
		let x: String = client.request(SYSTEM_NAME, rpc_params![]).unwrap();
		println!("x: {}", x);
	}

	fn system_name(&self) {
		const SYSTEM_NAME: &str = "system_name";
		// let jsonreq = json_req("state_getMetadata", [0_u8; 0], 1);
		let client = self.api.client();

		let params = RpcParams::new();
		// params.insert(0).unwrap();
		// let built_params = params.build().unwrap();

		let x: String = client.request(SYSTEM_NAME, params).unwrap();
		println!("x: {}", x);

		// let rpc_response = json_resp(resp)?;
		// let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
		// 	.map_err(|e| ApiError::Other(format!("{:?}", e)))?;

		// Ok(RuntimeMetadataPrefixed::decode(&mut rpc_return_value.value.as_slice()).map_err(
		// 	|_| {
		// 		let error = CodecError::from("Decode RuntimeMetadataPrefixed error");
		// 		ApiError::DecodeValue(DecodeError::CodecError(error))
		// 	},
		// )?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SidechainResp {
	pub id: String,
	pub jsonrpc: String,
	pub result: String,
}

#[derive(Clone, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct RpcResponse {
	pub jsonrpc: String,
	pub result: String, // hex encoded RpcReturnValue
	pub id: u32,
}

pub fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
	json!({
		"method": method,
		"params": params,
		"jsonrpc": "2.0",
		"id": id.to_string(),
	})
}

pub fn json_resp(resp: String) -> ApiResult<SidechainResp> {

	// let x = hex::decode("800fce66daccefbc3f443c9c1fa7fd8596b2133f19fbef7093f005d4eb3e61c2f80000").unwrap();
	// let x = RpcReturnValue::decode(&mut x.as_slice()).unwrap();
	// println!(">>> x: {:?}", x);

	let resp: SidechainResp = serde_json::from_str(&resp).unwrap();
	Ok(resp)
}
