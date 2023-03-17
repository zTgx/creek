use crate::{API, primitives::{Assertion, MrEnclave, VCContext}, vc_management::PALLET_NAME};
use substrate_api_client::{compose_extrinsic, UncheckedExtrinsicV4, XtStatus, CallIndex, SubstrateDefaultSignedExtra, PlainTip};
use sp_core::H256;

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

pub fn send_extrinsic(xthex_prefixed: String) {
    let tx_hash = API.send_extrinsic(xthex_prefixed, XtStatus::InBlock).unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}

/// request_vc
pub fn request_vc(shard: MrEnclave, assertion: Assertion) {
    let xt = build_request_vc_extrinsic(shard, assertion);
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = API.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
}

pub fn query_vc_registry(vc_index: H256) -> VCContext {
    let vc_context: VCContext = API
    .get_storage_map("VCManagement", "VCRegistry", vc_index, None)
    .unwrap()
    .unwrap();

    println!("[VCManagement] VCContext is {:?}", vc_context);

    vc_context
}