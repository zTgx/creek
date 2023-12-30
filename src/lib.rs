#![recursion_limit = "256"]
#![feature(string_remove_matches)]
#![allow(clippy::large_enum_variant)] //StfError: The `Err`-variant returned from this function is very large
#![allow(clippy::result_large_err)]

pub mod core;
pub mod primitives;
pub mod service;
pub mod utils;

use frame_metadata::RuntimeMetadataPrefixed;
use primitives::{
	AccountId, CResult, Ed25519Pubkey, EnclaveShieldingPubKey, Index, MrEnclave, ShardIdentifier,
};
use service::wsclient::SidechainRpcClient;

#[derive(Debug, Clone, Default)]
pub struct Creek {
	client: SidechainRpcClient,
}

impl Creek {
	pub fn new() -> Self {
		let url: &str = "wss://localhost:2600";
		let client = SidechainRpcClient::new(url);

		Self { client }
	}

	/// Get the rpc client.
	pub fn client(&self) -> &SidechainRpcClient {
		&self.client
	}
}

/// Worker Getter Function
/// Used to obtain a collection of information interfaces on sidechain, 
/// including shard, nonce, shieldingkey, etc., for WorkerTx transaction interfaces
pub trait WorkerGetters {
	fn rpc_methods(&self) -> CResult<Vec<String>>;
	fn system_version(&self) -> CResult<String>;
	fn system_name(&self) -> CResult<String>;
	fn system_health(&self) -> CResult<String>;

	fn state_get_mrenclave(&self) -> CResult<MrEnclave>;
	// fn state_execute_getter(&self);
	fn state_get_runtime_version(&self) -> CResult<String>;
	fn state_get_metadata(&self) -> CResult<RuntimeMetadataPrefixed>;
	fn state_get_storage(
		&self,
		mrenclave_in_base58: String,
		storage_key_in_hex: String,
	) -> CResult<Vec<u8>>;

	fn author_get_untrusted_url(&self) -> CResult<String>;
	fn author_get_mu_ra_url(&self) -> CResult<String>;
	fn author_get_shard(&self) -> CResult<ShardIdentifier>;
	fn author_get_shard_vault(&self) -> CResult<AccountId>;
	fn author_get_next_nonce(
		&self,
		shard_in_base58: String,
		account_in_hex: String,
	) -> CResult<Index>;
	fn author_get_enclave_signer_account(&self) -> CResult<Ed25519Pubkey>;
	fn author_get_shielding_key(&self) -> CResult<EnclaveShieldingPubKey>;

	// fn attesteer_forward_ias_attestation_report(&self);
	// fn attesteer_forward_dcap_quote(&self);

	// fn chain_subscribe_all_heads(&self);
}

/// Worker State Transfer Function
/// A set of transaction interfaces that can change the sidechain state, including link identity, request VC, etc
pub trait WorkerSTF {
	fn link_identity(&self);
	fn link_web2(&self);
}
