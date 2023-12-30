use super::hex::{remove_whitespace, FromHexPrefixed, JsonResponse};
use crate::primitives::{
	cerror::CError, crypto::RpcReturnValue, AccountId, CResult, Index, MrEnclave, ShardIdentifier,
};
use codec::Decode;
use frame_metadata::RuntimeMetadataPrefixed;

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
