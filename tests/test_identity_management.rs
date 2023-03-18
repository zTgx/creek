use litentry_test_suit::{
    get_shard, get_signer,
    identity_management::{
        api::*,
        events::{
            wait_identity_created_event, wait_set_user_shielding_key_handle_failed_event,
            wait_user_shielding_key_set_event, SetUserShieldingKeyEvent,
            SetUserShieldingKeyHandlingFailedEvent,
        },
    },
    primitives::{Address32, Identity, SubstrateNetwork},
    USER_AES256G_KEY,
};

#[test]
fn tc_set_user_shielding_key() {
    let shard = get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    set_user_shielding_key(shard, aes_key);

    let event = wait_user_shielding_key_set_event();
    let expect_event = SetUserShieldingKeyEvent {
        account: get_signer(),
    };
    assert_eq!(event, expect_event);

    println!(" ✅ tc_set_user_shielding_key");
}

#[test]
fn tc_set_user_shielding_key_faild() {
    let shard = get_shard();
    let aes_key = [0, 1].to_vec();
    set_user_shielding_key(shard, aes_key);

    let event = wait_set_user_shielding_key_handle_failed_event();
    let expect_event = SetUserShieldingKeyHandlingFailedEvent;
    assert_eq!(event, expect_event);

    println!(" ✅ tc_set_user_shielding_key_faild");
}

#[test]
fn tc_create_identity() {
    let shard = get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    set_user_shielding_key(shard, aes_key);

    // Alice
    let add =
        hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap();
    let mut y = [0u8; 32];
    y[..32].clone_from_slice(&add);

    let address = Address32::from(y);
    let network = SubstrateNetwork::Litentry;
    let identity = Identity::Substrate { network, address };
    let ciphertext_metadata: Option<Vec<u8>> = None;

    create_identity(address, identity, ciphertext_metadata);

    let event = wait_identity_created_event();
    assert_eq!(event.who, get_signer());

    println!(" ✅ tc_create_identity");
}
