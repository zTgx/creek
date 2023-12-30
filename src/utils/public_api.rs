use super::hex::FromHexPrefixed;
use crate::{
	primitives::{cerror::CError, AccountId, CResult, Index, MrEnclave, ShardIdentifier},
	service::json::{JsonResponse, RpcReturnValue},
};
use basex_rs::{BaseX, ALPHABET_BITCOIN};
use codec::Decode;
use frame_metadata::RuntimeMetadataPrefixed;

pub fn storage_value_key(module_prefix: &str, storage_prefix: &str) -> Vec<u8> {
	let mut bytes = sp_core::twox_128(module_prefix.as_bytes()).to_vec();
	bytes.extend(&sp_core::twox_128(storage_prefix.as_bytes())[..]);
	bytes
}

pub fn mrenclave_to_bs58(mrenclave: &MrEnclave) -> String {
	BaseX::with_alphabet(ALPHABET_BITCOIN).to_bs58(mrenclave)
}

pub fn mrenclave_from_bs58(mrenclave_in_base58: String) -> Option<MrEnclave> {
	BaseX::with_alphabet(ALPHABET_BITCOIN).from_bs58(&mrenclave_in_base58).map(|m| {
		let mut bytes = [0u8; 32];
		bytes[..32].clone_from_slice(&m);
		bytes
	})
}

pub fn remove_whitespace(s: &str) -> String {
	s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn decode_rpc_methods(jsonreponse: &JsonResponse) -> Vec<String> {
	let mut sresult = remove_whitespace(&jsonreponse.result);
	sresult.remove_matches("methods:[");
	sresult.remove_matches("]");

	let mut rpc_methods = vec![];
	let methods: Vec<&str> = sresult.split(',').collect();
	methods.iter().for_each(|m| {
		rpc_methods.push(m.to_string());
	});

	rpc_methods
}

pub fn decode_rpc_return_value(jsonresp: &JsonResponse) -> CResult<RpcReturnValue> {
	RpcReturnValue::from_hex(&jsonresp.result).map_err(CError::HexError)
}

pub fn decode_mr_enclave(rpc_return_value: &RpcReturnValue) -> CResult<MrEnclave> {
	MrEnclave::decode(&mut rpc_return_value.value.as_slice()).map_err(CError::CodecError)
}

pub fn decode_runtime_metadata(
	rpc_return_value: &RpcReturnValue,
) -> CResult<RuntimeMetadataPrefixed> {
	RuntimeMetadataPrefixed::decode(&mut rpc_return_value.value.as_slice())
		.map_err(CError::CodecError)
}

pub fn decode_nonce(rpc_return_value: &RpcReturnValue) -> CResult<Index> {
	Index::decode(&mut rpc_return_value.value.as_slice()).map_err(CError::CodecError)
}

pub fn decode_string(rpc_return_value: &RpcReturnValue) -> CResult<String> {
	String::decode(&mut rpc_return_value.value.as_slice()).map_err(CError::CodecError)
}

pub fn decode_shard_identifier(rpc_return_value: &RpcReturnValue) -> CResult<ShardIdentifier> {
	ShardIdentifier::decode(&mut rpc_return_value.value.as_slice()).map_err(CError::CodecError)
}

pub fn decode_accountid(rpc_return_value: &RpcReturnValue) -> CResult<AccountId> {
	AccountId::decode(&mut rpc_return_value.value.as_slice()).map_err(CError::CodecError)
}
