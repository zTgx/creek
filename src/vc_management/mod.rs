use sp_core::H256;
use substrate_api_client::{SubstrateDefaultSignedExtra, PlainTip, compose_extrinsic, CallIndex, UncheckedExtrinsicV4};

use crate::{primitives::{MrEnclave, Assertion}, API};

pub mod events;
pub mod api;
pub mod verification;

pub const PALLET_NAME: &'static str = "VCManagement";

pub type VCRequestFn = (CallIndex, H256, Assertion);
pub type VCRequestXt<SignedExtra> = UncheckedExtrinsicV4<VCRequestFn, SignedExtra>;

pub fn build_request_vc_extrinsic(shard: MrEnclave, assertion: Assertion) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>> {
    compose_extrinsic!(
        API.clone(),
        PALLET_NAME,
        "request_vc",
        H256::from(shard),
        assertion
    )
}
