use std::time::SystemTime;

use codec::Encode;
use litentry_api_client::{
    identity_management::{
        events::{IdentityCreatedEvent, IdentityManagementEventApi, IdentityVerifiedEvent},
        xtbuilder::IdentityManagementXtBuilder,
        IdentityManagementApi,
    },
    primitives::{
        address::Address32,
        assertion::{Assertion, IndexingNetworks, ParameterString},
        identity::{Identity, SubstrateNetwork, ValidationData},
    },
    utils::{
        address::{create_n_random_sr25519_address, pubkey_to_address32},
        crypto::{
            decrypt_challage_code_with_user_shielding_key,
            decrypt_id_graph_with_user_shielding_key, decrypt_identity_with_user_shielding_key,
            generate_user_shielding_key,
        },
        identity::ValidationDataBuilder,
        print_passed,
    },
    vc_management::{events::VcManagementEventApi, VcManagementApi},
    ApiClient, ApiClientPatch, SubscribeEventPatch,
};
use sp_core::{sr25519, Pair};

/**
 * Including the corner case of everything
 *
 * Format:
 * 1. A detailed description of this corner case
 * 2. Implement this part of the verification code from scratch
 *
 */

/*
Description:
https://github.com/litentry/litentry-parachain/issues/1468

How long does it take to generate a VC with 20+ identities? What about 50 identities?
About 350 secs in this way
 */
#[test]
fn tc_request_vc_with_20s_identities_or_more_one_single_thread() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [
        SubstrateNetwork::Polkadot,
        SubstrateNetwork::Kusama,
        SubstrateNetwork::Litentry,
        SubstrateNetwork::Litmus,
        SubstrateNetwork::Khala,
    ];

    let identity_address = create_n_random_sr25519_address(6);
    let mut created_identity_idex = 0;

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };
            api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);
            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

            created_identity_idex += 1;
        })
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 created {} identities in one single thread using {} secs!",
        created_identity_idex, elapsed_secs
    );

    assert_eq!(created_identity_idex, 30);
}

#[test]
fn tc_request_vc_based_on_more_than_30_identities() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [
        SubstrateNetwork::Polkadot,
        SubstrateNetwork::Kusama,
        SubstrateNetwork::Litentry,
        SubstrateNetwork::Litmus,
        SubstrateNetwork::Khala,
    ];

    let identity_address = create_n_random_sr25519_address(6);
    let mut created_identity_idx = 0;

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };
            api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    &user_shielding_key,
                    encrypted_challenge_code,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(&shard, &identity, &vdata);

                let event = api_client.wait_event_identity_verified();
                assert!(event.is_ok());
            }

            created_identity_idx += 1;
        })
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 created {} identities in one single thread using {} secs!",
        created_identity_idx, elapsed_secs
    );

    assert_eq!(created_identity_idx, 30);

    {
        println!("  [+] Start testing and apply for all assertions based on 30 dentities. ");

        let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
        let api_client = ApiClient::new_with_signer(alice);

        let shard = api_client.get_shard();
        let user_shielding_key = generate_user_shielding_key();
        api_client.set_user_shielding_key(&shard, &user_shielding_key);

        println!("  [+] Start testing and apply for all assertions based on 30 dentities. ");

        let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
        let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
        let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
        let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();
        let networks = IndexingNetworks::with_bounded_capacity(1);

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
        let assertion_names = vec!["A1", "A2", "A3", "A4", "A6", "A7", "A8", "A10", "A11"];

        assertions.into_iter().enumerate().for_each(|(idx, assertion)| {
            let assertion_name = assertion_names[idx];
            println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion {}. <<<<<<<<<<<<<<<<<<<<<<<< ", assertion_name);

            let now = SystemTime::now();

            api_client.request_vc(&shard, &assertion);
            let event = api_client.wait_event_vc_issued();
            assert!(event.is_ok());
            assert_eq!(event.unwrap().account, api_client.get_signer().unwrap());

            let elapsed_secs = now.elapsed().unwrap().as_secs();
            println!(
                " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue {} took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
                assertion_name, elapsed_secs
            );
        });
    }
}

#[test]
fn tc_create_all_substrate_network_then_request_vc() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [
        SubstrateNetwork::Polkadot,
        SubstrateNetwork::Kusama,
        SubstrateNetwork::Litentry,
        SubstrateNetwork::Litmus,
        SubstrateNetwork::Khala,
        SubstrateNetwork::TestNet,
    ];

    let identity_address = create_n_random_sr25519_address(1);
    let mut created_identity_idex = 0;

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };

            api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);
            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

            created_identity_idex += 1;
        })
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 created {} identities in one single thread using {} secs!",
        created_identity_idex, elapsed_secs
    );

    assert_eq!(created_identity_idex, 6);

    {
        println!("  [+] Start testing and apply for assertions based on 6 dentities. ");

        let balance = ParameterString::try_from("1.001".as_bytes().to_vec()).unwrap();

        let a4 = Assertion::A4(balance);

        let assertions = vec![a4];
        let assertion_names = vec!["A4"];

        assertions.into_iter().enumerate().for_each(|(idx, assertion)| {
            let assertion_name = assertion_names[idx];
            println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion {}. <<<<<<<<<<<<<<<<<<<<<<<< ", assertion_name);

            let now = SystemTime::now();

            api_client.request_vc(&shard, &assertion);

            let event = api_client.wait_event_vc_issued();
            assert!(event.is_ok());
            assert_eq!(event.unwrap().account, api_client.get_signer().unwrap());

            let elapsed_secs = now.elapsed().unwrap().as_secs();
            println!(
                " 🚩 >>>>>>>>>>>>>>>>>>>>>>> Issue {} took {} secs <<<<<<<<<<<<<<<<<<<<<<<< ",
                assertion_name, elapsed_secs
            );
        });
    }
}

#[test]
fn tc_create_10s_verified_identities() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [SubstrateNetwork::Litentry, SubstrateNetwork::Litmus];

    let address_len = 5;
    let identites_len = networks.len() * address_len;

    let identity_address = create_n_random_sr25519_address(address_len as u32);
    let mut created_identity_idx = 0;

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };
            api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    &user_shielding_key,
                    encrypted_challenge_code,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(&shard, &identity, &vdata);

                let event = api_client.wait_event_identity_verified();
                assert!(event.is_ok());
            }

            created_identity_idx += 1;
        })
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 created {} identities in one single thread using {} secs!",
        created_identity_idx, elapsed_secs
    );

    assert_eq!(created_identity_idx, identites_len);
}

#[test]
fn tc_create_more_than_20_identities_and_check_idgraph_size() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [SubstrateNetwork::Litentry, SubstrateNetwork::Litmus];

    let address_len = 11;
    let identites_len = networks.len() * address_len;

    let identity_address = create_n_random_sr25519_address(address_len as u32);
    let mut created_identity_idx = 0;
    let mut id_graph_size = 0;

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };
            api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    &user_shielding_key,
                    encrypted_challenge_code,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(&shard, &identity, &vdata);

                let event = api_client.wait_event_identity_verified();
                assert!(event.is_ok());
                let event = event.unwrap();
                assert_eq!(event.account, api_client.get_signer().unwrap());

                id_graph_size += event.id_graph.len();
            }

            created_identity_idx += 1;
        })
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 Creating {} identities stats: \n
            >>> Took {} secs \n
            >>> id_graph {} bytes \n",
        created_identity_idx, elapsed_secs, id_graph_size
    );

    assert_eq!(created_identity_idx, identites_len);
}

/*
The returned IDGraph in the IdentityVerified event will contain the current IDGraph of all linked identities, capped by 20 (see IDGRAPH_MAX_LEN in trusted_call.rs). That means you only need to check it once when the last identity is linked.

Examples:

link and verify 1st identity => the IDGraph in the IdentityVerified event contains 1 identity
link and verify 2nd identity => the IDGraph in the IdentityVerified event contains 2 identities
...
link and verify 20th identity => the IDGraph in the IdentityVerified event contains 20 identities
link and verify 21st identity => the IDGraph in the IdentityVerified event contains 20 identities (latest, the first identity is excluded)
..
link and verify 100th identity => the IDGraph in the IdentityVerified event contains 20 identities (81st - 100th)

Mainly to see:

if IDGraph will be capped
how large is the returned IDGraph

 */
#[test]
fn tc_batch_all_create_more_than_100_identities_and_check_idgraph_size() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [SubstrateNetwork::Litentry, SubstrateNetwork::Litmus];

    let address_len = 50;
    let identites_len = networks.len() * address_len;

    let identity_address = create_n_random_sr25519_address(address_len as u32);
    let mut created_identity_idx = 0;
    let mut id_graph_size = 0;
    let mut batch_calls = vec![];
    let mut key_pairs = vec![];
    let mut all_created_identities = vec![];
    let mut all_verifed_identities = vec![];
    let mut the_last_verified = vec![];

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            created_identity_idx += 1;

            key_pairs.push(pair.clone());

            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };

            {
                batch_calls.push(
                    api_client
                        .build_extrinsic_create_identity(
                            &shard,
                            &alice,
                            &identity,
                            &ciphertext_metadata,
                        )
                        .function,
                );

                if batch_calls.len() == 10 {
                    api_client.send_extrinsic(api_client.batch_all(&batch_calls).hex_encode());

                    batch_calls.clear();
                }
            }
        });
    });

    let events_arr: Vec<IdentityCreatedEvent> = api_client.collect_events(identites_len);
    let mut verified_calls = vec![];

    // Verify identity
    for (indx, event) in events_arr.iter().enumerate() {
        let pair = &key_pairs[indx];

        let encrypted_challenge_code = &event.code;
        let challenge_code = decrypt_challage_code_with_user_shielding_key(
            &user_shielding_key,
            encrypted_challenge_code.clone(),
        )
        .unwrap();
        let identity =
            decrypt_identity_with_user_shielding_key(&user_shielding_key, event.identity.clone())
                .unwrap();
        all_created_identities.push(identity.clone());

        let vdata =
            ValidationData::build_vdata_substrate(&pair, &alice, &identity, &challenge_code);

        {
            verified_calls.push(
                api_client
                    .build_extrinsic_verify_identity(&shard, &identity, &vdata)
                    .function,
            );

            if verified_calls.len() == 10 {
                api_client.send_extrinsic(api_client.batch_all(&verified_calls).hex_encode());

                verified_calls.clear();
            }
        }
    }

    let events_arr: Vec<IdentityVerifiedEvent> = api_client.collect_events(identites_len);
    events_arr.iter().enumerate().for_each(|(indx, event)| {
        all_verifed_identities.push(event.clone());

        if indx == created_identity_idx - 1 {
            the_last_verified.push(event.clone());
        }
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    assert_eq!(created_identity_idx, identites_len);

    let left = all_verifed_identities.last().unwrap();
    let right = the_last_verified.last().unwrap();
    assert_eq!(left, right);

    println!(
        ">>>🚩Creating {} identities stats: \n",
        created_identity_idx
    );
    println!(">>>took {} secs", elapsed_secs);

    let decrypted_id_graph =
        decrypt_id_graph_with_user_shielding_key(&user_shielding_key, right.id_graph.clone())
            .unwrap();

    assert_eq!(decrypted_id_graph.len(), 20);
    decrypted_id_graph.iter().for_each(|(identity, context)| {
        assert!(context.is_verified);
        assert!(all_created_identities[identites_len - 20..].contains(&identity));

        let identity_size = identity.encode().len();
        let context_size = context.encode().len();
        let size = identity_size + context_size;

        id_graph_size += size;
    });
    println!(
        ">>>last IdentityVerifiedEvent id_graph(Vec<(Identity, IdentityContext).len() == 20) <{}> bytes \n",
        id_graph_size
    );

    print_passed()
}

#[test]
fn tc_create_litentry_litmus_rococo_verified_identities() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(&shard, &user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [
        SubstrateNetwork::Litentry,
        SubstrateNetwork::LitentryRococo,
        SubstrateNetwork::Litmus,
    ];

    let address_len = 1;
    let identites_len = networks.len() * address_len;

    let identity_address = create_n_random_sr25519_address(address_len as u32);
    let mut created_identity_idx = 0;

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };
            api_client.create_identity(&shard, &alice, &identity, &ciphertext_metadata);

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    &user_shielding_key,
                    encrypted_challenge_code,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(&shard, &identity, &vdata);

                let event = api_client.wait_event_identity_verified();
                assert!(event.is_ok());
            }

            created_identity_idx += 1;
        })
    });

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 created {} identities in one single thread using {} secs!",
        created_identity_idx, elapsed_secs
    );

    assert_eq!(created_identity_idx, identites_len);
}
