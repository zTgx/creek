use crate::{
    identity_management::IDENTITY_PALLET_NAME,
    primitives::{
        address::Address32, crypto::RsaPublicKeyGenerator, enclave::Enclave, vc::VCContext,
        AccountId, MrEnclave, VCIndex,
    },
    utils::address::vec_to_u8_array,
    vc_management::VC_PALLET_NAME,
    ApiClient,
};
use rsa::RsaPublicKey;
use sp_core::{ed25519, hexdisplay::HexDisplay, Pair};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::ApiResult;

pub trait ParachainPatch {
    fn get_tee_shielding_pubkey(&self) -> RsaPublicKey;
    fn get_vc_pubkey(&self) -> ed25519::Public;
    fn get_shard(&self) -> MrEnclave;
    fn vc_registry(&self, vc_index: &VCIndex) -> Option<VCContext>;
    fn delegatee(&self, account: Address32) -> ApiResult<Option<()>>;
}

impl<P> ParachainPatch for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn get_tee_shielding_pubkey(&self) -> RsaPublicKey {
        let enclave_count: u64 = self
            .api
            .get_storage_value("Teerex", "EnclaveCount", None)
            .unwrap()
            .unwrap();

        let enclave: Enclave<AccountId, Vec<u8>> = self
            .api
            .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
            .unwrap()
            .unwrap();

        let shielding_key = enclave.shielding_key.unwrap();
        RsaPublicKey::new_with_rsa3072_pubkey(shielding_key)
    }

    fn get_vc_pubkey(&self) -> ed25519::Public {
        let enclave_count: u64 = self
            .api
            .get_storage_value("Teerex", "EnclaveCount", None)
            .unwrap()
            .unwrap();

        let enclave: Enclave<AccountId, Vec<u8>> = self
            .api
            .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
            .unwrap()
            .unwrap();

        let vc_pubkey = enclave.vc_pubkey.expect("vc pubkey");
        ed25519::Public(vec_to_u8_array::<32>(vc_pubkey))
    }

    /// There're two methos to get the mrenclave
    /// 1. Online -> to use this method `get_shard` or
    /// 2. Offline -> to `litentry-parachain/tee-worker` run `make enclave`
    /// Both should be display exactly same value.
    ///
    /// TODO:
    /// But there's a question, what's the difference betwwen `mrenclave` and `shard`?
    fn get_shard(&self) -> MrEnclave {
        let enclave_count: u64 = self
            .api
            .get_storage_value("Teerex", "EnclaveCount", None)
            .unwrap()
            .unwrap();

        let enclave: Enclave<AccountId, Vec<u8>> = self
            .api
            .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
            .unwrap()
            .unwrap();

        let shard = enclave.mr_enclave;
        let shard_in_hex = format!("0x{}", HexDisplay::from(&shard));

        println!("\n ✅ New shard : {}", shard_in_hex);

        shard
    }

    fn vc_registry(&self, vc_index: &VCIndex) -> Option<VCContext> {
        let vc_context: Option<VCContext> = self
            .api
            .get_storage_map(VC_PALLET_NAME, "VCRegistry", vc_index, None)
            .unwrap();

        vc_context
    }

    fn delegatee(&self, account: Address32) -> ApiResult<Option<()>> {
        let ret: ApiResult<Option<()>> =
            self.api
                .get_storage_map(IDENTITY_PALLET_NAME, "Delegatee", account, None);

        ret
    }
}
