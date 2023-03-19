use crate::{
    identity_management::xtbuilder::IdentityManagementXtBuilder,
    primitives::{Address32, Identity, MrEnclave},
    utils::encrypt_with_tee_shielding_pubkey,
    ApiClient,
};
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};

pub trait IdentityManagementApi {
    fn set_user_shielding_key(&self, shard: MrEnclave, aes_key: Vec<u8>);
    fn create_identity(
        &self,
        shard: MrEnclave,
        address: Address32,
        identity: Identity,
        ciphertext_metadata: Option<Vec<u8>>,
    );
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
            encrypt_with_tee_shielding_pubkey(tee_shielding_pubkey, &aes_key);
        let xt = self.build_extrinsic_set_user_shielding_key(shard, encrpted_shielding_key);
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
}
