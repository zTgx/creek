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
            PaddingScheme::new_pkcs1v15_encrypt(),
            &shielding_key,
        )
        .expect("failed to encrypt")
}

pub fn set_user_shielding_key() {
    // input: shard / shielding key
    let shard = get_shard();
    // generate user shielding key
    // let shielding_key = Aes256Gcm::generate_key(&mut OsRng);

    // 0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12
    let aes_key = "22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12";
    let aes_key = hex::decode(aes_key).unwrap();
    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);

    println!(">>>> shard: {:?}", shard);
    println!(">>>> encrpted_shielding_key: {:?}", encrpted_shielding_key);

    let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
        API.clone(),
        "IdentityManagement",
        "set_user_shielding_key",
        H256::from(shard),
        encrpted_shielding_key
    );

    println!("[+] Composed Extrinsic:\n {:?}\n", xt);

    let tx_hash = API
        .send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
        .unwrap();
    println!("[+] Transaction got included. Hash: {:?}", tx_hash);
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

// {
// 	pubkey: 2KWuhnGZ6sK1XXkAW2ckDjyQYB3yMTwoEVQpmuXv9h3Wi2fE
// 	mrEnclave: 0x269005ce06bc73c265c67061c2d5a653776d7c3220de6d3e292c5bf3ac3e4600
// 	timestamp: 1,678,534,249,000
// 	url: wss://localhost:2000
// 	shieldingKey: {n:[79,16,27,159,134,112,106,45,127,12,242,48,183,188,136,255,103,46,19,53,121,186,161,213,94,32,193,217,58,7,117,16,220,224,37,43,103,170,116,171,125,143,19,45,229,121,57,52,178,150,108,141,126,159,51,31,206,234,246,36,214,207,87,104,177,81,204,190,159,205,194,197,71,139,187,200,6,179,64,153,105,37,163,160,11,75,53,247,63,35,220,35,128,56,248,139,78,8,57,20,24,56,7,54,99,113,172,31,27,67,111,118,121,212,168,189,192,214,247,121,208,200,207,73,127,49,129,177,175,230,22,160,9,180,233,209,201,168,187,229,118,245,42,237,22,249,228,78,137,135,146,70,51,85,177,67,161,133,17,199,41,32,179,129,74,230,166,62,168,86,117,149,132,210,147,250,26,100,65,111,80,49,12,78,249,232,109,127,33,255,106,144,102,80,190,150,57,147,17,241,54,45,117,8,96,23,208,67,57,134,92,41,231,103,85,17,187,5,21,38,137,33,204,251,162,253,31,54,115,209,104,101,149,40,86,252,90,12,145,147,51,199,151,65,244,177,206,4,145,208,101,99,6,170,48,65,186,36,40,200,18,88,177,53,252,208,88,213,145,193,152,141,187,50,7,79,49,14,22,164,51,152,98,71,131,134,240,96,28,33,87,150,145,243,141,21,2,224,77,161,11,179,26,121,124,192,165,40,122,40,240,60,194,151,76,207,41,63,69,116,157,126,53,226,203,35,57,237,80,114,77,26,124,103,211,228,151,26,156,5,210,135,84,99,185,91,6,128,221,229,96,21,65,134,142,71,223,184,35,67,248,120,88,56,40,222,146,227,87,163,247,28,171,165,213,69,249,223,158,95,106,222,235,195],e:[1,0,0,1]}
// 	vcPubkey: 0x2ee22be32877b7f22c4b631ccf89b834b1f2091fae7e1f3f61c33167ddf3f0ae
// 	sgxMode: Production
// }

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


        let b = BigUint::from_radix_be(&key.n, 256).unwrap();
        let a = BigUint::from_radix_be(&key.e, 256).unwrap();

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
