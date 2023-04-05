use super::rpc::SidechainRpcClientTrait;
use super::{decode_from_rpc_response, json_req, remove_whitespace, SidechainRpc};
use crate::primitives::crypto::{DirectRequestStatus, RpcReturnValue, RsaPublicKeyGenerator};
use crate::utils::hex::FromHexPrefixed;
use crate::{sidechain::json_resp, ApiClient};
use codec::{Decode, Error as CodecError};
use rsa::RsaPublicKey;
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{ApiResult, DecodeError, Error as ApiError, RuntimeMetadataPrefixed};

impl<P> SidechainRpc for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    /*
    supported rpc methods:
    [
        ✅ "author_getMuRaUrl",
        ✅ "author_getShieldingKey",
        ✅ "author_getUntrustedUrl",
        ✅ "author_pendingExtrinsics",
        "author_pendingTrustedCallsFor",
        "author_submitAndWatchExtrinsic",
        "author_submitExtrinsic",

        ❌ "chain_subscribeAllHeads",

        "state_executeGetter",
        ✅ "state_getMetadata",
        ✅ "state_getRuntimeVersion",
        ✅ "state_getStorage",

        ✅ "system_health",
        ✅ "system_name",
        ✅ "system_version"
    ]
     */
    fn rpc_methods(&self) -> ApiResult<Vec<String>> {
        let jsonreq = json_req("rpc_methods", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let resp = json_resp(resp)?;
        let mut sresult = remove_whitespace(&resp.result);
        sresult.remove_matches("methods:[");
        sresult.remove_matches("]");

        let mut supported_methods = vec![];
        let methods: Vec<&str> = sresult.split(',').collect();
        methods.iter().for_each(|m| {
            supported_methods.push(m.to_string());
        });

        Ok(supported_methods)
    }

    /*
    { id: "1", jsonrpc: "2.0", result: "hello, world" }
     */
    fn system_version(&self) -> ApiResult<String> {
        let jsonreq = json_req("system_version", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn system_name(&self) -> ApiResult<String> {
        let jsonreq = json_req("system_name", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn system_health(&self) -> ApiResult<String> {
        let jsonreq = json_req("system_health", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn state_get_runtime_version(&self) -> ApiResult<String> {
        let jsonreq = json_req("state_getRuntimeVersion", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn state_get_metadata(&self) -> ApiResult<RuntimeMetadataPrefixed> {
        let jsonreq = json_req("state_getMetadata", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let rpc_response = json_resp(resp)?;
        let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
            .map_err(|e| ApiError::Other(format!("{:?}", e)))?;

        Ok(
            RuntimeMetadataPrefixed::decode(&mut rpc_return_value.value.as_slice()).map_err(
                |_| {
                    let error = CodecError::from("Decode RuntimeMetadataPrefixed error");
                    ApiError::DecodeValue(DecodeError::CodecError(error))
                },
            )?,
        )
    }

    /// {"id":"1","jsonrpc":"2.0","result":"0x3c386c6f63616c686f73743a333434330000"}
    /// "localhost:3443"
    fn author_get_mu_ra_url(&self) -> ApiResult<String> {
        let jsonreq = json_req("author_getMuRaUrl", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let response_message = decode_from_rpc_response(&resp)?;
        Ok(response_message)
    }

    fn author_get_shielding_key(&self) -> ApiResult<RsaPublicKey> {
        let jsonreq = json_req("author_getShieldingKey", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let shielding_pubkey_string = decode_from_rpc_response(&resp)?;
        Ok(
            RsaPublicKey::new_with_rsa3072_pubkey(shielding_pubkey_string.as_bytes().to_vec())
                .map_err(|e| ApiError::Other(format!("Get author shielding key error: {:?}", e)))?,
        )
    }

    /// ws://localhost:3000
    fn author_get_untrusted_url(&self) -> ApiResult<String> {
        let jsonreq = json_req("author_getUntrustedUrl", [0_u8; 0], 1);
        let resp = self.sidechain.request(jsonreq)?;
        let response_message = decode_from_rpc_response(&resp)?;
        Ok(response_message)
    }

    /// shards: Base58 format
    fn author_pending_extrinsics(&self, shards: Vec<String>) -> ApiResult<Vec<Vec<Vec<u8>>>> {
        let jsonreq = json_req("author_pendingExtrinsics", shards, 1);
        let resp = self.sidechain.request(jsonreq)?;
        let rpc_response = json_resp(resp)?;

        let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
            .map_err(|e| ApiError::Other(format!("{:?}", e)))?;

        Ok(
            Vec::<Vec<Vec<u8>>>::decode(&mut rpc_return_value.value.as_slice()).map_err(|_| {
                let error = CodecError::from("Decode RuntimeMetadataPrefixed error");
                ApiError::DecodeValue(DecodeError::CodecError(error))
            })?,
        )
    }

    fn state_get_storage(
        &self,
        mrenclave_in_base58: String,
        storage_key_in_hex: String,
    ) -> ApiResult<Vec<u8>> {
        let jsonreq = json_req(
            "state_getStorage",
            [mrenclave_in_base58, storage_key_in_hex],
            1,
        );
        let resp = self.sidechain.request(jsonreq)?;
        let rpc_response = json_resp(resp)?;

        let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
            .map_err(|e| ApiError::Other(format!("{:?}", e)))?;
        match rpc_return_value.status {
            DirectRequestStatus::Ok => Ok(rpc_return_value.value),
            _ => Ok(Default::default()),
        }
    }
}
