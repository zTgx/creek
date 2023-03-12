mod primitives;

pub mod lit_set_user_shielding_key;
pub mod lit_vc;

#[macro_use]
extern crate lazy_static;

use crate::primitives::{Enclave, MrEnclave, NODE_PORT, NODE_SERVER_URL};
use sp_core::{crypto::AccountId32 as AccountId, sr25519, Pair};
use substrate_api_client::{rpc::WsRpcClient, Api, Metadata, PlainTipExtrinsicParams};

lazy_static! {
    pub static ref API: Api::<sr25519::Pair, WsRpcClient, PlainTipExtrinsicParams> = {
        let url = format!("{}:{}", NODE_SERVER_URL, NODE_PORT);
        let client = WsRpcClient::new(&url);

        let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
        Api::<sr25519::Pair, WsRpcClient, PlainTipExtrinsicParams>::new(client)
            .map(|api| api.set_signer(alice))
            .unwrap()
    };
}

pub fn get_shard() -> MrEnclave {
    let enclave_count: u64 = API
        .get_storage_value("Teerex", "EnclaveCount", None)
        .unwrap()
        .unwrap();

    let enclave: Enclave<AccountId, Vec<u8>> = API
        .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
        .unwrap()
        .unwrap();

    enclave.mr_enclave
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
