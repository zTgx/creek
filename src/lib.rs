mod primitives;
pub mod utils;
pub mod ethereum_signature;
pub mod identity_management;
pub mod vc_management;

#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::channel;

use primitives::RsaPublicKeyGenerator;
use rsa::RsaPublicKey;
use sp_core::{crypto::AccountId32 as AccountId, sr25519, Pair};
use kitchensink_runtime::{Block, Header};
use substrate_api_client::{rpc::WsRpcClient, Api, Metadata, PlainTipExtrinsicParams};
use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use crate::primitives::{Enclave, MrEnclave, NODE_PORT, NODE_SERVER_URL};
use sp_runtime::generic::SignedBlock as SignedBlockG;
type SignedBlock = SignedBlockG<Block>;


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

lazy_static! {
    pub static ref LIT_Aes256G_KEY: Vec<u8> = {
        let aes_key = Aes256Gcm::generate_key(&mut OsRng);
        aes_key.to_vec()
    };
}


pub fn get_signer() -> AccountId {
    API.signer_account().unwrap()
}

pub fn get_tee_shielding_pubkey() -> RsaPublicKey {
    let enclave_count: u64 = API
        .get_storage_value("Teerex", "EnclaveCount", None)
        .unwrap()
        .unwrap();

    let enclave: Enclave<AccountId, Vec<u8>> = API
        .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
        .unwrap()
        .unwrap();

    let shielding_key = enclave.shielding_key.unwrap();
    RsaPublicKey::new_with_rsa3072_pubkey(shielding_key)
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

pub fn get_shard_mock() -> MrEnclave {
    [65_u8, 56, 208, 116, 135, 54, 101, 208, 13, 173, 159, 82, 115, 60, 181, 148, 205, 211, 71, 48, 174, 210, 172, 218, 70, 146, 182, 230, 5, 74, 110, 208]
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

pub fn get_extrinsics() {
	let head = API.get_finalized_head().unwrap().unwrap();

	println!("Genesis block: \n {:?} \n", API.get_block_by_num::<Block>(Some(0)).unwrap());

	println!("Finalized Head:\n {} \n", head);

	let h: Header = API.get_header(Some(head)).unwrap().unwrap();
	println!("Finalized header:\n {:?} \n", h);

	let b: SignedBlock = API.get_signed_block(Some(head)).unwrap().unwrap();
	println!("Finalized signed block:\n {:?} \n", b);

	println!("Latest Header: \n {:?} \n", API.get_header::<Header>(None).unwrap());

	println!("Latest block: \n {:?} \n", API.get_block::<Block>(None).unwrap());

	println!("Subscribing to finalized heads");
	let (sender, receiver) = channel();
	API.subscribe_finalized_heads(sender).unwrap();

	for _ in 0..5 {
		let head: Header =
			receiver.recv().map(|header| serde_json::from_str(&header).unwrap()).unwrap();
        let number = head.number;
		println!("Got new Block number {:?}", number);

        let block = API.get_block_by_num::<Block>(Some(number)).unwrap().unwrap();
        block.extrinsics.iter().for_each(|e| {
            
        });
	}
}