use crate::primitives::{cerror::CError, CResult};
use substrate_api_client::{ac_primitives::AssetRuntimeConfig, rpc::TungsteniteRpcClient, Api};

pub struct ParachainRpcClient {
	pub api: Api<AssetRuntimeConfig, TungsteniteRpcClient>,
}

impl ParachainRpcClient {
	pub fn new(endpoint: &str) -> CResult<Self> {
		let client = TungsteniteRpcClient::new(endpoint, 100).map_err(|_| CError::APIError)?;
		let api = Api::<AssetRuntimeConfig, _>::new(client).unwrap();
		Ok(Self { api })
	}
}
