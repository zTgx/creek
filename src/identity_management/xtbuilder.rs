use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use sp_core::H256;
use substrate_api_client::{
    compose_extrinsic, CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};
use crate::{
    primitives::{Address32, MrEnclave},
    ApiClient, identity_management::IDENTITY_PALLET_NAME,
};

pub type SetUserShieldingKeyFn = (CallIndex, H256, Vec<u8>);
pub type SetUserShieldingKeyXt<SignedExtra> =
    UncheckedExtrinsicV4<SetUserShieldingKeyFn, SignedExtra>;
pub type CreateIdentityFn = (CallIndex, H256, Address32, Vec<u8>, Option<Vec<u8>>);
pub type CreateIdentityXt<SignedExtra> = UncheckedExtrinsicV4<CreateIdentityFn, SignedExtra>;

pub trait IdentityManagementXtBuilder {
    fn build_extrinsic_set_user_shielding_key(
        &self,
        shard: MrEnclave,
        encrpted_shielding_key: Vec<u8>,
    ) -> SetUserShieldingKeyXt<SubstrateDefaultSignedExtra<PlainTip>>;

    fn build_extrinsic_create_identity(
        &self,
        shard: MrEnclave,
        address: Address32,
        ciphertext: Vec<u8>,
        ciphertext_metadata: Option<Vec<u8>>,
    ) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>>;
}

impl<P> IdentityManagementXtBuilder for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn build_extrinsic_set_user_shielding_key(
        &self,
        shard: MrEnclave,
        encrpted_shielding_key: Vec<u8>,
    ) -> SetUserShieldingKeyXt<SubstrateDefaultSignedExtra<PlainTip>> {
        compose_extrinsic!(
            self.api.clone(),
            IDENTITY_PALLET_NAME,
            "set_user_shielding_key",
            H256::from(shard),
            encrpted_shielding_key
        )
    }

    fn build_extrinsic_create_identity(
        &self,
        shard: MrEnclave,
        address: Address32,
        ciphertext: Vec<u8>,
        ciphertext_metadata: Option<Vec<u8>>,
    ) -> CreateIdentityXt<SubstrateDefaultSignedExtra<PlainTip>> {
        compose_extrinsic!(
            self.api.clone(),
            IDENTITY_PALLET_NAME,
            "create_identity",
            H256::from(shard),
            address,
            ciphertext,
            ciphertext_metadata
        )
    }
}
