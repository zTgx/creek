use super::build_request_vc_extrinsic;
use crate::{
    primitives::{Assertion, MrEnclave, VCContext},
    send_extrinsic, API,
};
use jsonschema::{Draft, JSONSchema};
use sp_core::H256;

/// request_vc
pub fn request_vc(shard: MrEnclave, assertion: Assertion) {
    let xt = build_request_vc_extrinsic(shard, assertion);
    send_extrinsic(xt.hex_encode());
}

/// verify vc schema
pub fn verify_vc_schema(decrypt_vc: &[u8]) -> bool {
    let vc: serde_json::Value = serde_json::from_slice(decrypt_vc).unwrap();
    let schema = include_bytes!("../../docs/templates/vc_schema.json");
    let schema: serde_json::Value = serde_json::from_slice(schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .compile(&schema)
        .unwrap();
    let is_valid = compiled_schema.is_valid(&vc);

    println!("\n ✅ VC json verifying...");

    is_valid
}

pub fn query_vc_registry(vc_index: H256) -> VCContext {
    let vc_context: VCContext = API
        .get_storage_map("VCManagement", "VCRegistry", vc_index, None)
        .unwrap()
        .unwrap();

    println!(" ✅ VCManagement VCContext : {:?}", vc_context);

    vc_context
}
