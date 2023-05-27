use codec::Error as CodecError;
use codec::{Decode, Encode};
use rsa::RsaPublicKey;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use substrate_api_client::{std::error::Error as ApiError, ApiResult, RuntimeMetadataPrefixed};

use crate::primitives::{address::Address32, identity::Identity};
use crate::{
    primitives::crypto::{DirectRequestStatus, RpcReturnValue},
    utils::hex::FromHexPrefixed,
};

pub mod api;
pub mod di;
pub mod rpc;

pub trait SidechainRpc {
    fn rpc_methods(&self) -> ApiResult<Vec<String>>;
    fn system_version(&self) -> ApiResult<String>;
    fn system_name(&self) -> ApiResult<String>;
    fn system_health(&self) -> ApiResult<String>;
    fn state_get_runtime_version(&self) -> ApiResult<String>;
    fn state_get_metadata(&self) -> ApiResult<RuntimeMetadataPrefixed>;

    fn author_get_mu_ra_url(&self) -> ApiResult<String>;
    fn author_get_shielding_key(&self) -> ApiResult<RsaPublicKey>;
    fn author_get_untrusted_url(&self) -> ApiResult<String>;
    fn author_pending_extrinsics(&self, shards: Vec<String>) -> ApiResult<Vec<Vec<Vec<u8>>>>;

    fn state_get_storage(
        &self,
        mrenclave_in_base58: String,
        storage_key_in_hex: String,
    ) -> ApiResult<Vec<u8>>;
}

/// storage key in hex
pub fn storage_key_challenge_code(account: &Address32, identity: &Identity) -> String {
    let mut entry_bytes = sp_core::twox_128("IdentityManagement".as_bytes()).to_vec();
    entry_bytes.extend(&sp_core::twox_128("ChallengeCodes".as_bytes())[..]);

    let encoded_account: &[u8] = &account.encode();
    let encoded_identity: &[u8] = &identity.encode();

    // Key1: Blake2_128Concat
    entry_bytes.extend(sp_core::blake2_128(encoded_account));
    entry_bytes.extend(encoded_account);

    // Key2: Blake2_128Concat
    entry_bytes.extend(sp_core::blake2_128(encoded_identity));
    entry_bytes.extend(encoded_identity);

    format!("0x{}", hex::encode(entry_bytes))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SidechainResp {
    pub id: String,
    pub jsonrpc: String,
    pub result: String,
}

#[derive(Clone, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct RpcResponse {
    pub jsonrpc: String,
    pub result: String, // hex encoded RpcReturnValue
    pub id: u32,
}

pub fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
    json!({
        "method": method,
        "params": params,
        "jsonrpc": "2.0",
        "id": id.to_string(),
    })
}

pub fn json_resp(resp: String) -> ApiResult<SidechainResp> {
    let resp: SidechainResp = serde_json::from_str(&resp)?;
    Ok(resp)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
    pub id: i32,
}

impl RpcRequest {
    pub fn compose_jsonrpc_call(
        method: String,
        params: Vec<String>,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(&RpcRequest {
            jsonrpc: "2.0".to_owned(),
            method,
            params,
            id: 1,
        })
    }
}

fn decode_from_rpc_response(json_rpc_response: &str) -> ApiResult<String> {
    let rpc_response: SidechainResp = serde_json::from_str(json_rpc_response)?;
    let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result).map_err(|_| {
        let x = hex::FromHexError::OddLength;
        ApiError::InvalidHexString(x)
    })?;

    let response_message = String::decode(&mut rpc_return_value.value.as_slice())?;
    match rpc_return_value.status {
        DirectRequestStatus::Ok => Ok(response_message),
        _ => {
            let error = CodecError::from("Decode error.");
            Err(ApiError::NodeApi(substrate_api_client::Error::Codec(error)))
        }
    }
}

#[allow(dead_code)]
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
