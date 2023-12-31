use crate::{
	primitives::{
		identity::Identity,
		network::Web3Network,
		signature::validation_data::ValidationData,
		trusted_call::{TrustedCall, TrustedCallSigned},
		CResult, ShardIdentifier,
	},
	utils::{hex::ToHexPrefixed, public_api::mrenclave_to_bs58},
	Creek, WorkerGetters,
};

pub(crate) trait LinkIdentityInner {
	fn link_identity_inner(
		&self,
		link_identity: Identity,
		networks: Vec<Web3Network>,
		shard: &ShardIdentifier,
		vdata: ValidationData,
	) -> CResult<TrustedCallSigned>;
}

impl LinkIdentityInner for Creek {
	fn link_identity_inner(
		&self,
		link_identity: Identity,
		networks: Vec<Web3Network>,
		shard: &ShardIdentifier,
		vdata: ValidationData,
	) -> CResult<TrustedCallSigned> {
		let signer_acccount = self.signer.account_id();
		let primary_identity = Identity::from(signer_acccount.clone());

		let trusted_call = TrustedCall::link_identity(
			primary_identity.clone(),
			primary_identity,
			link_identity,
			vdata,
			networks,
			None,
			Default::default(),
		);

		let mrenclave = self.state_get_mrenclave()?;
		let sidechain_nonce = self.author_get_next_nonce(
			mrenclave_to_bs58(&shard.to_fixed_bytes()),
			signer_acccount.to_hex(),
		)?;
		let signed_call = trusted_call.sign(&self.signer, sidechain_nonce, &mrenclave, shard);
		Ok(signed_call)
	}
}
