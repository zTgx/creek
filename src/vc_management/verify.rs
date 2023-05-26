use crate::primitives::vc::{Credential, CredentialType};
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

pub fn verify_vc(vc_pubkey: &ed25519::Public, vc: &Credential) -> Result<bool, String> {
    let verified_schema = verify_vc_schema(vc)?;
    println!("Verify scheme: {verified_schema}");
    if !verified_schema {
        return Ok(verified_schema);
    }

    let verified_vc_info = verify_vc_info(vc);
    println!("Verify scheme: {verified_vc_info}");

    let verified_subject = verify_vc_subject(vc);
    println!("Verify scheme: {verified_subject}");

    let verified_issuer = verify_vc_issuer(vc);
    println!("Verify scheme: {verified_issuer}");

    let verified_proof = verify_vc_proof(vc_pubkey, vc)?;
    println!("Verify scheme: {verified_proof}");

    Ok(
        verified_schema
            && verified_vc_info
            && verified_subject
            && verified_issuer
            && verified_proof,
    )
}

pub fn verify_vc_schema(vc: &Credential) -> Result<bool, String> {
    let vc: serde_json::Value = serde_json::to_value(vc).map_err(|e| format!("{:?}", e))?;
    let schema = include_bytes!("../../docs/templates/vc_schema.json");
    let schema: serde_json::Value =
        serde_json::from_slice(schema).map_err(|e| format!("{:?}", e))?;
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .compile(&schema)
        .map_err(|e| format!("{:?}", e))?;

    Ok(compiled_schema.is_valid(&vc))
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

pub fn verify_vc_proof(vc_pubkey: &ed25519::Public, vc: &Credential) -> Result<bool, String> {
    let mut value = serde_json::to_value(vc).map_err(|e| format!("{:?}", e))?;
    let proof = vc.proof.clone().expect("Proof");
    let sig = proof.proof_value;
    let sig = hex::decode(sig).map_err(|e| format!("{:?}", e))?;

    value["proof"] =
        serde_json::to_value::<Option<String>>(None).map_err(|e| format!("{:?}", e))?;

    let vc: Credential = serde_json::from_value(value).map_err(|e| format!("{:?}", e))?;
    let message = serde_json::to_string(&vc).map_err(|e| format!("{:?}", e))?;
    let signature =
        ed25519::Signature::from_slice(&sig).ok_or_else(|| "signature error.".to_string())?;

    Ok(Ed25519Pair::verify(&signature, message, vc_pubkey))
}
