use crate::{Creek, primitives::{identity::Identity, signature::validation_data::{ValidationString, ValidationData}, trusted_call::{TrustedCall, TrustedCallSigned}, ShardIdentifier, CResult, cerror::CError, network::Web3Network}, WorkerGetters, utils::{public_api::mrenclave_to_bs58, identity::ValidationDataBuilder, hex::ToHexPrefixed}};

pub(crate) trait LinkIdentityInner {
    fn link_identity_inner(&self, link_identity: Identity, networks: Vec<Web3Network>, shard: &ShardIdentifier) -> CResult<TrustedCallSigned>;
}

impl LinkIdentityInner for Creek {
    fn link_identity_inner(&self, link_identity: Identity, networks: Vec<Web3Network>, shard: &ShardIdentifier) -> CResult<TrustedCallSigned> {
        let signer_acccount = self.signer.account_id();
		let sidechain_nonce = self
			.author_get_next_nonce(
				mrenclave_to_bs58(&shard.to_fixed_bytes()),
				signer_acccount.to_hex(),
			)?;

		let message =
			ValidationString::try_from(sidechain_nonce.to_string().as_bytes().to_vec()).map_err(|_e| {
                CError::Other("Parse sidechain nonce error".to_string())
            })?;

		let vdata = ValidationData::build_vdata_twitter(&message).map_err(|_e| {
            CError::APIError
        })?;

		let mrenclave = self.state_get_mrenclave()?;

        let primary_identity = Identity::from(signer_acccount);

		let trusted_call = TrustedCall::link_identity(
			primary_identity.clone(),
			primary_identity,
			link_identity,
			vdata,
			networks,
			None,
			Default::default(),
		);
		println!(">>> trusted_call: {:?}", trusted_call);

        let signed_call = trusted_call.sign(&self.signer, sidechain_nonce, &mrenclave, &shard);
        Ok(signed_call)
    }
}