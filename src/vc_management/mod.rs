use sp_core::H256;
use substrate_api_client::{
    compose_extrinsic, CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};

use crate::{
    primitives::{Assertion, MrEnclave},
    API,
};

pub mod api;
pub mod events;

pub const VC_PALLET_NAME: &str = "VCManagement";

pub type VCRequestFn = (CallIndex, H256, Assertion);
pub type VCRequestXt<SignedExtra> = UncheckedExtrinsicV4<VCRequestFn, SignedExtra>;

pub fn build_request_vc_extrinsic(
    shard: MrEnclave,
    assertion: Assertion,
) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>> {
    compose_extrinsic!(
        API.clone(),
        VC_PALLET_NAME,
        "request_vc",
        H256::from(shard),
        assertion
    )
}
