use super::{json_req, remove_whitespace, SidechainRpc};
use crate::hex::{FromHexPrefixed};
use crate::primitives::RpcReturnValue;
// use crate::rpc_error::Error;
use crate::{sidechain::json_resp, ApiClient, SidechainRpcClientTrait};
use codec::Decode;
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{ApiResult, Error};
use substrate_api_client::RuntimeMetadataPrefixed;

impl<P> SidechainRpc for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    /*
    supported rpc methods:
    [
        "author_getMuRaUrl", 
        "author_getShieldingKey", 
        "author_getUntrustedUrl", 
        "author_pendingExtrinsics", 
        "author_pendingTrustedCallsFor", 
        "author_submitAndWatchExtrinsic", 
        "author_submitExtrinsic", 
        
        "chain_subscribeAllHeads", 
    
        "state_executeGetter", 
        "state_getMetadata", 
        "state_getRuntimeVersion", 
        "state_getStorage", 

        "system_health", 
        "system_name", 
        "system_version"
    ]
     */
    fn rpc_methods(&self) -> ApiResult<Vec<String>> {
        let jsonreq = json_req("rpc_methods", [0_u8; 0], 1);
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
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
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn system_name(&self) -> ApiResult<String> {
        let jsonreq = json_req("system_name", [0_u8; 0], 1);
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn system_health(&self) -> ApiResult<String> {
        let jsonreq = json_req("system_health", [0_u8; 0], 1);
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result)
    }

    fn state_get_runtime_version(&self) -> ApiResult<String> {
        let jsonreq = json_req("state_getRuntimeVersion", [0_u8; 0], 1);
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
        let resp = json_resp(resp)?;
        Ok(resp.result) 
    }

    fn state_get_metadata(&self) -> ApiResult<RuntimeMetadataPrefixed> {
        let jsonreq = json_req("state_getMetadata", [0_u8; 0], 1);
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
        let rpc_response = json_resp(resp)?;

		// let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
		// 	.map_err(|e| Error::Other(Box::new(e)))?;

        let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result).unwrap();

		// Decode Metadata.
		let x = RuntimeMetadataPrefixed::decode(&mut rpc_return_value.value.as_slice());
        Ok(x.unwrap())
    }
}
