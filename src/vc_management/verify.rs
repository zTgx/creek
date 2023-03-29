use crate::primitives::{Credential, CredentialType};
use jsonschema::{Draft, JSONSchema};
use sp_core::{
    ed25519::{self, Pair as Ed25519Pair},
    Pair,
};

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

pub fn verify_vc(vc_pubkey: &ed25519::Public, vc: &Credential) -> bool {
    let verified_schema = verify_vc_schema(vc);
    let verified_vc_info = verify_vc_info(vc);
    let verified_subject = verify_vc_subject(vc);
    let verified_issuer = verify_vc_issuer(vc);
    let verified_proof = verify_vc_proof(vc_pubkey, vc);

    verified_schema && verified_vc_info && verified_subject && verified_issuer && verified_proof
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
    let context = &vc.context;
    let verified_context =
        context.len() == 2 && context[0] == CONTEXT[0] && context[1] == CONTEXT[1];

    let types = &vc.types;
    let verified_types = types[0] == CredentialType::VerifiableCredential;

    verified_context && verified_types
}

/// TODO:
/// This data structure is the main content of VC, including the core content of assertion.
/// How to verify this part accurately and effectively?
pub fn verify_vc_subject(_vc: &Credential) -> bool {
    true
}

/// TODO:
/// Here is the RA related verification
pub fn verify_vc_issuer(_vc: &Credential) -> bool {
    true
}

pub fn verify_vc_proof(vc_pubkey: &ed25519::Public, vc: &Credential) -> bool {
    let mut value = serde_json::to_value(vc).expect("msg");

    let sig = vc.proof.clone().unwrap().proof_value;
    let sig = hex::decode(sig).unwrap();

    value["proof"] = serde_json::to_value::<Option<String>>(None).unwrap();

    let vc: Credential = serde_json::from_value(value).unwrap();
    let message = serde_json::to_string(&vc).expect("msg");

    Ed25519Pair::verify(
        &ed25519::Signature::from_slice(&sig).unwrap(),
        message,
        vc_pubkey,
    )
}
