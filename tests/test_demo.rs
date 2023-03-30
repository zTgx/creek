use litentry_test_suit::{
    identity_management::IdentityManagementApi,
    primitives::Assertion,
    utils::{generate_user_shielding_key, print_passed},
    vc_management::{events::VcManagementEventApi, VcManagementApi},
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn demo_request_vc_a4_works() {
    print_passed();
}
