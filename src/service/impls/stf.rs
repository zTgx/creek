use crate::{
	primitives::{
		identity::Identity,
		network::Web3Network, CResult,
	},
	service::{json::RpcReturnValue, wsclient::DiRequest, impls::stf_inner::LinkIdentityInner},
	utils::hex::FromHexPrefixed,
	Creek, WorkerGetters, WorkerSTF,
};

impl WorkerSTF for Creek {
	fn link_identity(&self, link_identity: Identity, networks: Vec<Web3Network>) -> CResult<()> {
		let shard = self.author_get_shard()?;
		let tee_shielding_key = self.author_get_shielding_key()?;
		let trusted_call_signed = self.link_identity_inner(link_identity, networks, &shard)?;

		let jsonresp = self.client().di_request(shard, tee_shielding_key, trusted_call_signed).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();
		println!("[LINK IDENTITY]: {:#?}", rpc_return_value);

		Ok(())
	}
}
