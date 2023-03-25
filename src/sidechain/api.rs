use super::{json_req, remove_whitespace, SidechainRpc};
use crate::{sidechain::json_resp, ApiClient, SidechainRpcClientTrait};
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::ApiResult;

impl<P> SidechainRpc for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn rpc_methods(&self) -> ApiResult<Vec<String>> {
        let jsonreq = json_req("rpc_methods", [0_u8; 0], 1);
        let resp = self.sidechain.get_sidechain_request(jsonreq)?;
        let resp = json_resp(resp)?;
        let mut sresult = remove_whitespace(&resp.result);
        sresult.remove_matches("methods: [");
        sresult.remove_matches("]");

        let mut supported_methods = vec![];
        let methods: Vec<&str> = sresult.split(',').collect();
        methods.iter().for_each(|m| {
            supported_methods.push(m.to_string());
        });

        Ok(supported_methods)
    }
}
