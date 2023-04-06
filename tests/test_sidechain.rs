use litentry_api_client::{
    api_client_patch::{event::SubscribeEventPatch, parachain::ParachainPatch},
    identity_management::{events::IdentityCreatedEvent, IdentityManagementApi},
    primitives::identity::{Identity, SubstrateNetwork},
    sidechain::{storage_key_challenge_code, SidechainRpc},
    utils::{
        address::pubkey_to_address32,
        crypto::{decrypt_challage_code_with_user_shielding_key, generate_user_shielding_key},
        enclave::mrenclave_to_bs58,
        print_passed,
    },
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_sidechain_rpc_methods_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let methods = api_client.rpc_methods().unwrap();
    println!("Sidechain supported methods: {:?}", methods);
}

#[test]
fn tc_sidechain_system_version_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let system_version = api_client.system_version().unwrap();
    println!("Sidechain system_version: {}", system_version);
}

#[test]
fn tc_sidechain_system_name_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let system_name = api_client.system_name().unwrap();
    println!("Sidechain system_name: {}", system_name);
}

#[test]
fn tc_sidechain_system_health_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let system_health = api_client.system_health().unwrap();
    println!("Sidechain system_health: {}", system_health);
}

#[test]
fn tc_sidechain_state_get_runtime_version_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let runtime_version = api_client.state_get_runtime_version().unwrap();
    println!("Sidechain runtime_version: {}", runtime_version);
}

#[test]
fn tc_sidechain_state_get_metadata_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let metadata = api_client.state_get_metadata().unwrap();
    println!("Sidechain metadata: {:?}", metadata);
}

#[test]
fn tc_sidechain_author_ge_mu_ra_url_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let mu_ra_url = api_client.author_get_mu_ra_url().unwrap();
    println!("Sidechain mu_ra_url: {:?}", mu_ra_url);
}

#[test]
fn tc_sidechain_author_get_shielding_key_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();
    let shielding_key_from_worker = api_client.author_get_shielding_key().unwrap();
    let tee_shielding_pubkey_from_parachain = api_client.get_tee_shielding_pubkey().unwrap();

    assert_eq!(
        shielding_key_from_worker,
        tee_shielding_pubkey_from_parachain
    );
}

#[test]
fn tc_sidechain_author_get_untrusted_url_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let untrusted_url = api_client.author_get_untrusted_url().unwrap();
    println!("Sidechain untrusted_url: {:?}", untrusted_url);
}

#[test]
fn tc_sidechain_author_pending_extrinsics_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let shard_in_base58 = mrenclave_to_bs58(&shard);
    let pending_extrinsics = api_client
        .author_pending_extrinsics(vec![shard_in_base58])
        .unwrap();
    println!("Sidechain pending_extrinsics: {:?}", pending_extrinsics);
}

#[test]
fn tc_sidechain_challenge_code_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let shard_in_base58 = mrenclave_to_bs58(&shard);
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    {
        let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
        let address = pubkey_to_address32(alice).unwrap();

        let network = SubstrateNetwork::Litentry;
        let identity = Identity::Substrate {
            network,
            address: address.clone(),
        };
        let ciphertext_metadata: Option<Vec<u8>> = None;

        api_client.create_identity(&shard, &address, &identity, &ciphertext_metadata);

        let event = api_client.wait_event::<IdentityCreatedEvent>();
        assert!(event.is_ok());
        let event = event.unwrap();
        assert_eq!(event.who, api_client.get_signer().unwrap());
        let encrypted_challenge_code = event.code;
        let challenge_code_from_parachain = decrypt_challage_code_with_user_shielding_key(
            &user_shielding_key,
            encrypted_challenge_code,
        )
        .unwrap();

        let challenge_code_key = storage_key_challenge_code(&address, &identity);
        println!("challenge_code_key: {}", challenge_code_key);

        // Waiting sidechain finalize the block
        std::thread::sleep(std::time::Duration::from_secs(3));

        let challenge_code_from_worker = api_client
            .state_get_storage(shard_in_base58, challenge_code_key)
            .unwrap();
        println!(
            "Sidechain challenge_code_from_worker: {:?}",
            challenge_code_from_worker
        );

        assert_eq!(
            challenge_code_from_parachain.to_vec(),
            challenge_code_from_worker
        );
    }

    print_passed();
}
