mod index;
mod lit_vc;

#[macro_use]
extern crate lazy_static;

use codec::{Decode, Encode};
use sp_core::{crypto::AccountId32 as AccountId, sr25519};
// use sp_keyring::AccountKeyring;
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

pub type PalletString = String;
// Todo: move this improved enclave definition into a primitives crate in the pallet_teerex repo.
#[derive(Encode, Decode, Clone, PartialEq, sp_core::RuntimeDebug)]
pub struct EnclaveGen<AccountId> {
    pub pubkey: AccountId,
    // FIXME: this is redundant information
    pub mr_enclave: [u8; 32],
    pub timestamp: u64,
    // unix epoch in milliseconds
    pub url: PalletString, // utf8 encoded url
}
pub type Enclave = EnclaveGen<AccountId>;
pub type ApiResult<T> = Result<T, ApiClientError>;

pub fn get_shard() -> u32 {
    let enclave_count: u64 = API
        .get_storage_value("Teerex", "EnclaveCount", None)
        .unwrap()
        .unwrap();

    let enclave: ApiResult<Option<Enclave>> =
        API.get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None);

    // let enclave = API
    // .get_storage_map("Teerex", "EnclaveRegistry", enclave_count)
    // .unwrap();

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
