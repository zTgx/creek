/*
Put the alpha-launch testcase of litentry-parachain here
Date: 2023-03-27
Modules: IdentityManagament / VCManagement / VC verify / Sidechain

Principle:
* Function names should be descriptive and clear.
* Names can be relatively long.
* One test case should be testing one thing.

Priority:
* P0 -> cover basic workflows (smoke testing) -> 20%
* P1 -> cover basic functionality & Abnormal process -> 20%
* P2 -> cover corner cases -> 60%
*/

use std::str::FromStr;

use litentry_api_client::{
    api_client_patch::{
        batch_all::BatchPatch, event::SubscribeEventPatch, parachain::ParachainPatch,
    },
    identity_management::IdentityManagementApi,
    primitives::assertion::{Assertion, IndexingNetwork, IndexingNetworks, ParameterString},
    utils::{
        crypto::{decrypt_vc_with_user_shielding_key, generate_user_shielding_key},
        vc::create_a_random_vc_index,
    },
    vc_management::{
        events::{VCDisabledEvent, VCIssuedEvent, VCManagementError, VCRevokedEvent},
        verify::verify_vc,
        xtbuilder::VcManagementXtBuilder,
        VcManagementApi,
    },
    ApiClient,
};
use sp_core::{sr25519, Pair, H256};

#[test]
fn alpha_function_name_should_be_descriptive_and_clear_works() {
    assert!(true);
}

#[test]
fn alpha_request_vc_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    let vc = decrypt_vc_with_user_shielding_key(&user_shielding_key, event.vc);
    assert!(vc.is_ok());
    let vc = vc.unwrap();
    let vc_pubkey = api_client.get_vc_pubkey().unwrap();
    let ret = verify_vc(&vc_pubkey, &vc);
    assert!(ret.is_ok());
    assert_eq!(ret.unwrap(), true)
}

#[test]
fn alpha_request_vc_a2_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let a2 = Assertion::A2(guild_id.clone());

    api_client.request_vc(&shard, &a2);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a3_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
    let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
    let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());

    api_client.request_vc(&shard, &a3);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a4_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a4 = Assertion::A4(balance);

    api_client.request_vc(&shard, &a4);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a6_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a6 = Assertion::A6;

    api_client.request_vc(&shard, &a6);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a7_10_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a7 = Assertion::A7(balance);

    api_client.request_vc(&shard, &a7);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a7_min_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a7 = Assertion::A7(balance);

    api_client.request_vc(&shard, &a7);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a7_max_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a7 = Assertion::A7(balance);

    api_client.request_vc(&shard, &a7);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

/// Need more
#[test]
fn alpha_request_vc_a8_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let litentry = IndexingNetwork::Litentry;
    let mut networks = IndexingNetworks::with_bounded_capacity(1);
    networks.try_push(litentry).unwrap();
    let a8 = Assertion::A8(networks);

    api_client.request_vc(&shard, &a8);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a10_10_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a10 = Assertion::A10(balance);

    api_client.request_vc(&shard, &a10);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a10_min_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a10 = Assertion::A10(balance);

    api_client.request_vc(&shard, &a10);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a10_max_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a10 = Assertion::A10(balance);

    api_client.request_vc(&shard, &a10);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a11_10_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a11 = Assertion::A11(balance);

    api_client.request_vc(&shard, &a11);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a11_min_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a11 = Assertion::A11(balance);

    api_client.request_vc(&shard, &a11);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
fn alpha_request_vc_a11_max_u128_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a11 = Assertion::A11(balance);

    api_client.request_vc(&shard, &a11);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}

#[test]
pub fn alpha_batch_all_request_vc_a4_a7_a10_a11_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a4 = Assertion::A4(balance.clone());
    let a7 = Assertion::A7(balance.clone());
    let a10 = Assertion::A10(balance.clone());
    let a11 = Assertion::A11(balance);

    let assertions = [a4, a7, a10, a11];
    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
pub fn alpha_batch_all_request_vc_all_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
    let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
    let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
    let litentry = IndexingNetwork::Litentry;
    let mut networks = IndexingNetworks::with_bounded_capacity(1);
    networks.try_push(litentry).unwrap();
    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();

    let a1 = Assertion::A1;
    let a2 = Assertion::A2(guild_id.clone());
    let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());
    let a4 = Assertion::A4(balance.clone());
    let a6 = Assertion::A6;
    let a7 = Assertion::A7(balance.clone());
    let a8 = Assertion::A8(networks);
    let a10 = Assertion::A10(balance.clone());
    let a11 = Assertion::A11(balance);

    let assertions = vec![a1, a2, a3, a4, a6, a7, a8, a10, a11];
    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
pub fn alpha_request_vc_a1_then_disable_it_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index = event.unwrap().index;
    api_client.disable_vc(&vc_index);

    let event = api_client.wait_event::<VCDisabledEvent>();
    assert!(event.is_ok());
}

#[test]
pub fn alpha_request_vc_two_a1_then_disable_second_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    // Frist A1
    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let vc_index_first_a1 = event.unwrap().index;

    // Second A1
    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index_second_a1 = event.unwrap().index;

    api_client.disable_vc(&vc_index_second_a1);
    let event = api_client.wait_event::<VCDisabledEvent>();

    assert!(event.is_ok());

    let a1_context = api_client.get_vc_context(&vc_index_first_a1).ok();
    assert!(a1_context.flatten().is_some());
}

#[test]
pub fn alpha_disable_vc_no_exsits_index_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let non_exists_vc_index = create_a_random_vc_index();
    api_client.disable_vc(&non_exists_vc_index);

    let event = api_client.wait_error::<VCManagementError>();
    assert!(event.is_err());
    match event {
        Ok(_) => panic!("Exptected the call to fail."),
        Err(e) => {
            let string_error = format!("{:?}", e);
            assert!(string_error.contains("pallet: \"VCManagement\""));
            assert!(string_error.contains("error: \"VCNotExist\""));
        }
    }
}

#[test]
fn alpha_disabled_vc_twice_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    let vc_index = event.index;
    api_client.disable_vc(&vc_index);
    api_client.disable_vc(&vc_index);

    let event = api_client.wait_error::<VCManagementError>();
    assert!(event.is_err());
    match event {
        Ok(_) => panic!("Exptected the call to fail."),
        Err(e) => {
            let string_error = format!("{:?}", e);
            assert!(string_error.contains("pallet: \"VCManagement\""));
            assert!(string_error.contains("error: \"VCAlreadyDisabled\""));
        }
    }
}

#[test]
fn alpha_request_vc_then_revoke_it_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index = event.unwrap().index;
    api_client.revoke_vc(&vc_index);

    let event = api_client.wait_event::<VCRevokedEvent>();
    assert!(event.is_ok());
}

#[test]
fn alpha_revoke_non_exists_vc_index_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let vc_index = create_a_random_vc_index();
    api_client.revoke_vc(&vc_index);

    let event = api_client.wait_error::<VCManagementError>();
    assert!(event.is_err());
    match event {
        Ok(_) => panic!("Exptected the call to fail."),
        Err(e) => {
            let string_error = format!("{:?}", e);
            assert!(string_error.contains("pallet: \"VCManagement\""));
            assert!(string_error.contains("error: \"VCNotExist\""));
        }
    }
}

#[test]
fn alpha_revoke_vc_twice_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    let vc_index = event.index;
    api_client.revoke_vc(&vc_index);
    api_client.revoke_vc(&vc_index);

    let event = api_client.wait_error::<VCManagementError>();
    assert!(event.is_err());
    match event {
        Ok(_) => panic!("Exptected the call to fail."),
        Err(e) => {
            let string_error = format!("{:?}", e);
            assert!(string_error.contains("pallet: \"VCManagement\""));
            assert!(string_error.contains("error: \"VCNotExist\""));
        }
    }
}

#[test]
fn alpha_request_disable_revoke_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index = event.unwrap().index;
    api_client.disable_vc(&vc_index);

    let event = api_client.wait_event::<VCDisabledEvent>();
    assert!(event.is_ok());

    api_client.revoke_vc(&vc_index);

    let event = api_client.wait_event::<VCRevokedEvent>();
    assert!(event.is_ok());
}

#[test]
fn alpha_request_vc_batch_all_10s_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let len = 10;
    let mut assertions = vec![];
    for _ in 0..len {
        assertions.push(Assertion::A1);
    }

    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
fn alpha_request_vc_batch_all_20s_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let len = 20;
    let mut assertions = vec![];
    for _ in 0..len {
        assertions.push(Assertion::A1);
    }

    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
fn alpha_request_vc_batch_all_50s_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let len = 50;
    let mut assertions = vec![];
    for _ in 0..len {
        assertions.push(Assertion::A1);
    }

    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
fn alpha_request_vc_batch_all_100s_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let len = 100;
    let mut assertions = vec![];
    for _ in 0..len {
        assertions.push(Assertion::A1);
    }

    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
fn alpha_request_vc_batch_all_200s_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let len = 200;
    let mut assertions = vec![];
    for _ in 0..len {
        assertions.push(Assertion::A1);
    }

    let mut calls = vec![];

    assertions.iter().for_each(|assertion| {
        calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&calls).hex_encode());

    let issued_events: Vec<VCIssuedEvent> = api_client.wait_events(assertions.len()).unwrap();
    assert_eq!(issued_events.len(), assertions.len());
}

#[test]
fn alpha_query_vc_registry_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    let vc_context = api_client.get_vc_context(&event.index).ok();
    assert!(vc_context.flatten().is_some());
}

#[test]
fn alpha_query_vc_registry_non_exists_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let non_exists_vc_index = create_a_random_vc_index();
    let vc_context = api_client.get_vc_context(&non_exists_vc_index).ok();
    assert!(vc_context.flatten().is_some());
}

/// Not Done yet
/// https://github.com/litentry/litentry-parachain/issues/1520
/// multiple-account request api simultaneously
/// 1. set_user_shielding_key
/// 2. request_vc
/// ...
///
// #[test]
// fn alpha_too_many_request_to_parachain_at_same_time_works() {
//     use std::thread;

//     const NTHREADS: usize = 10;

//     // create multi accounts
//     let pair = create_n_random_sr25519_address(NTHREADS).unwrap();
//     let amount = 10;

//     // sudo transfer balance 10 to those accounts
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();
//     let shard = api_client.get_shard().unwrap();
//     {
//         pair.iter().for_each(|pair| {
//             let account = pair.public();
//             let xt = api_client
//                 .api
//                 .balance_transfer(sp_runtime::MultiAddress::Id(account.into()), amount);

//             api_client.send_extrinsic(xt.hex_encode());

//             println!("Send {} amounts to {:?} Done", amount, account);
//         });

//         println!("Send all balance Done.")
//     }

//     // 1.
//     fn set_user_shielding_key(account: &sr25519::Pair, shard: MrEnclave) {
//         println!("New account: {:?}", account.public());
//         let api_client = ApiClient::new_with_signer(account.clone()).unwrap();

//         let user_shielding_key = generate_user_shielding_key();
//         api_client
//             .set_user_shielding_key(&shard, &user_shielding_key)
//             .unwrap();
//     }

//     std::thread::sleep(std::time::Duration::from_secs(3));

//     let mut children = vec![];

//     pair.iter().for_each(|pair| {
//         let pair = pair.clone();
//         children.push(thread::spawn(move || set_user_shielding_key(&pair, shard)));
//     });

//     for child in children {
//         // Wait for the thread to finish. Returns a result.
//         let _ = child.join();
//     }

//     let events: Vec<SetUserShieldingKeyEvent> = api_client.wait_events(NTHREADS).unwrap();
//     println!("events: {:?}", events)
// }

#[test]
fn alpha_query_enclave_count_storage_works() {
    use sp_core::storage::StorageKey;

    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let key = [
        212, 144, 122, 173, 74, 14, 160, 178, 189, 172, 196, 32, 211, 107, 233, 120, 54, 87, 4,
        174, 78, 139, 245, 224, 241, 123, 140, 4, 177, 67, 119, 2,
    ];

    // let storage_key = storage_value_key("Teerex", "EnclaveCount");
    let storage_key = StorageKey(key.to_vec());

    let hash = "0x1ae23916d72fe9153ffe25b1e9df6c380d7555b6fb0fe68e0ccf86e6d1fe07bc";
    let block_hash = H256::from_str(hash).unwrap();
    let x = api_client
        .api
        .get_storage_proof_by_keys(vec![storage_key], Some(block_hash));

    println!("teerex enclave count proof: {:?}", x)
}
