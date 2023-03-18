pub mod api;
pub mod events;
pub mod verify;

pub const VC_PALLET_NAME: &str = "VCManagement";
pub type VCRequestFn = (CallIndex, H256, Assertion);
pub type VCRequestXt<SignedExtra> = UncheckedExtrinsicV4<VCRequestFn, SignedExtra>;

use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use sp_core::H256;
use substrate_api_client::{
    compose_extrinsic, CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};
use crate::{
    primitives::{Assertion, MrEnclave},
    ApiClient,
};

pub trait VcManagementXtBuilder {
    fn build_extrinsic_request_vc(
        &self,
        shard: MrEnclave,
        assertion: Assertion,
    ) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>>;
}

impl<P> VcManagementXtBuilder for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn build_extrinsic_request_vc(
        &self,
        shard: MrEnclave,
        assertion: Assertion,
    ) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>> {
        compose_extrinsic!(
            self.api.clone(),
            VC_PALLET_NAME,
            "request_vc",
            H256::from(shard),
            assertion
        )
    }
}
