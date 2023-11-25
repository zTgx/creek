use vc_sdk::utils::di::{decode_nonce, decode_user_shielding_key};

#[test]
fn tc_decode_nonce_works() {
    // RpcReturnValue.value
    // 0x011002000000
    let encoded_nonce = "011002000000";
    let nonce = decode_nonce(encoded_nonce).unwrap();
    assert_eq!(nonce, 2);
}

#[test]
fn tc_decode_user_shielding_key_works() {
    let encoded_user_shielding_key =
        "01848022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12";
    let user_shielding_key = decode_user_shielding_key(encoded_user_shielding_key).unwrap();
    assert_eq!(
        user_shielding_key,
        "22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12".to_string()
    );
}
