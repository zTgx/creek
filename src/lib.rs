mod lit_vc;

#[macro_use]
extern crate lazy_static;

use codec::{Decode, Encode};
use sp_core::{crypto::AccountId32 as AccountId, sr25519};
// use sp_keyring::AccountKeyring;
use scale_info::TypeInfo;
use std::env;
use substrate_api_client::ApiClientError;
use substrate_api_client::{rpc::WsRpcClient, Api, AssetTipExtrinsicParams, Metadata};

const NODE_SERVER_URL: &str = "NODE_SERVER_URL";
const NODE_PORT: &str = "NODE_PORT";
const DEFAULT_NODE_SERVER_URL: &str = "ws://127.0.0.1";
const DEFAULT_NODE_PORT: &str = "9944";

lazy_static! {
    pub static ref API: Api::<sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams> = {
        let node_server = env::var(NODE_SERVER_URL).unwrap_or(DEFAULT_NODE_SERVER_URL.to_string());
        let node_port = env::var(NODE_PORT).unwrap_or(DEFAULT_NODE_PORT.to_string());
        let url = format!("{}:{}", node_server, node_port);
        let client = WsRpcClient::new(&url);

        Api::<sr25519::Pair, WsRpcClient, AssetTipExtrinsicParams>::new(client).unwrap()
    };
}

pub fn print_metadata() {
    let meta = Metadata::try_from(API.get_metadata().unwrap()).unwrap();
    meta.print_overview();
}

pub fn get_total_issuance() {
    let result: u128 = API
        .get_storage_value("Balances", "TotalIssuance", None)
        .unwrap()
        .unwrap();
    println!("[+] TotalIssuance is {}", result);
}

pub type ApiResult<T> = Result<T, ApiClientError>;
pub type MrEnclave = [u8; 32];

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Default, sp_core::RuntimeDebug)]
pub struct SgxEnclaveMetadata {
    pub quote: Vec<u8>,
    pub quote_sig: Vec<u8>,
    pub quote_cert: Vec<u8>,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
    Debug,
    Production,
}

impl Default for SgxBuildMode {
    fn default() -> Self {
        SgxBuildMode::Production
    }
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Enclave<PubKey, Url> {
    pub pubkey: PubKey, // FIXME: this is redundant information
    pub mr_enclave: MrEnclave,
    // Todo: make timestamp: Moment
    pub timestamp: u64,                 // unix epoch in milliseconds
    pub url: Url,                       // utf8 encoded url
    pub shielding_key: Option<Vec<u8>>, // JSON serialised enclave shielding key
    pub vc_pubkey: Option<Vec<u8>>,
    pub sgx_mode: SgxBuildMode,
    // pub sgx_metadata: SgxEnclaveMetadata,
}

pub fn get_shard() -> u32 {
    let enclave_count: u64 = API
        .get_storage_value("Teerex", "EnclaveCount", None)
        .unwrap()
        .unwrap();

    let enclave: Enclave<AccountId, Vec<u8>> = API
        .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
        .unwrap()
        .unwrap();

    println!("[+] enclave: {:?}", enclave);

    // let proof = API
    //     .get_storage_value_proof("Balances", "TotalIssuance", None)
    //     .unwrap();
    // println!("[+] StorageValueProof: {:?}", proof);

    // // get StorageMap
    // let account = AccountKeyring::Alice.public();

    // let result: AccountInfo = API
    //     .get_storage_map("System", "Account", account, None)
    //     .unwrap()
    //     .or_else(|| Some(AccountInfo::default()))
    //     .unwrap();
    // println!("[+] AccountInfo for Alice is {:?}", result);

    // // get StorageMap key prefix
    // let result = API.get_storage_map_key_prefix("System", "Account").unwrap();
    // println!("[+] key prefix for System Account map is {:?}", result);

    // get Alice's AccountNonce with api.get_nonce()
    // let signer = AccountKeyring::Alice.pair();
    // API.signer = Some(signer);
    // println!("[+] Alice's Account Nonce is {}", API.get_nonce().unwrap());

    3u32
}
