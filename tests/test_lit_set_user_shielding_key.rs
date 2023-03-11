use parachain_test_suit::{get_shard, print_metadata};

#[test]
fn set_user_shielding_key_works() {
    let shard = get_shard();
    assert_eq!(shard, 3u32);
}
