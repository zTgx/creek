use crate::{
    identity_management::xtbuilder::IdentityManagementXtBuilder,
    primitives::{Address32, Identity, MrEnclave, ValidationData},
    utils::encrypt_with_tee_shielding_pubkey,
    ApiClient,
};
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::ApiResult;

use super::IDENTITY_PALLET_NAME;

pub trait IdentityManagementApi {
    fn set_user_shielding_key(&self, shard: MrEnclave, aes_key: Vec<u8>);
    fn add_delegatee(&self, account: Address32);
    fn create_identity(
        &self,
        shard: MrEnclave,
        address: Address32,
        identity: Identity,
        ciphertext_metadata: Option<Vec<u8>>,
    );
    fn create_identity_offline(
        &self,
        nonce: u32,
        shard: MrEnclave,
        address: Address32,
        identity: Identity,
        ciphertext_metadata: Option<Vec<u8>>,
    );
    fn remove_identity(&self, shard: MrEnclave, identity: Identity);
    fn verify_identity(&self, shard: MrEnclave, identity: Identity, vdata: ValidationData);
}

pub trait IdentityManagementQueryApi {
    fn delegatee(&self, account: Address32) -> ApiResult<Option<()>>;
}

impl<P> IdentityManagementApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn set_user_shielding_key(&self, shard: MrEnclave, aes_key: Vec<u8>) {
        let tee_shielding_pubkey = self.get_tee_shielding_pubkey();
        let encrpted_shielding_key =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_pubkey, &aes_key);
        let xt = self.build_extrinsic_set_user_shielding_key(shard, encrpted_shielding_key);
        self.send_extrinsic(xt.hex_encode());
    }

    fn add_delegatee(&self, account: Address32) {
        let xt = self.build_extrinsic_add_delegatee(account);
        self.send_extrinsic(xt.hex_encode());
    }

    fn create_identity(
        &self,
        shard: MrEnclave,
        address: Address32,
        identity: Identity,
        ciphertext_metadata: Option<Vec<u8>>,
    ) {
        let xt =
            self.build_extrinsic_create_identity(shard, address, identity, ciphertext_metadata);
        self.send_extrinsic(xt.hex_encode());
    }

    fn create_identity_offline(
        &self,
        nonce: u32,
        shard: MrEnclave,
        address: Address32,
        identity: Identity,
        ciphertext_metadata: Option<Vec<u8>>,
    ) {
        let xt = self.build_extrinsic_offline_create_identity(
            nonce,
            shard,
            address,
            identity,
            ciphertext_metadata,
        );
        self.send_extrinsic(xt.hex_encode());
    }

    fn remove_identity(&self, shard: MrEnclave, identity: Identity) {
        let xt = self.build_extrinsic_remove_identity(shard, identity);
        self.send_extrinsic(xt.hex_encode());
    }

    fn verify_identity(&self, shard: MrEnclave, identity: Identity, vdata: ValidationData) {
        let xt = self.build_extrinsic_verify_identity(shard, identity, vdata);
        self.send_extrinsic(xt.hex_encode());
    }
}

impl<P> IdentityManagementQueryApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn delegatee(&self, account: Address32) -> ApiResult<Option<()>> {
        let ret: ApiResult<Option<()>> =
            self.api
                .get_storage_map(IDENTITY_PALLET_NAME, "Delegatee", account, None);

        ret
    }
}
