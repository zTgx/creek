use super::VcManagementApi;
use crate::{
    primitives::{assertion::Assertion, MrEnclave},
    vc_management::xtbuilder::VcManagementXtBuilder,
    ApiClient,
};
use sp_core::{Pair, H256};
use sp_runtime::{MultiSignature, MultiSigner};

impl<P> VcManagementApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn request_vc(&self, shard: &MrEnclave, assertion: &Assertion) {
        let xt = self.build_extrinsic_request_vc(shard, assertion);
        self.send_extrinsic(xt.hex_encode());
    }

    fn disable_vc(&self, vc_index: &H256) {
        let xt = self.build_extrinsic_disable_vc(vc_index);
        self.send_extrinsic(xt.hex_encode());
    }

    fn revoke_vc(&self, vc_index: &H256) {
        let xt = self.build_extrinsic_revoke_vc(vc_index);
        self.send_extrinsic(xt.hex_encode());
    }
}
