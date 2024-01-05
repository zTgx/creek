use creek::{primitives::vc::VCContext, utils::hex::FromHexPrefixed};

#[test]
fn decode_vc_context_works() {
	let encoded = "0x645d79a63bd3b0b47ceba0a4c9824d7d8e7a1627c67f11765c3a70ad691e14f706040779df561d21d15e8a3fcc984eb7ca5987a3cfe2f92ca9920a4fcdb9b3eea625f200";
	let rpc_return_value = VCContext::from_hex(&encoded).unwrap();
	println!("decoded rpc_return_value: {:#?}", rpc_return_value);

	// VCContext {
	// 	subject: 645d79a63bd3b0b47ceba0a4c9824d7d8e7a1627c67f11765c3a70ad691e14f7 (5ELJQwUo...),
	// 	assertion: A8(
	// 		BoundedVec(
	// 			[
	// 				Ethereum,
	// 			],
	// 			128,
	// 		),
	// 	),
	// 	hash: 0x79df561d21d15e8a3fcc984eb7ca5987a3cfe2f92ca9920a4fcdb9b3eea625f2,
	// 	status: Active,
	// }
}
