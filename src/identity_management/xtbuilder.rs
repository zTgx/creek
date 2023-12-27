use crate::{
	api_client_patch::parachain::ParachainPatch,
	identity_management::IDENTITY_PALLET_NAME,
	primitives::{
		address::Address32,
		identity::{Identity, ValidationData},
		MrEnclave,
	},
	utils::crypto::encrypt_with_tee_shielding_pubkey,
	ApiClient,
};
use codec::Encode;
use rsa::RsaPublicKey;
use sp_core::{Pair, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
	compose_call, compose_extrinsic, compose_extrinsic_offline, ApiResult, CallIndex, PlainTip,
	SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};

pub type SetUserShieldingKeyFn = (CallIndex, H256, Vec<u8>);
pub type SetUserShieldingKeyXt<SignedExtra> =
	UncheckedExtrinsicV4<SetUserShieldingKeyFn, SignedExtra>;
pub type AddDelegateFn = (CallIndex, Address32);
pub type AddDelegateXt<SignedExtra> = UncheckedExtrinsicV4<AddDelegateFn, SignedExtra>;
pub type CreateIdentityFn = (CallIndex, H256, Address32, Vec<u8>, Option<Vec<u8>>);
pub type CreateIdentityXt<SignedExtra> = UncheckedExtrinsicV4<CreateIdentityFn, SignedExtra>;
pub type RemoveIdentityFn = (CallIndex, H256, Vec<u8>);
pub type RemoveIdentityXt<SignedExtra> = UncheckedExtrinsicV4<RemoveIdentityFn, SignedExtra>;
pub type VerifyIdentityFn = (CallIndex, H256, Vec<u8>, Vec<u8>);
pub type VerifyIdentityXt<SignedExtra> = UncheckedExtrinsicV4<VerifyIdentityFn, SignedExtra>;

pub trait IdentityManagementXtBuilder {
	fn build_extrinsic_set_user_shielding_key(
		&self,
		shard: &MrEnclave,
		encrpted_shielding_key: &[u8],
	) -> SetUserShieldingKeyXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_add_delegatee(
		&self,
		account: &Address32,
	) -> AddDelegateXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_create_identity(
		&self,
		shard: &MrEnclave,
		address: &Address32,
		identity: &Identity,
		ciphertext_metadata: &Option<Vec<u8>>,
	) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_offline_create_identity(
		&self,
		nonce: u32,
		shard: &MrEnclave,
		address: &Address32,
		identity: &Identity,
		ciphertext_metadata: &Option<Vec<u8>>,
	) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_remove_identity(
		&self,
		shard: &MrEnclave,
		identity: &Identity,
	) -> RemoveIdentityXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_verify_identity(
		&self,
		shard: &MrEnclave,
		identity: &Identity,
		validation_data: &ValidationData,
	) -> VerifyIdentityXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn encrypt_identity_with_tee_shielding_key(
		tee_shielding_pubkey: RsaPublicKey,
		identity: Identity,
	) -> ApiResult<Vec<u8>>;
}

impl<P> IdentityManagementXtBuilder for ApiClient<P>
where
	P: Pair,
	MultiSignature: From<P::Signature>,
	MultiSigner: From<P::Public>,
{
	fn encrypt_identity_with_tee_shielding_key(
		tee_shielding_pubkey: RsaPublicKey,
		identity: Identity,
	) -> ApiResult<Vec<u8>> {
		let identity_encoded = identity.encode();
		let encrypted_identity =
			encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

		Ok(encrypted_identity)
	}

	fn build_extrinsic_set_user_shielding_key(
		&self,
		shard: &MrEnclave,
		encrpted_shielding_key: &[u8],
	) -> SetUserShieldingKeyXt<SubstrateDefaultSignedExtra<PlainTip>> {
		compose_extrinsic!(
			self.api.clone(),
			IDENTITY_PALLET_NAME,
			"set_user_shielding_key",
			H256::from(shard),
			encrpted_shielding_key.to_vec()
		)
	}

	fn build_extrinsic_add_delegatee(
		&self,
		account: &Address32,
	) -> AddDelegateXt<SubstrateDefaultSignedExtra<PlainTip>> {
		compose_extrinsic!(self.api.clone(), IDENTITY_PALLET_NAME, "add_delegatee", *account)
	}

	fn build_extrinsic_create_identity(
		&self,
		shard: &MrEnclave,
		address: &Address32,
		identity: &Identity,
		ciphertext_metadata: &Option<Vec<u8>>,
	) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>> {
		let identity_encoded = identity.encode();
		let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();
		let encrypted_identity =
			encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

		compose_extrinsic!(
			self.api.clone(),
			IDENTITY_PALLET_NAME,
			"create_identity",
			H256::from(shard),
			*address,
			encrypted_identity,
			ciphertext_metadata.clone()
		)
	}

	fn build_extrinsic_offline_create_identity(
		&self,
		nonce: u32,
		shard: &MrEnclave,
		address: &Address32,
		identity: &Identity,
		ciphertext_metadata: &Option<Vec<u8>>,
	) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>> {
		let identity_encoded = identity.encode();
		let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();
		let encrypted_identity =
			encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

		let meta = self.api.clone().metadata;
		let call = compose_call!(
			meta,
			IDENTITY_PALLET_NAME,
			"create_identity",
			H256::from(shard),
			*address,
			encrypted_identity,
			ciphertext_metadata.clone()
		);

		compose_extrinsic_offline!(
			self.api.clone().signer.unwrap(),
			call,
			self.api.clone().extrinsic_params(nonce)
		)
	}

	fn build_extrinsic_remove_identity(
		&self,
		shard: &MrEnclave,
		identity: &Identity,
	) -> RemoveIdentityXt<SubstrateDefaultSignedExtra<PlainTip>> {
		let identity_encoded = identity.encode();
		let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();
		let encrypted_identity =
			encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);

		compose_extrinsic!(
			self.api.clone(),
			IDENTITY_PALLET_NAME,
			"remove_identity",
			H256::from(shard),
			encrypted_identity
		)
	}

	fn build_extrinsic_verify_identity(
		&self,
		shard: &MrEnclave,
		identity: &Identity,
		validation_data: &ValidationData,
	) -> VerifyIdentityXt<SubstrateDefaultSignedExtra<PlainTip>> {
		let tee_shielding_pubkey = self.get_tee_shielding_pubkey().unwrap();

		let identity_encoded = identity.encode();
		let encrypted_identity =
			encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &identity_encoded);
		let validation_data_encoded = validation_data.encode();
		let encrypted_validation_data =
			encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &validation_data_encoded);

		compose_extrinsic!(
			self.api.clone(),
			IDENTITY_PALLET_NAME,
			"verify_identity",
			H256::from(shard),
			encrypted_identity,
			encrypted_validation_data
		)
	}
}
