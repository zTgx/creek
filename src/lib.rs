#![recursion_limit = "256"]
#![feature(string_remove_matches)]
#![allow(clippy::large_enum_variant)] //StfError: The `Err`-variant returned from this function is very large
#![allow(clippy::result_large_err)]

pub mod primitives;
pub mod service;
pub mod utils;

use crate::primitives::Ed25519Public;
use primitives::{
	address::Address32, assertion::Assertion, enclave::Enclave, identity::Identity,
	keypair::KeyPair, network::Web3Network, signature::validation_data::ValidationData, AccountId,
	CResult, MrEnclave,
};
use rsa::RsaPublicKey;
use service::{
	getter_trait::WorkerGetters, parachainclient::ParachainRpcClient, workerclient::DirectClient,
};
use std::collections::HashMap;

pub struct Creek {
	pub parachain_client: ParachainRpcClient,
	pub worker_client: DirectClient,
	pub signer: KeyPair,
}

/// Where the magic begins
pub trait CreekExplorer {
	fn explorer(parachain_endpoint: &str, worker_endpoint: &str, signer: KeyPair)
		-> CResult<Creek>;
}

/// For Web3 Identity:
/// * Using the to-being-linked account keypair to sign a validation data to aprove that this linked
///   account is owned by you.
/// * Then call link_identity from `WorkerOp` trait to enact link operation.
pub trait ValidationDataBuilder {
	fn web3_vdata(&self, keypair: &KeyPair) -> CResult<ValidationData>;
	fn twitter_vdata(&self, twitterid: &str) -> CResult<ValidationData>;
}

/// Worker operations traits
pub trait WorkerOp {
	/// link identity steps:
	/// * before link identity make sure you sign the vdata
	/// * link_identity: The `Identity` you want to be linked.
	/// * networks: The `Identity` supported network. (For Web2 Identity, networks MUST BE ved![])
	fn link_identity(
		&self,
		link_identity: Identity,
		networks: Vec<Web3Network>,
		vdata: ValidationData,
	) -> CResult<()>;

	/// request vc(verified credential)
	/// * Select the VC you want to generate from `Assertion` type
	fn request_vc(&self, assertion: Assertion) -> CResult<()>;
}

/// Parachain Operation traits
pub trait ParachainOp {
	fn delegatee(&self, account: Address32) -> CResult<Option<()>>;
	fn enclave_count(&self) -> CResult<Option<u64>>;
	fn enclave(&self, enclave_count: u64) -> CResult<Option<Enclave<AccountId, String>>>;
	fn get_shard(&self) -> CResult<MrEnclave>;
	fn get_tee_shielding_pubkey(&self) -> CResult<RsaPublicKey>;
	fn get_vc_pubkey(&self) -> CResult<Ed25519Public>;

	/// Due to different code versions, there are problems when parsing VCContext directly, so it is
	/// stored in the form of HashMap. The encoding format of the key remains unchanged,
	/// while the value performs hex encoding on the obtained Vec<u8> data, which is compatible with
	/// data parsing in different versions.
	fn vc_registry(&self) -> CResult<HashMap<String, String>>;
}
