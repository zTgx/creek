use crate::{
	primitives::{
		assertion::Assertion, cerror::CError, identity::Identity, network::Web3Network,
		signature::validation_data::ValidationData, CResult,
	},
	service::{
		impls::{get_rsa_request, worker_inner::LinkIdentityInner},
		json::{json_req, RpcReturnValue},
		workerclient::SidechainRpcRequest,
	},
	utils::hex::FromHexPrefixed,
	Creek, WorkerGetters, WorkerOp,
};

/// According to this ref: https://github.com/litentry/litentry-parachain/blob/038b0f47e9df6657b7a656126371e46056b5b354/tee-worker/sidechain/rpc-handler/src/direct_top_pool_api.rs
/// Here are the methods:
/// 1. io_handler.add_sync_method("author_submitAndWatchRsaRequest"
/// 2. io_handler.add_sync_method("author_submitAndWatchBroadcastedRsaRequest"
/// 3. io_handler.add_sync_method("author_submitRsaRequest"
/// 4. io_handler.add_method("author_submitVCRequest"
/// 5. io_handler.add_sync_method("author_submitAndWatchAesRequest"
/// 6. io_handler.add_sync_method("author_submitAndWatchBroadcastedAesRequest",
/// 7. io_handler.add_sync_method("author_pendingExtrinsics"
/// 8. io_handler.add_sync_method("author_pendingTrustedCallsFor"
///
/// Currently, I'm using `author_submitAndWatchRsaRequest`, but what is the difference between those
/// methods? Figure it out.
impl WorkerOp for Creek {
	fn link_identity(
		&self,
		link_identity: Identity,
		networks: Vec<Web3Network>,
		vdata: ValidationData,
	) -> CResult<()> {
		let shard = self.author_get_shard()?;
		let shielding_pubkey = self.author_get_shielding_key()?;

		let trusted_call_signed =
			self.link_identity_inner(link_identity, networks, &shard, vdata)?;

		let param = get_rsa_request(shard, trusted_call_signed, shielding_pubkey);
		let jsonreq = json_req("author_submitAndWatchRsaRequest", [param], 1);
		let jsonresp = self.worker_client.request(jsonreq)?;

		let rpc_return_value =
			RpcReturnValue::from_hex(&jsonresp.result).map_err(CError::HexError)?;

		println!("[LINK IDENTITY]: {:#?}", rpc_return_value);

		Ok(())
	}

	fn request_vc(&self, assertion: Assertion) -> CResult<()> {
		let shard = self.author_get_shard()?;
		let shielding_pubkey = self.author_get_shielding_key()?;

		let trusted_call_signed = self.request_vc_inner(&shard, assertion)?;

		let param = get_rsa_request(shard, trusted_call_signed, shielding_pubkey);
		let jsonreq = json_req("author_submitAndWatchRsaRequest", [param], 1);

		let jsonresp = self.worker_client.request(jsonreq)?;
		let rpc_return_value =
			RpcReturnValue::from_hex(&jsonresp.result).map_err(CError::HexError)?;

		println!("[REQUEST VC]: {:#?}", rpc_return_value);

		Ok(())
	}
}
