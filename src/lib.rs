#![feature(string_remove_matches)]

pub mod api_client_patch;
pub mod identity_management;
pub mod primitives;
pub mod ra;
pub mod sidechain;
pub mod utils;
pub mod vc_management;

use sidechain::rpc::SidechainRpcClient;
use sp_core::{crypto::AccountId32 as AccountId, Pair};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
    rpc::WsRpcClient, Api, PlainTipExtrinsicParams, PlainTipExtrinsicParamsBuilder, XtStatus,
};

const _NODE_URL: &str = "ws://127.0.0.1:9944";
const _PROD_NODE_URL: &str = "wss://tee-staging.litentry.io:443";
const WORKER_URL: &str = "wss://localhost:2000";

const ACCOUNT_SEED_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
pub type ApiType<P> = Api<P, WsRpcClient, PlainTipExtrinsicParams>;

#[derive(Clone)]
pub struct ApiClient<P>
where
    P: Pair,
{
    pub api: ApiType<P>,
    pub sidechain: SidechainRpcClient,
}

impl<P> ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    pub fn new_with_signer(signer: P) -> Self {
        let client = WsRpcClient::new(_NODE_URL);
        let api = ApiType::new(client)
            .map(|api| api.set_signer(signer))
            .unwrap();

        let sidechain = SidechainRpcClient::new(WORKER_URL);

        ApiClient { api, sidechain }
    }

    pub fn get_signer(&self) -> Option<AccountId> {
        self.api.signer_account()
    }

    pub fn update_api(&mut self, tx_params: PlainTipExtrinsicParamsBuilder) {
        let updated_api = self.api.clone().set_extrinsic_params_builder(tx_params);
        self.api = updated_api;
    }

    pub fn send_extrinsic(&self, xthex_prefixed: String) {
        let tx_hash = self
            .api
            .send_extrinsic(xthex_prefixed, XtStatus::InBlock)
            .unwrap();
        println!(" ✅ Transaction got included. Hash: {:?}", tx_hash);
    }
}
