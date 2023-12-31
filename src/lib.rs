#![recursion_limit = "256"]
#![feature(string_remove_matches)]
#![allow(clippy::large_enum_variant)] //StfError: The `Err`-variant returned from this function is very large
#![allow(clippy::result_large_err)]

pub mod primitives;
pub mod service;
pub mod utils;

use frame_metadata::RuntimeMetadataPrefixed;
use primitives::{
	identity::Identity, AccountId, CResult, Ed25519Pubkey, EnclaveShieldingPubKey, Index,
		keypair::KeyPair,
		MrEnclave, ShardIdentifier, network::Web3Network, 
};
use service::wsclient::SidechainRpcClient;

#[derive(Clone)]
pub struct Creek {
	client: SidechainRpcClient,
	pub signer: KeyPair,
}

impl Creek {
	pub fn new_with_signer(signer: KeyPair) -> Self {
		let url: &str = "wss://localhost:2600";
		let client = SidechainRpcClient::new(url);

		Self { client, signer }
	}

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
/// A set of transaction interfaces that can change the sidechain state, including link identity,
/// request VC, etc
pub trait WorkerSTF {
	/// link identity steps:
	/// * link_identity: The `Identity` you want to be linked.
	/// * networks: The `Identity` supported network. (For Web2 Identity, networks MUST BE ved![])
	fn link_identity(&self, link_identity: Identity, networks: Vec<Web3Network>) -> CResult<()>;
}
