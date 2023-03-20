use litentry_test_suit::{
    identity_management::{api::IdentityManagementApi, events::IdentityManagementEventApi},
    primitives::{Address32, Assertion, Identity, SubstrateNetwork},
    utils::generate_user_shielding_key,
    vc_management::{api::VcManagementApi, events::VcManagementEventApi},
    ApiClient,
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
 */
#[test]
fn tc_request_vc_with_20s_identities_or_more() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let user_shielding_key = generate_user_shielding_key();
    api_client.set_user_shielding_key(shard, user_shielding_key);

    // Alice
    let add =
        hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
    let mut y = [0u8; 32];
    y[..32].clone_from_slice(&add);

    let address = Address32::from(y);
    let ciphertext_metadata: Option<Vec<u8>> = None;

    let networks = [
        SubstrateNetwork::Polkadot,
        SubstrateNetwork::Kusama,
        SubstrateNetwork::Litentry,
        SubstrateNetwork::Litmus,
        SubstrateNetwork::Khala,
    ];

    let bob = sr25519::Pair::from_string("//Bob", None).unwrap();
    let bob: Address32 = bob.public().0.into();

    let coc = sr25519::Pair::from_string("//Coc", None).unwrap();
    let coc: Address32 = coc.public().0.into();

    let dod = sr25519::Pair::from_string("//Dod", None).unwrap();
    let dod: Address32 = dod.public().0.into();

    let addresses = [address.clone(), bob, coc, dod];
    networks.iter().for_each(|network| {
        addresses.iter().for_each(|address| {
            let identity = Identity::Substrate {
                network: network.clone(),
                address: address.clone(),
            };
            api_client.create_identity(
                shard,
                address.clone(),
                identity.clone(),
                ciphertext_metadata.clone(),
            );

            println!(">>> Identity: {:?}", identity.clone());
            let event = api_client.wait_event_identity_created();
            println!("<<< event: {:?}", event);
            assert!(event.is_ok());
            assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());
        })
    });

    // Inputs
    let a4 = Assertion::A4(1_u128);
    api_client.request_vc(shard, a4);

    // Wait event
    let event = api_client.wait_event_vc_issued();
    assert!(event.is_ok());
}
