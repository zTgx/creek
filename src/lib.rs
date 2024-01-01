#![recursion_limit = "256"]
#![feature(string_remove_matches)]
#![allow(clippy::large_enum_variant)] //StfError: The `Err`-variant returned from this function is very large
#![allow(clippy::result_large_err)]

pub mod helper;
pub mod primitives;
pub mod service;
pub mod utils;

use primitives::{
	identity::Identity, keypair::KeyPair, network::Web3Network,
	signature::validation_data::ValidationData, CResult,
};
use service::{getter_trait::WorkerGetters, wsclient::SidechainRpcClient};

#[derive(Clone)]
pub struct Creek {
	pub client: SidechainRpcClient,
	pub signer: KeyPair,
}

impl Creek {
	pub fn new_with_signer(signer: KeyPair) -> Self {
		let url: &str = "wss://localhost:2600";
		let client = SidechainRpcClient::new(url);

		Self { client, signer }
	}
}

/// Worker State Transfer Function
/// A set of transaction interfaces that can change the sidechain state, including link identity,
/// request VC, etc
pub trait WorkerSTF {
	/// link identity steps:
	/// * link_identity: The `Identity` you want to be linked.
	/// * networks: The `Identity` supported network. (For Web2 Identity, networks MUST BE ved![])
	fn link_identity(
		&self,
		link_identity: Identity,
		networks: Vec<Web3Network>,
		vdata: ValidationData,
	) -> CResult<()>;
}

/// Before link identity:
/// For Web3 Identity:
/// 1. Using the linked keypair to sign a validation data to aprove that this linked account is
/// owned by you. 2. Then call link_identity from `WorkerSTF` trait to enact link operation.
pub trait ValidationDataBuilder {
	fn twitter_vdata(&self, twitterid: &str) -> CResult<ValidationData>;
	fn web3_vdata(&self, keypair: &KeyPair) -> CResult<ValidationData>;
}
