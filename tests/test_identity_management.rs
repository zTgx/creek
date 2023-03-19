use litentry_test_suit::{
    identity_management::{
        api::*,
        events::{
            IdentityManagementEventApi, SetUserShieldingKeyEvent,
            SetUserShieldingKeyHandlingFailedEvent,
        },
    },
    primitives::{Address32, Identity, SubstrateNetwork},
    utils::print_passed,
    ApiClient, USER_AES256G_KEY,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_set_user_shielding_key() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

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
    let aes_key = [0, 1].to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    let event = api_client.wait_event_set_user_shielding_key_handle_failed();
    let expect_event = SetUserShieldingKeyHandlingFailedEvent;

    assert!(event.is_ok());
    assert_eq!(event.unwrap(), expect_event);

    print_passed();
}

#[test]
fn tc_create_identity() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice);

    let shard = api_client.get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    api_client.set_user_shielding_key(shard, aes_key);

    // Alice
    let add =
        hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
    let mut y = [0u8; 32];
    y[..32].clone_from_slice(&add);

    let address = Address32::from(y);
    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    api_client.create_identity(shard, address, identity, ciphertext_metadata);

    let event = api_client.wait_event_identity_created();
    assert!(event.is_ok());
    assert_eq!(event.unwrap().who, api_client.get_signer().unwrap());

    print_passed();
}
