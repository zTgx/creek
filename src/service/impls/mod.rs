use codec::Encode;
use rsa::RsaPublicKey;
use sp_core::hexdisplay::AsBytesRef;

use crate::{
	primitives::{
		aes::{aes_encrypt_default, RequestAesKey},
		aes_request::AesRequest,
		rsa_request::RsaRequest,
		trusted_call::TrustedCallSigned,
		CResult, Index, ShardIdentifier,
	},
	service::getter_trait::WorkerGetters,
	utils::{
		crypto::encrypt_with_tee_shielding_pubkey, hex::ToHexPrefixed,
		public_api::mrenclave_to_bs58,
	},
	Creek,
};

pub mod getter;
pub mod parachain;
pub mod vdata;
pub mod worker;
pub mod worker_inner;

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

pub(crate) fn get_rsa_request(
	shard: ShardIdentifier,
	trusted_call_signed: TrustedCallSigned,
	shielding_pubkey: RsaPublicKey,
) -> String {
	let operation_call_encrypted = encrypt_with_tee_shielding_pubkey(
		&shielding_pubkey,
		&trusted_call_signed.into_trusted_operation(true).encode(),
	);

	let request = RsaRequest::new(shard, operation_call_encrypted);
	request.to_hex()
}

pub(crate) fn get_aes_request(
	shard: ShardIdentifier,
	trusted_call_signed: TrustedCallSigned,
	shielding_pubkey: RsaPublicKey,
) -> String {
	// let operation_call_encrypted = encrypt_with_tee_shielding_pubkey(
	// 	&shielding_pubkey,
	// 	&trusted_call_signed.into_trusted_operation(true).encode(),
	// );

	let key: RequestAesKey = RequestAesKey::default();

	let encrypted_key = encrypt_with_tee_shielding_pubkey(&shielding_pubkey, key.as_bytes_ref());

	let operation_call_encrypted =
		aes_encrypt_default(&key, &trusted_call_signed.into_trusted_operation(true).encode());

	let request = AesRequest { shard, key: encrypted_key, payload: operation_call_encrypted };

	request.to_hex()
}
