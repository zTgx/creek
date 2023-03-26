use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use substrate_api_client::{ApiResult, RuntimeMetadataPrefixed};

pub mod api;

pub trait SidechainRpc {
    fn rpc_methods(&self) -> ApiResult<Vec<String>>;
    fn system_version(&self) -> ApiResult<String>;
    fn system_name(&self) -> ApiResult<String>;
    fn system_health(&self) -> ApiResult<String>;
    fn state_get_runtime_version(&self) -> ApiResult<String>;
    fn state_get_metadata(&self) -> ApiResult<RuntimeMetadataPrefixed>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SidechainResp {
    pub id: String,
    pub jsonrpc: String,
    pub result: String,
}

fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
    json!({
        "method": method,
        "params": params,
        "jsonrpc": "2.0",
        "id": id.to_string(),
    })
}

fn json_resp(resp: String) -> ApiResult<SidechainResp> {
    let resp: SidechainResp = serde_json::from_str(&resp)?;
    Ok(resp)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
