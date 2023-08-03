use litentry_api_client::{
    api_client_patch::{
        batch_all::BatchPatch, event::SubscribeEventPatch, parachain::ParachainPatch,
    },
    identity_management::IdentityManagementApi,
    primitives::assertion::{Assertion, ParameterString},
    utils::{crypto::generate_user_shielding_key, print_passed, vc::create_a_random_vc_index},
    vc_management::{
        events::{VCDisabledEvent, VCIssuedEvent, VCManagementError, VCRevokedEvent},
        xtbuilder::VcManagementXtBuilder,
        VcManagementApi,
    },
    ApiClient,
};
use sp_core::{sr25519, Pair};
use std::time::SystemTime;

// #[test]
// fn tc_request_vc_all_works() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client
//         .set_user_shielding_key(&shard, &user_shielding_key)
//         .unwrap();

//     let a1 = Assertion::A1;

//     let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
//     let a2 = Assertion::A2(guild_id.clone());

//     let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
//     let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
//     let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
//     let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());

//     let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
//     let a4 = Assertion::A4(balance.clone());

//     let original_tweet_id =
//         ParameterString::try_from("original_tweet_id".as_bytes().to_vec()).unwrap();
//     let a5 = Assertion::A5(original_tweet_id);
//     let a6 = Assertion::A6;

//     let a7 = Assertion::A7(balance.clone());

//     let litentry = IndexingNetwork::Litentry;
//     let mut networks = IndexingNetworks::with_bounded_capacity(1);
//     networks.try_push(litentry).unwrap();
//     let a8 = Assertion::A8(networks);

//     let a10 = Assertion::A10(balance.clone());
//     let a11 = Assertion::A11(balance);

//     let assertions = vec![a1, a2, a3, a4, a5, a6, a7, a8, a10, a11];
//     assertions.into_iter().for_each(|assertion| {
//         api_client.request_vc(&shard, &assertion);

//         let event = api_client.wait_event::<VCIssuedEvent>();
//         assert!(event.is_ok());
//         let event = event.unwrap();
//         assert_eq!(event.account, api_client.get_signer().unwrap());
//     });

//     print_passed()
// }

#[test]
pub fn tc_batch_request_vc() {
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
    let mut assertion_calls = vec![];
    assertions.into_iter().for_each(|assertion| {
        assertion_calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.api.batch(assertion_calls).hex_encode());
}

#[test]
pub fn tc_batch_all_request_vc_a471011_works() {
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
    let mut assertion_calls = vec![];
    assertions.into_iter().for_each(|assertion| {
        assertion_calls.push(
            api_client
                .build_extrinsic_request_vc(&shard, &assertion)
                .function,
        );
    });
    api_client.send_extrinsic(api_client.batch_all(&assertion_calls).hex_encode());
}

#[test]
pub fn tc_request_vc_then_disable_it_success() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    // Inputs
    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    // Wait event
    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index = event.unwrap().index;
    println!(" ✅ VC Index : {:?}", vc_index);

    api_client.disable_vc(&vc_index);

    let event = api_client.wait_event::<VCDisabledEvent>();
    assert!(event.is_ok());

    print_passed();
}

#[test]
pub fn tc_request_2_vc_then_disable_second_success() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    // Inputs
    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index_a1 = event.unwrap().index;
    println!(" ✅ A1 VC Index : {:?}", vc_index_a1);

    let a6 = Assertion::A6;
    api_client.request_vc(&shard, &a6);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index_a6 = event.unwrap().index;
    println!(" ✅ A6 VC Index : {:?}", vc_index_a6);

    api_client.disable_vc(&vc_index_a6);
    let event = api_client.wait_event::<VCDisabledEvent>();
    assert!(event.is_ok());

    let a1_context = api_client.get_vc_context(&vc_index_a1).ok();
    assert!(a1_context.is_some());

    print_passed();
}

#[test]
fn tc_request_vc_and_revoke_it_success() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    // Inputs
    let a1 = Assertion::A1;
    api_client.request_vc(&shard, &a1);

    // Wait event
    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());

    let vc_index = event.unwrap().index;
    println!(" ✅ A1 VC Index : {:?}", vc_index);

    api_client.revoke_vc(&vc_index);

    let event = api_client.wait_event::<VCRevokedEvent>();
    assert!(event.is_ok());

    print_passed();
}

#[test]
fn tc_request_vc_a1_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let a1 = Assertion::A1;

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion A1. <<<<<<<<<<<<<<<<<<<<<<<< ");
    let now = SystemTime::now();
    api_client.request_vc(&shard, &a1);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());

    let elapsed_secs = now.elapsed().unwrap().as_secs();
    println!(
        " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue A1 took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
        elapsed_secs
    );

    print_passed();
}

#[test]
fn tc_request_vc_a4_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a4 = Assertion::A4(balance);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion A4. <<<<<<<<<<<<<<<<<<<<<<<< ");
    let now = SystemTime::now();
    api_client.request_vc(&shard, &a4);

    let event = api_client.wait_event::<VCIssuedEvent>();
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

// #[test]
// fn tc_request_vc_all_with_timestamp() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client
//         .set_user_shielding_key(&shard, &user_shielding_key)
//         .unwrap();

//     println!("  [+] Start testing and apply for all assertions based on 30 dentities. ");

//     let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
//     let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
//     let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
//     let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
//     let networks = IndexingNetworks::with_bounded_capacity(1);

//     let a1 = Assertion::A1;
//     let a2 = Assertion::A2(guild_id.clone());
//     let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());
//     let a4 = Assertion::A4(balance.clone());
//     let a6 = Assertion::A6;
//     let a7 = Assertion::A7(balance.clone());
//     let a8 = Assertion::A8(networks);
//     let a10 = Assertion::A10(balance.clone());
//     let a11 = Assertion::A11(balance);

//     let assertions = vec![a1, a2, a3, a4, a6, a7, a8, a10, a11];
//     let assertion_names = vec!["A1", "A2", "A3", "A4", "A6", "A7", "A8", "A10", "A11"];

//     assertions.into_iter().enumerate().for_each(|(idx, assertion)| {
//         let assertion_name = assertion_names[idx];
//         println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion {}. <<<<<<<<<<<<<<<<<<<<<<<< ", assertion_name);

//         let now = SystemTime::now();

//         api_client.request_vc(&shard, &assertion);

//         let event = api_client.wait_event::<VCIssuedEvent>();
//         assert!(event.is_ok());
//         assert_eq!(event.unwrap().account, api_client.get_signer().unwrap());

//         let elapsed_secs = now.elapsed().unwrap().as_secs();
//         println!(
//             " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue {} took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
//             assertion_name, elapsed_secs
//         );
//     });
// }

#[test]
fn tc_disable_non_exists_vc_index() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let vc_index = create_a_random_vc_index();
    api_client.disable_vc(&vc_index);

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

    print_passed();
}

#[test]
fn tc_revoke_non_exists_vc_index() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let vc_index = create_a_random_vc_index();
    api_client.disable_vc(&vc_index);

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

    print_passed();
}

#[test]
fn tc_double_disabled_vc() {
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

    print_passed();
}

#[test]
fn tc_double_revoke_vc() {
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

    print_passed();
}

// TODO:
// Remove sidecar related code
// Maybe sidecar is not a suitable solution right now, keep it here for future use.
// #[allow(dead_code)]
// // #[test]
// fn tc_query_storage_vc_registry_by_endpoint() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client.set_user_shielding_key(&shard, &user_shielding_key).unwrap();

//     let a1 = Assertion::A1;
//     api_client.request_vc(&shard, &a1);

//     let event = api_client.wait_event::<VCIssuedEvent>();
//     assert!(event.is_ok());
//     let event = event.unwrap();
//     assert_eq!(event.account, api_client.get_signer().unwrap());

//     let encrypted_vc = event.vc;
//     let vc = decrypt_vc_with_user_shielding_key(&user_shielding_key, encrypted_vc).unwrap();
//     let endpoint = vc.credential_subject.endpoint;
//     let vc_index = event.index;
//     let index = vc_index.to_string();
//     let vc_cotext = reqwest::blocking::get(endpoint + &index)
//         .unwrap()
//         .json::<VCContext>()
//         .unwrap();
//     assert_eq!(vc_cotext.hash, vc_index);
// }

// #[test]
// fn tc_request_vc_a5_invalid_input_works() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client
//         .set_user_shielding_key(&shard, &user_shielding_key)
//         .unwrap();

//     let original_tweet_id = ParameterString::try_from([].to_vec()).unwrap();
//     let a5 = Assertion::A5(original_tweet_id);

//     println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion A5. <<<<<<<<<<<<<<<<<<<<<<<< ");
//     let now = SystemTime::now();
//     api_client.request_vc(&shard, &a5);

//     {
//         let issued_events: Vec<RequestVCFailedEvent> = api_client.wait_events(1).unwrap();
//         println!("event collect: {:?}", issued_events);
//     }
//     let event = api_client.wait_event::<RequestVCFailedEvent>();
//     println!("event: {:?}", event);
//     assert!(event.is_ok());
//     let event = event.unwrap();
//     assert_eq!(event.assertion, a5);

//     let elapsed_secs = now.elapsed().unwrap().as_secs();
//     println!(
//         " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue A5 took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
//         elapsed_secs
//     );

//     print_passed();
// }

// #[test]
// fn tc_request_vc_a8_with_empty_networks_works() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client
//         .set_user_shielding_key(&shard, &user_shielding_key)
//         .unwrap();

//     let networks = IndexingNetworks::with_bounded_capacity(0);
//     let a8 = Assertion::A8(networks);

//     println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion A8. <<<<<<<<<<<<<<<<<<<<<<<< ");
//     let now = SystemTime::now();
//     api_client.request_vc(&shard, &a8);

//     let event = api_client.wait_event::<VCIssuedEvent>();
//     assert!(event.is_ok());
//     let event = event.unwrap();
//     assert_eq!(event.assertion, a8);

//     let elapsed_secs = now.elapsed().unwrap().as_secs();
//     println!(
//         " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue A8 took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
//         elapsed_secs
//     );

//     print_passed();
// }

// #[test]
// fn tc_request_vc_a5_works() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client
//         .set_user_shielding_key(&shard, &user_shielding_key)
//         .unwrap();

//     let tid = ParameterString::try_from("1646193933473681408".as_bytes().to_vec()).unwrap();
//     let a5 = Assertion::A5(tid);

//     api_client.request_vc(&shard, &a5);

//     let event = api_client.wait_event::<VCIssuedEvent>();
//     assert!(event.is_ok());

//     print_passed();
// }

// #[test]
// fn tc_request_vc_a8_with_3_networks_works() {
//     let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
//     let api_client = ApiClient::new_with_signer(alice).unwrap();

//     let shard = api_client.get_shard().unwrap();
//     let user_shielding_key = generate_user_shielding_key();
//     api_client
//         .set_user_shielding_key(&shard, &user_shielding_key)
//         .unwrap();

//     let mut networks = IndexingNetworks::with_bounded_capacity(3);
//     networks.try_push(IndexingNetwork::Ethereum).unwrap();
//     networks.try_push(IndexingNetwork::Litentry).unwrap();
//     networks.try_push(IndexingNetwork::LitentryRococo).unwrap();

//     let a8 = Assertion::A8(networks);

//     println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion A8. <<<<<<<<<<<<<<<<<<<<<<<< ");
//     let now = SystemTime::now();
//     api_client.request_vc(&shard, &a8);

//     let event = api_client.wait_event::<VCIssuedEvent>();
//     assert!(event.is_ok());
//     let event = event.unwrap();
//     assert_eq!(event.assertion, a8);

//     let elapsed_secs = now.elapsed().unwrap().as_secs();
//     println!(
//         " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue A8 took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
//         elapsed_secs
//     );

//     print_passed();
// }

#[test]
fn tc_request_vc_a10_works() {
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

    println!("VC Issued: {:?}", event);
}

#[test]
fn tc_request_vc_a11_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
    let a10 = Assertion::A11(balance);

    api_client.request_vc(&shard, &a10);

    let event = api_client.wait_event::<VCIssuedEvent>();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.account, api_client.get_signer().unwrap());
}
