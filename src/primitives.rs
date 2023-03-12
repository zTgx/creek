use codec::{Decode, Encode};
use rsa::{BigUint, RsaPublicKey};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

pub const NODE_SERVER_URL: &str = "ws://127.0.0.1";
pub const NODE_PORT: &str = "9944";

#[derive(
    Serialize, Deserialize, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo,
)]
pub struct Rsa3072Pubkey {
    pub n: Vec<u8>,
    pub e: Vec<u8>,
}

pub trait RsaPublicKeyGenerator {
    type Input;

    fn new_with_rsa3072_pubkey(shielding_key: Self::Input) -> RsaPublicKey;
}

impl RsaPublicKeyGenerator for RsaPublicKey {
    type Input = Vec<u8>;

    fn new_with_rsa3072_pubkey(shielding_key: Self::Input) -> RsaPublicKey {
        let key: Rsa3072Pubkey = serde_json::from_slice(&shielding_key).unwrap();
        let b = BigUint::from_radix_le(&key.n, 256).unwrap();
        let a = BigUint::from_radix_le(&key.e, 256).unwrap();

        RsaPublicKey::new(b, a).unwrap()
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
