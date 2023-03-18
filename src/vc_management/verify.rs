use jsonschema::{Draft, JSONSchema};

/**
 * Here are the key points that need to be verified for VC, and a large number of verifiable tests need to be conducted for VC.
 * All test cases are processed centrally in `test_vc_verify.r`.
 * 
 * https://www.notion.so/web3builders/VC-Verification-e21fcb4cd9004672ac25b44609c8a86b
 */
pub fn verify_vc_schema(decrypt_vc: &[u8]) -> bool {
    let vc: serde_json::Value = serde_json::from_slice(decrypt_vc).unwrap();
    let schema = include_bytes!("../../docs/templates/vc_schema.json");
    let schema: serde_json::Value = serde_json::from_slice(schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .compile(&schema)
        .unwrap();
   
   compiled_schema.is_valid(&vc)
}