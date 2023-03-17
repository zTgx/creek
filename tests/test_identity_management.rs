use litentry_test_suit::{identity_management::api::*, get_shard, USER_AES256G_KEY};

#[test]
fn xx() {
    let shard = get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    set_user_shielding_key(shard, aes_key);
}
