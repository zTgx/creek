use std::time::SystemTime;

use sp_core::{sr25519, Pair};

use crate::{
    identity_management::IdentityManagementApi,
    primitives::assertion::{Assertion, ParameterString},
    utils::{crypto::generate_user_shielding_key, print_passed},
    vc_management::{events::VcManagementEventApi, VcManagementApi},
    ApiClient,
};

/// The relevant documentation for the fuzzy interface is here
/// @github : https://github.com/rust-fuzz/cargo-fuzz
/// Tutorial: https://rust-fuzz.github.io/book/cargo-fuzz/tutorial.html
pub fn fuzz_request_vc_a4(_balance: u128) {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a4 = Assertion::A4(balance);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion A4. <<<<<<<<<<<<<<<<<<<<<<<< ");
    let now = SystemTime::now();
    api_client.request_vc(&shard, &a4);

    let event = api_client.wait_event_vc_issued();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    let elapsed_secs = now.elapsed().unwrap().as_secs();
    println!(
        " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue A4 took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
        elapsed_secs
    );

    print_passed();
}
