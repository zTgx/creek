use litentry_test_suit::{ApiClient, utils::{generate_user_shielding_key, print_passed}, identity_management::IdentityManagementApi, primitives::Assertion, vc_management::{VcManagementApi, events::VcManagementEventApi}};
use sp_core::{sr25519, Pair};

#[test]
fn demo_request_vc_a4_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let divd = 1_u64;
    let divisor = 1000_u64;
    let a4 = Assertion::A4(divd, divisor);

    api_client.request_vc(&shard, &a4);

    let event = api_client.wait_event_vc_issued();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    print_passed();
}