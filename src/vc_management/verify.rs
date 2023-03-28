use jsonschema::{Draft, JSONSchema};

use crate::primitives::{Credential, CredentialType};

/**
 * Here are the key points that need to be verified for VC, and a large number of verifiable tests need to be conducted for VC.
 * All test cases are processed centrally in `test_vc_verify.r`.
 *
 * https://www.notion.so/web3builders/VC-Verification-e21fcb4cd9004672ac25b44609c8a86b
 */

const CONTEXT: [&str; 2] = [
    "https://www.w3.org/2018/credentials/v1",
    "https://w3id.org/security/suites/ed25519-2020/v1",
];

pub fn verify_vc(vc: &Credential) -> bool {
    let verified_schema = verify_vc_schema(vc);
    let verified_vc_info = verify_vc_info(vc);
    let verified_subject = verify_vc_credential_subject(vc);

    verified_schema && verified_vc_info && verified_subject
}

pub fn verify_vc_schema(vc: &Credential) -> bool {
    let vc: serde_json::Value = serde_json::to_value(vc).unwrap();
    let schema = include_bytes!("../../docs/templates/vc_schema.json");
    let schema: serde_json::Value = serde_json::from_slice(schema).unwrap();
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .compile(&schema)
        .unwrap();

    compiled_schema.is_valid(&vc)
}

pub fn verify_vc_info(vc: &Credential) -> bool {
    println!("vc: {:?}", vc);

    let context = &vc.context;
    let verified_context =
        context.len() == 2 && context[0] == CONTEXT[0] && context[1] == CONTEXT[1];

    let types = &vc.types;
    let verified_types = types[0] == CredentialType::VerifiableCredential;

    verified_context && verified_types
}

pub fn verify_vc_credential_subject(_vc: &Credential) -> bool {
    true
}
