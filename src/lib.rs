#![recursion_limit = "256"]
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
use substrate_api_client::{rpc::WsRpcClient, Api, ApiResult, PlainTipExtrinsicParams, XtStatus};

#[cfg(feature = "local")]
const NODE_URL: &str = "ws://127.0.0.1:9944";
#[cfg(feature = "local")]
const WORKER_URL: &str = "ws://127.0.0.1:2000";

#[cfg(feature = "staging")]
const NODE_URL: &str = "wss://tee-staging.litentry.io:443";
#[cfg(feature = "staging")]
const WORKER_URL: &str = "ws://127.0.0.1:2000";

#[cfg(feature = "prod2")]
const NODE_URL: &str = "wss://tee-internal.litentry.io:443";
#[cfg(feature = "prod2")]
const WORKER_URL: &str = "wss://tee-internal.litentry.io:2000";

// Default to `local` worker mode when no cargo features are set.
#[cfg(not(any(feature = "local", feature = "staging", feature = "prod2")))]
const NODE_URL: &str = "ws://127.0.0.1:9944";
#[cfg(not(any(feature = "local", feature = "staging", feature = "prod2")))]
const WORKER_URL: &str = "ws://127.0.0.1:2000";

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
    pub fn new_with_signer(signer: P) -> ApiResult<Self> {
        // env_logger::init();

        let client = WsRpcClient::new(NODE_URL);
        let api = ApiType::new(client).map(|api| api.set_signer(signer))?;

        let sidechain = SidechainRpcClient::new(WORKER_URL);

        println!("[+] Parachain rpc : {}", NODE_URL);
        println!("[+] Sidechain rpc : {}", WORKER_URL);

        Ok(ApiClient { api, sidechain })
    }

    pub fn get_signer(&self) -> Option<AccountId> {
        self.api.signer_account()
    }

    pub fn send_extrinsic(&self, xthex_prefixed: String) {
        match self.api.send_extrinsic(xthex_prefixed, XtStatus::InBlock) {
            Ok(tx_hash) => match tx_hash {
                Some(tx_hash) => {
                    println!(" ✅ Transaction got included. Hash: {:?}", tx_hash);
                }
                None => {
                    println!(" ❌ Transaction None");
                }
            },
            Err(e) => {
                println!(" ❌ Transaction error : {:?}", e);
            }
        }
    }
}
