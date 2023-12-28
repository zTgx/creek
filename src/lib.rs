#![recursion_limit = "256"]
#![feature(string_remove_matches)]

// #[cfg(target_arch = "x86_64")]
// pub mod ra;
pub mod client;
pub mod utils;
pub mod primitives;

use client::service::SidechainRpcClient;
use primitives::{ShardIdentifier, EnclaveShieldingPubKey, CResult};

pub struct Creek {
	client: SidechainRpcClient,
}

impl Creek {
	pub fn new() -> Self {
		let url: &str = "wss://localhost:2600";
		let client = SidechainRpcClient::new(url);

		Self {
			client
		}
	}

	/// Get the rpc client.
	pub fn client(&self) -> &SidechainRpcClient {
		&self.client
	}
}

pub trait WorkerHeartBeat {
	fn tick(&self) {
		// if cuurnt list is not match rpc_methods perfactly.
		// warning: we should update client code align with worker server.
	}
}

pub trait WorkerPublicApis {
	fn rpc_methods(&self);
	// fn system_version(&self);
	// fn system_name(&self);
	// fn system_health(&self);

	// fn state_get_mrenclave(&self);
	// fn state_execute_getter(&self);
	// fn state_get_runtime_version(&self);
	// fn state_get_metadata(&self);
	// fn state_get_storage(&self, mrenclave_in_base58: String, storage_key_in_hex: String);

	// fn author_get_untrusted_url(&self);
	// fn author_get_mu_ra_url(&self);
	fn author_get_shard(&self) -> CResult<ShardIdentifier>;
	// fn author_get_shard_vault(&self);
	// fn author_get_next_nonce(&self);
	// fn author_get_enclave_signer_account(&self);
	fn author_get_shielding_key(&self) -> CResult<EnclaveShieldingPubKey>;

	// fn attesteer_forward_ias_attestation_report(&self);
	// fn attesteer_forward_dcap_quote(&self);
	
	// fn chain_subscribe_all_heads(&self);
}