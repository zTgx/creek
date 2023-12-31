#![recursion_limit = "256"]
#![feature(string_remove_matches)]
#![allow(clippy::large_enum_variant)] //StfError: The `Err`-variant returned from this function is very large
#![allow(clippy::result_large_err)]

pub mod primitives;
pub mod service;
pub mod utils;

use frame_metadata::RuntimeMetadataPrefixed;
use primitives::{
	identity::Identity, keypair::KeyPair, network::Web3Network,
	signature::validation_data::ValidationData, AccountId, CResult, Ed25519Pubkey,
	EnclaveShieldingPubKey, Index, MrEnclave, ShardIdentifier,
};
use service::wsclient::SidechainRpcClient;
use utils::{hex::ToHexPrefixed, public_api::mrenclave_to_bs58};

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

	pub fn get_sidechain_nonce(&self) -> CResult<Index> {
		let shard = self.author_get_shard()?;
		let signer_acccount = self.signer.account_id();

		self.author_get_next_nonce(
			mrenclave_to_bs58(&shard.to_fixed_bytes()),
			signer_acccount.to_hex(),
		)
	}
}

/// Before link identity:
/// For Web3 Identity:
/// 1. Using the linked keypair to sign a validation data to aprove that this linked account is
/// owned by you. 2. Then call link_identity from `WorkerSTF` trait to enact link operation.
pub trait ValidationDataBuilder {
	fn twitter_vdata(&self, twitterid: &str) -> CResult<ValidationData>;
	fn web3_vdata(&self, keypair: &KeyPair) -> CResult<ValidationData>;
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
