use litentry_test_suit::{
    identity_management::{
        events::{
            IdentityManagementEventApi, SetUserShieldingKeyEvent,
            SetUserShieldingKeyHandlingFailedEvent,
        },
        IdentityManagementApi,
    },
    primitives::{
        Address32, Identity, IdentityMultiSignature, ParameterString, SubstrateNetwork,
        ValidationData, Web3CommonValidationData, Web3ValidationData,
    },
    utils::{
        create_n_random_sr25519_address, decrypt_challage_code_with_user_shielding_key,
        decrypt_id_graph_with_user_shielding_key, decrypt_identity_with_user_shielding_key,
        generate_incorrect_user_shielding_key, generate_user_shielding_key,
        hex_account_to_address32, print_passed, ValidationDataBuilder,
    },
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_set_user_shielding_key_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let event = api_client.wait_event_user_shielding_key_set();
    let expect_event = SetUserShieldingKeyEvent {
        account: api_client.get_signer().unwrap(),
    };
    assert!(event.is_ok());
    assert_eq!(event.unwrap(), expect_event);

    print_passed();
}

#[test]
fn tc_set_user_shielding_key_faild() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_incorrect_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let event = api_client.wait_event_set_user_shielding_key_handle_failed();
    let expect_event = SetUserShieldingKeyHandlingFailedEvent;

    assert!(event.is_ok());
    assert_eq!(event.unwrap(), expect_event);

    print_passed();
}

#[test]
fn tc_add_delegatee_error() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let event = api_client.wait_event_user_shielding_key_set();
    let expect_event = SetUserShieldingKeyEvent {
        account: api_client.get_signer().unwrap(),
    };
    assert!(event.is_ok());
    assert_eq!(event.unwrap(), expect_event);

    let bob_pair = sr25519::Pair::from_string("//Bob", None).unwrap();
    let bob: Address32 = bob_pair.public().0.into();
    api_client.add_delegatee(&bob);

    let event = api_client.wait_event_delegatee_added();
    assert!(event.is_err());
}

#[test]
fn tc_create_identity() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();

    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate {
        network,
        address: address.clone(),
    };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    print_passed();
}

#[test]
fn tc_create_identity_then_remove_it() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();
    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    api_client.remove_identity(&shard, &identity);
    let event = api_client.wait_event_identity_removed();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    print_passed();
}

#[test]
fn tc_create_identity_then_verify_it() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();

    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.who, api_client.get_signer().unwrap());

    let encrypted_challenge_code = event.code;
    let challenge_code = decrypt_challage_code_with_user_shielding_key(
        &user_shielding_key,
        encrypted_challenge_code,
    )
    .unwrap();

    let vdata =
        ValidationData::build_vdata_substrate(&alice_pair, &address, &identity, &challenge_code);
    api_client.verify_identity(&shard, &identity, &vdata);
    let event = api_client.wait_event_identity_verified();
    assert!(event.is_ok());
    let event = event.unwrap();
    let id_graph = decrypt_id_graph_with_user_shielding_key(event.id_graph, &user_shielding_key);

    assert!(id_graph.is_ok());
    let id_graph = id_graph.unwrap();
    id_graph.iter().for_each(|(_, identity_context)| {
        assert_eq!(identity_context.is_verified, true);
    });

    print_passed()
}

#[test]
fn tc_create_a_random_identity_then_verify_it() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();

    let network = SubstrateNetwork::Litentry;
    let pair = create_n_random_sr25519_address(1)[0].clone();
    let pubkey: Address32 = pair.public().0.into();

    let identity = Identity::Substrate {
        network,
        address: pubkey,
    };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.who, api_client.get_signer().unwrap());

    let encrypted_challenge_code = event.code;
    let challenge_code = decrypt_challage_code_with_user_shielding_key(
        &user_shielding_key,
        encrypted_challenge_code,
    )
    .unwrap();

    let vdata = ValidationData::build_vdata_substrate(&pair, &alice, &identity, &challenge_code);
    api_client.verify_identity(&shard, &identity, &vdata);
    let event = api_client.wait_event_identity_verified();
    assert!(event.is_ok());

    print_passed()
}

#[test]
fn tc_create_identity_with_all_substrate_network() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [
        SubstrateNetwork::Polkadot,
        SubstrateNetwork::Kusama,
        SubstrateNetwork::Litentry,
        SubstrateNetwork::Litmus,
        SubstrateNetwork::Khala,
    ];

    let addresses = [address.clone()];
    networks.iter().for_each(|network| {
        addresses.iter().for_each(|address| {
            let identity = Identity::Substrate {
                network: network.clone(),
                address: address.clone(),
            };
            api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());
        })
    });

    print_passed();
}

#[test]
fn tc_verify_identity_with_unexpected_message_event() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    // Alice
    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();
    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    let message = ParameterString::try_from("message".as_bytes().to_vec()).unwrap();
    let sr25519_sig = alice_pair.sign(&message);
    let signature = IdentityMultiSignature::Sr25519(sr25519_sig);
    let web3_common_validation_data = Web3CommonValidationData { message, signature };

    let vdata = ValidationData::Web3(Web3ValidationData::Substrate(web3_common_validation_data));
    api_client.verify_identity(&shard, &identity, &vdata);
    let event = api_client.wait_event_unexpected_message();
    assert!(event.is_ok());

    print_passed()
}

#[test]
fn tc_create_identity_error_unauthorised_user() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let ciphertext_metadata: Option<Vec<u8>> = None;

    let bob = sr25519::Pair::from_string("//Bob", None).unwrap();
    let bob: Address32 = bob.public().0.into();

    let identity = Identity::Substrate {
        network: SubstrateNetwork::Polkadot,
        address: bob.clone(),
    };
    api_client.create_identity(&shard, &bob, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_err());
    match event {
        Ok(_) => panic!("Exptected the call to fail."),
        Err(e) => {
            let string_error = format!("{:?}", e);
            assert!(string_error.contains("pallet: \"IdentityManagement\""));
            assert!(string_error.contains("error: \"UnauthorisedUser\""));
        }
    }

    print_passed();
}

#[test]
fn tc_create_identity_then_decrypt_it() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let address = hex_account_to_address32(alice).unwrap();

    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    let event = event.unwrap();
    assert_eq!(event.who, api_client.get_signer().unwrap());

    let encrypted_identity = event.identity;
    let decrypted_identity =
        decrypt_identity_with_user_shielding_key(&user_shielding_key, encrypted_identity);
    assert!(decrypted_identity.is_ok());
    assert_eq!(identity, decrypted_identity.unwrap());

    print_passed()
}
