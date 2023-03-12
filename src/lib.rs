mod lit_vc;

#[macro_use]
extern crate lazy_static;

use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use codec::{Decode, Encode};
use rsa::PaddingScheme;
pub use rsa::{BigUint, PublicKey, PublicKeyParts, RsaPublicKey};
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32 as AccountId, sr25519};
// use sp_keyring::AccountKeyring;
use hex;
use serde_json;
use sp_core::Pair;
use sp_core::H256;
use std::env;
use substrate_api_client::{compose_extrinsic, ApiClientError, UncheckedExtrinsicV4, XtStatus};
use substrate_api_client::{
    rpc::WsRpcClient, Api, AssetTipExtrinsicParams, Metadata, PlainTipExtrinsicParams,
};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::channel;
use hex::FromHex;




const NODE_SERVER_URL: &str = "NODE_SERVER_URL";
const NODE_PORT: &str = "NODE_PORT";
const DEFAULT_NODE_SERVER_URL: &str = "ws://127.0.0.1";
const DEFAULT_NODE_PORT: &str = "9944";

lazy_static! {
    pub static ref API: Api::<sr25519::Pair, WsRpcClient, PlainTipExtrinsicParams> = {
        let node_server = env::var(NODE_SERVER_URL).unwrap_or(DEFAULT_NODE_SERVER_URL.to_string());
        let node_port = env::var(NODE_PORT).unwrap_or(DEFAULT_NODE_PORT.to_string());
        let url = format!("{}:{}", node_server, node_port);
        let client = WsRpcClient::new(&url);

        // let alice = AccountKeyring::Alice.pair();
        let alice = sr25519::Pair::from_string(&format!("//{}", <&'static str>::from("Alice")), None).unwrap();

        Api::<sr25519::Pair, WsRpcClient, PlainTipExtrinsicParams>::new(client)
            .map(|api| api.set_signer(sr25519::Pair::from(alice)))
            .unwrap()
    };
}

pub fn encrypt_with_tee_shielding_pubkey(shielding_key: &[u8]) -> Vec<u8> {
    use sha2::Sha256;

    let tee_shielding_pubkey: RsaPublicKey = get_tee_shielding_pubkey();

    // encrypt with public key
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(
            &mut rng,
            PaddingScheme::new_oaep::<Sha256>(),
            &shielding_key,
        )
        .expect("failed to encrypt")
}

pub fn set_user_shielding_key() {
    // input: shard / shielding key
    let shard = get_shard();
    // generate user shielding key
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);
    println!(">>> aes_key: {:?}", aes_key);

    // 0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12
    // let aes_key = "22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12";
    // let aes_key = hex::decode(aes_key).unwrap();
    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);
    // let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&shielding_key);

    println!(">>>> shard: {:?}", shard);
    println!(">>>> encrpted_shielding_key: {:?}", encrpted_shielding_key);

    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "set_user_shielding_key",
        H256::from(shard),
        encrpted_shielding_key.to_vec()
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);

    // println!("Subscribe to events");
	// let (events_in, events_out) = channel();
	// API.subscribe_events(events_in).unwrap();

    // {
    //     for _ in 0..5 {
    //         let event_str = events_out.recv().unwrap();
    //         println!("event_str: {}", event_str);
            
    //     }
    // }


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

pub fn get_tee_shielding_pubkey() -> rsa::RsaPublicKey {
    let enclave_count: u64 = API
        .get_storage_value("Teerex", "EnclaveCount", None)
        .unwrap()
        .unwrap();

    let enclave: Enclave<AccountId, Vec<u8>> = API
        .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
        .unwrap()
        .unwrap();

    // println!("[+] enclave: {:?}", enclave);

    // RsaPublicKey::try_from(enclave.shielding_key.unwrap())
    let shielding_key = enclave.shielding_key.unwrap();

    // println!("shielding_key: {:?}", shielding_key);
    let key: serde_json::Value = serde_json::from_slice(&shielding_key).unwrap();

    #[derive(Serialize, Deserialize, Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
    struct xx {
        pub n: Vec<u8>,
        pub e: Vec<u8>,
    };

    {
        // let key: serde_json::Value = serde_json::from_slice(&shielding_key).unwrap();
        // println!("key >>> {:#?}", key["e"].as_array().unwrap());
    
        let key: xx = serde_json::from_slice(&shielding_key).unwrap();
        println!("xx : {:?}", key);


        let b = BigUint::from_radix_le(&key.n, 256).unwrap();
        let a = BigUint::from_radix_le(&key.e, 256).unwrap();

        RsaPublicKey::new(b, a).unwrap()
    }

    // let n: serde_json::Value = key["n"].clone();
    // println!("n: {:?}", n.to_string());
    // let e: serde_json::Value = key["e"].clone();
    // println!("e: {:?}", e.to_string());

    // let x = [
    //     131, 195, 242, 12, 9, 64, 113, 154, 112, 165, 41, 137, 118, 106, 77, 174, 128, 234, 48,
    //     194, 44, 167, 233, 131, 7, 123, 105, 30, 36, 101, 204, 6, 17, 190, 39, 79, 106, 65, 248,
    //     133, 236, 245, 68, 175, 168, 17, 134, 155, 248, 198, 128, 6, 106, 87, 253, 142, 250, 254,
    //     194, 108, 55, 247, 82, 31, 118, 180, 220, 68, 60, 202, 244, 122, 65, 178, 146, 46, 175,
    //     244, 1, 130, 190, 138, 211, 123, 54, 78, 167, 75, 106, 226, 87, 22, 89, 52, 40, 115, 35,
    //     105, 126, 172, 41, 85, 135, 245, 238, 164, 231, 29, 195, 152, 49, 200, 145, 23, 45, 214,
    //     255, 67, 192, 110, 243, 46, 14, 41, 142, 114, 111, 37, 203, 211, 82, 2, 117, 97, 195, 37,
    //     52, 174, 183, 45, 196, 50, 76, 165, 98, 53, 193, 59, 191, 160, 224, 200, 161, 125, 120,
    //     148, 70, 146, 112, 220, 82, 45, 95, 182, 229, 48, 224, 185, 182, 127, 98, 67, 140, 150,
    //     207, 73, 216, 216, 230, 82, 54, 147, 160, 219, 234, 169, 254, 157, 224, 235, 158, 119, 209,
    //     14, 50, 131, 141, 202, 4, 32, 73, 246, 175, 144, 224, 79, 192, 53, 226, 36, 157, 32, 184,
    //     59, 159, 150, 116, 77, 76, 80, 188, 251, 208, 160, 140, 248, 154, 242, 67, 247, 165, 210,
    //     166, 104, 101, 55, 74, 194, 83, 246, 102, 20, 55, 223, 192, 193, 141, 67, 90, 84, 147, 123,
    //     48, 116, 207, 145, 180, 123, 81, 84, 187, 213, 172, 210, 128, 166, 251, 54, 181, 73, 27,
    //     119, 38, 219, 250, 91, 252, 246, 220, 169, 168, 159, 217, 143, 11, 212, 69, 62, 249, 114,
    //     109, 195, 59, 168, 39, 204, 47, 220, 39, 130, 23, 17, 146, 67, 5, 254, 168, 146, 4, 229,
    //     44, 236, 217, 153, 250, 32, 10, 95, 185, 66, 22, 154, 86, 36, 41, 168, 248, 62, 108, 101,
    //     129, 32, 4, 199, 176, 198, 121, 232, 163, 124, 70, 70, 237, 248, 216, 215, 103, 209, 67,
    //     82, 76, 31, 2, 219, 194, 140, 223, 180, 181, 32, 0, 167, 238, 13, 144, 156, 176, 219, 117,
    //     33, 34, 223, 7, 81, 14, 198, 120, 68, 224, 5, 205, 127, 202, 148,
    // ];
    // let e = [1, 0, 0, 1];
    // let b = BigUint::from_radix_be(&x, 256).unwrap();
    // let a = BigUint::from_radix_be(&e, 256).unwrap();
    // println!("b bits: {}", b.bits());
    // // println!("to hex: {:?}", b.to_str_radix(16));

    // // let key: RsaPublicKey = serde_json::from_slice(&shielding_key).unwrap();

    // let key = RsaPublicKey::new(b, a).unwrap();

    // key
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

    // println!("[+] enclave: {:?}", enclave);

    enclave.mr_enclave

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
}
