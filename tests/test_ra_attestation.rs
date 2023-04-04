use litentry_api_client::{
    ra::attestation::ra_attestation, vc_management::getter::VcManagementGetterApi, ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_ra_attestation_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let enclave_registry = api_client.get_enclave_registry();
    assert!(enclave_registry.is_ok());

    let enclave_registry = enclave_registry.unwrap();
    assert!(ra_attestation(&enclave_registry).is_ok());
}
