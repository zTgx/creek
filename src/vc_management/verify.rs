use jsonschema::{Draft, JSONSchema};

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