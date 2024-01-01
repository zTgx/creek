use crate::{
	primitives::{CResult, Index},
	service::getter_trait::WorkerGetters,
	utils::{hex::ToHexPrefixed, public_api::mrenclave_to_bs58},
	Creek,
};

pub trait CreekHelper {
	fn get_sidechain_nonce(&self) -> CResult<Index>;
}

impl CreekHelper for Creek {
	fn get_sidechain_nonce(&self) -> CResult<Index> {
		let shard = self.author_get_shard()?;
		let signer_acccount = self.signer.account_id();

		self.author_get_next_nonce(
			mrenclave_to_bs58(&shard.to_fixed_bytes()),
			signer_acccount.to_hex(),
		)
	}
}
