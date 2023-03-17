use litentry_test_suit::{get_shard, identity_management::api::*, USER_AES256G_KEY};

#[test]
fn xx() {
    let shard = get_shard();
    let aes_key = USER_AES256G_KEY.to_vec();
    set_user_shielding_key(shard, aes_key);
}
