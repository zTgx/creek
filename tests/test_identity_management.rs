use litentry_test_suit::{
    get_shard, get_signer,
    identity_management::{
        api::*,
        events::{
            wait_set_user_shielding_key_handle_failed_event, wait_user_shielding_key_set_event,
            SetUserShieldingKeyEvent, SetUserShieldingKeyHandlingFailedEvent,
        },
    },
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
