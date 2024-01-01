use creek::{
	primitives::vc::VCContext,
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

#[test]
fn decode_vc_context_works() {
	let encoded = "0xc66b31eb137a002a1fbe5324fff546d8485b7ac23b5b1879738b5ada19e4a2f415426e5058f169317ed1fb7baa63959bb4b83fc170567a959cfe7c7fa59608ba4c00";
	let rpc_return_value = VCContext::from_hex(&encoded).unwrap();
	println!("decoded rpc_return_value: {:#?}", rpc_return_value);
}
