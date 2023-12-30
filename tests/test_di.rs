use creek::{
	service::json::RpcReturnValue,
	utils::{hex::FromHexPrefixed, public_api::decode_nonce},
};

#[test]
fn tc_decode_nonce_works() {
	let encoded_nonce = "0x100e0000000000";
	let rpc_return_value = RpcReturnValue::from_hex(encoded_nonce).unwrap();
	let nonce = decode_nonce(&rpc_return_value).unwrap();
	assert_eq!(nonce, 14);
}

#[test]
fn decode_string_works() {
	let encoded = "0x00010100b24f0fd92e2763f0d03a5bc664a333a98673eee678350bf0d677213f7caaccb7";
	let rpc_return_value = RpcReturnValue::from_hex(&encoded).unwrap();
	println!("decoded rpc_return_value: {:#?}", rpc_return_value);
}
