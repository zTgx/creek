use std::{sync::mpsc::channel, time::SystemTime};

use litentry_test_suit::{
    identity_management::{api::IdentityManagementApi, events::IdentityManagementEventApi},
    primitives::{
        Address32, Assertion, AssertionNetworks, Identity, ParameterString, SubstrateNetwork,
        ValidationData,
    },
    utils::{
        create_n_random_sr25519_address, decrypt_challage_code_with_user_shielding_key,
        generate_user_shielding_key, hex_account_to_address32, ValidationDataBuilder,
    },
    vc_management::{api::VcManagementApi, events::VcManagementEventApi},
    ApiClient,
};
use sp_core::{sr25519, Pair};
use threadpool::ThreadPool;

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
    api_client.set_user_shielding_key(shard, user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();
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
            api_client.create_identity(shard, alice, identity.clone(), ciphertext_metadata.clone());
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
fn tc_request_vc_with_20s_identities_or_more_parallelise() {
    // FIXME: NOT DONE YET
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(shard, user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();
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

    let n_workers = 10;
    let n_jobs = 30;
    let pool = ThreadPool::new(n_workers);
    // Synchronized with a channel
    let (tx, rx) = channel();

    let nonce = api_client.clone().api.get_nonce().unwrap_or(0u32);

    let started_timestamp = SystemTime::now();
    networks.iter().for_each(|network| {
        identity_address.iter().for_each(|pair| {
            let address: Address32 = pair.public().0.into();
            let identity = Identity::Substrate {
                network: network.clone(),
                address,
            };

            let tx = tx.clone();
            let api_client = api_client.clone();
            let identity = identity.clone();
            let ciphertext_metadata = ciphertext_metadata.clone();

            pool.execute(move || {
                api_client.create_identity_offline(
                    nonce,
                    shard,
                    alice,
                    identity.clone(),
                    ciphertext_metadata.clone(),
                );

                let event = api_client.wait_event_identity_created();
                assert!(event.is_ok());

                let event = event.unwrap();
                assert_eq!(event.who, api_client.get_signer().unwrap());

                tx.send(event)
                    .expect("channel will be there waiting for the pool");
            });

            created_identity_idex += 1;
        })
    });

    let count = rx.iter().take(n_jobs).count();

    let elapsed_secs = started_timestamp.elapsed().unwrap().as_secs();
    println!(
        " 🚩 created {} identities in multi-thread using {} secs!",
        created_identity_idex, elapsed_secs
    );

    assert_eq!(created_identity_idex, count);
}

#[test]
fn tc_request_vc_based_on_more_than_30_identities() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(shard, user_shielding_key.clone());

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();
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
            api_client.create_identity(shard, alice, identity.clone(), ciphertext_metadata.clone());

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    encrypted_challenge_code,
                    &user_shielding_key,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(shard, &identity, vdata);

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
        api_client.set_user_shielding_key(shard, user_shielding_key);

        println!("  [+] Start testing and apply for all assertions based on 30 dentities. ");

        let guild_id = ParameterString::try_from("guild_id".as_bytes().to_vec()).unwrap();
        let channel_id = ParameterString::try_from("channel_id".as_bytes().to_vec()).unwrap();
        let role_id = ParameterString::try_from("role_id".as_bytes().to_vec()).unwrap();
        let balance = 10_u128;
        let networks = AssertionNetworks::with_bounded_capacity(1);

        let a1 = Assertion::A1;
        let a2 = Assertion::A2(guild_id.clone());
        let a3 = Assertion::A3(guild_id.clone(), channel_id.clone(), role_id.clone());
        let a4 = Assertion::A4(balance);
        let a6 = Assertion::A6;
        let a7 = Assertion::A7(balance);
        let a8 = Assertion::A8(networks);
        let a10 = Assertion::A10(balance);
        let a11 = Assertion::A11(balance);

        let assertions = vec![a1, a2, a3, a4, a6, a7, a8, a10, a11];
        let assertion_names = vec!["A1", "A2", "A3", "A4", "A6", "A7", "A8", "A10", "A11"];

        assertions.into_iter().enumerate().for_each(|(idx, assertion)| {
            let assertion_name = assertion_names[idx];
            println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion {}. <<<<<<<<<<<<<<<<<<<<<<<< ", assertion_name);

            let now = SystemTime::now();

            api_client.request_vc(shard, assertion);
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
    api_client.set_user_shielding_key(shard, user_shielding_key);

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();
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

            api_client.create_identity(shard, alice, identity.clone(), ciphertext_metadata.clone());
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

        let a4 = Assertion::A4(10);

        let assertions = vec![a4];
        let assertion_names = vec!["A4"];

        assertions.into_iter().enumerate().for_each(|(idx, assertion)| {
            let assertion_name = assertion_names[idx];
            println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion {}. <<<<<<<<<<<<<<<<<<<<<<<< ", assertion_name);

            let now = SystemTime::now();

            api_client.request_vc(shard, assertion);

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
    api_client.set_user_shielding_key(shard, user_shielding_key.clone());

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();
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
            api_client.create_identity(shard, alice, identity.clone(), ciphertext_metadata.clone());

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    encrypted_challenge_code,
                    &user_shielding_key,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(shard, &identity, vdata);

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

/*
https://github.com/litentry/litentry-parachain/issues/1487

Mainly to see:

if IDGraph will be capped
how large is the returned IDGraph

 */
#[test]
fn tc_create_more_than_20_identities_and_check_idgraph_size() {
    let alice_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice_pair.clone());

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(shard, user_shielding_key.clone());

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = hex_account_to_address32(alice).unwrap();
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
            api_client.create_identity(shard, alice, identity.clone(), ciphertext_metadata.clone());

            let event = api_client.wait_event_identity_created();
            assert!(event.is_ok());
            let event = event.unwrap();
            assert_eq!(event.who, api_client.get_signer().unwrap());

            // Verify identity
            {
                let encrypted_challenge_code = event.code;
                let challenge_code = decrypt_challage_code_with_user_shielding_key(
                    encrypted_challenge_code,
                    &user_shielding_key,
                )
                .unwrap();

                let vdata = ValidationData::build_vdata_substrate(
                    &pair,
                    &alice,
                    &identity,
                    &challenge_code,
                );
                api_client.verify_identity(shard, &identity, vdata);

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
