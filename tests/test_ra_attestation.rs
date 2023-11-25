use vc_sdk::{
    api_client_patch::parachain::ParachainPatch,
    ra::{RaAttestation, RaAttestationExecutor},
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_ra_attestation_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let enclave_registry = api_client.enclave(1).unwrap();
    assert!(enclave_registry.is_some());

    let enclave_registry = enclave_registry.unwrap();
    let ra = RaAttestation::new(enclave_registry);
    assert!(ra.execute().is_ok());
}
