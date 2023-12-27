#![recursion_limit = "256"]
#![feature(string_remove_matches)]

pub mod api_client_patch;
pub mod direct_call;
pub mod identity_management;
pub mod primitives;

#[cfg(target_arch = "x86_64")]
pub mod ra;

pub mod sidechain;
pub mod utils;
pub mod vc_management;

use sidechain::rpc::SidechainRpcClient;
use sp_core::{crypto::AccountId32 as AccountId, Pair};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
	rpc::{JsonrpseeClient, WsRpcClient},
	Api, ApiResult, PlainTipExtrinsicParams, XtStatus,
};

pub type ApiType<P> = Api<P, WsRpcClient, PlainTipExtrinsicParams>;

#[derive(Clone)]
pub struct ApiClient<P>
where
	P: Pair,
{
	pub api: ApiType<P>,
	pub sidechain: SidechainRpcClient,
}

impl<P> ApiClient<P>
where
	P: Pair,
	MultiSignature: From<P::Signature>,
	MultiSigner: From<P::Public>,
{
	pub fn new() {
		let client = JsonrpseeClient::with_default_url().unwrap();
		let signer = AccountKeyring::Alice.pair();
		let mut api = Api::<AssetRuntimeConfig, _>::new(client).unwrap();
		api.set_signer(signer.clone().into());
	}

	pub fn send_extrinsic(&self, xthex_prefixed: String) {
		// match self.api.submit_and_watch_extrinsic_until(xthex_prefixed, XtStatus::InBlock) {
		// 	Ok(tx_hash) => match tx_hash {
		// 		Some(tx_hash) => {
		// 			println!(" ✅ Transaction got included. Hash: {:?}", tx_hash);
		// 		},
		// 		None => {
		// 			println!(" ❌ Transaction None");
		// 		},
		// 	},
		// 	Err(e) => {
		// 		println!(" ❌ Transaction error : {:?}", e);
		// 	},
		// }

        todo!()
	}
}
