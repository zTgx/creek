mod lit_set_user_shielding_key;
mod lit_vc;

#[macro_use]
extern crate lazy_static;

use aes_gcm::{aead::OsRng, Aes256Gcm, KeyInit};
use codec::{Decode, Encode};
use rsa::PaddingScheme;
pub use rsa::{BigUint, PublicKey, PublicKeyParts, RsaPublicKey};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sp_core::Pair;
use sp_core::H256;
use sp_core::{crypto::AccountId32 as AccountId, sr25519};
use std::env;

use substrate_api_client::{compose_extrinsic, ApiClientError, UncheckedExtrinsicV4, XtStatus};
use substrate_api_client::{rpc::WsRpcClient, Api, Metadata, PlainTipExtrinsicParams};

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

        let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

        Api::<sr25519::Pair, WsRpcClient, PlainTipExtrinsicParams>::new(client)
            .map(|api| api.set_signer(alice))
            .unwrap()
    };
}

pub fn encrypt_with_tee_shielding_pubkey(shielding_key: &[u8]) -> Vec<u8> {
    let tee_shielding_pubkey: RsaPublicKey = get_tee_shielding_pubkey();
    let mut rng = rand::thread_rng();
    tee_shielding_pubkey
        .encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), shielding_key)
        .expect("failed to encrypt")
}

pub fn set_user_shielding_key() {
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);
    let encrpted_shielding_key = encrypt_with_tee_shielding_pubkey(&aes_key);
    let shard = get_shard();

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

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
    Debug,

    #[default]
    Production,
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

    let shielding_key = enclave.shielding_key.unwrap();

    #[derive(
        Serialize,
        Deserialize,
        Encode,
        Decode,
        Default,
        Clone,
        PartialEq,
        Eq,
        sp_core::RuntimeDebug,
        TypeInfo,
    )]
    struct Rsa3072Pubkey {
        pub n: Vec<u8>,
        pub e: Vec<u8>,
    }

    {
        let key: Rsa3072Pubkey = serde_json::from_slice(&shielding_key).unwrap();
        println!("Rsa3072Pubkey : {:?}", key);

        let b = BigUint::from_radix_le(&key.n, 256).unwrap();
        let a = BigUint::from_radix_le(&key.e, 256).unwrap();

        RsaPublicKey::new(b, a).unwrap()
    }
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
}
