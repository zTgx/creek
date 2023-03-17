use crate::{API, primitives::{Assertion, MrEnclave, VCContext}, send_extrinsic};
use sp_core::H256;
use super::build_request_vc_extrinsic;

/// request_vc
pub fn request_vc(shard: MrEnclave, assertion: Assertion) {
    let xt = build_request_vc_extrinsic(shard, assertion);
    send_extrinsic(xt.hex_encode());
}

pub fn query_vc_registry(vc_index: H256) -> VCContext {
    let vc_context: VCContext = API
    .get_storage_map("VCManagement", "VCRegistry", vc_index, None)
    .unwrap()
    .unwrap();

    println!(" ✅ VCManagement VCContext : {:?}", vc_context);

    vc_context
}