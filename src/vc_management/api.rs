use crate::{
    primitives::{Assertion, MrEnclave, VCContext},
    ApiClient,
    vc_management::xtbuilder::VcManagementXtBuilder,
};
use sp_core::{Pair, H256};
use sp_runtime::{MultiSignature, MultiSigner};

pub trait VcManagementApi {
    fn request_vc(&self, shard: MrEnclave, assertion: Assertion);
    fn query_vc_registry(&self, vc_index: H256) -> VCContext;
}

impl<P> VcManagementApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn request_vc(&self, shard: MrEnclave, assertion: Assertion) {
        let xt = self.build_extrinsic_request_vc(shard, assertion);
        self.send_extrinsic(xt.hex_encode());
    }

    fn query_vc_registry(&self, vc_index: H256) -> VCContext {
        let vc_context: VCContext = self
            .api
            .get_storage_map("VCManagement", "VCRegistry", vc_index, None)
            .unwrap()
            .unwrap();

        vc_context
    }
}
