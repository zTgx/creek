use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::ApiResult;

use crate::{
    primitives::{enclave::Enclave, AccountId},
    ApiClient,
};

pub trait VcManagementGetterApi {
    fn get_enclave_registry(&self) -> ApiResult<Enclave<AccountId, String>>;
}

impl<P> VcManagementGetterApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn get_enclave_registry(&self) -> ApiResult<Enclave<AccountId, String>> {
        let enclave_count: u64 = self
            .api
            .get_storage_value("Teerex", "EnclaveCount", None)
            .unwrap()
            .unwrap();

        //ApiResult<Option<Enclave<AccountId, String>>>
        let enclave: Enclave<AccountId, String> = self
            .api
            .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
            .unwrap()
            .unwrap();

        Ok(enclave)
    }
}
