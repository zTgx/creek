use codec::{Decode, Encode, MaxEncodedLen};
use rsa::{BigUint, RsaPublicKey};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::ConstU32, BoundedVec};

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

// we use 256-bit AES-GCM as user shielding key
pub const USER_SHIELDING_KEY_LEN: usize = 32;
pub const USER_SHIELDING_KEY_NONCE_LEN: usize = 12;
pub const USER_SHIELDING_KEY_TAG_LEN: usize = 16;

// all-in-one struct containing the encrypted ciphertext with user's
// shielding key and other metadata that is required for decryption
//
// by default a postfix tag is used => last 16 bytes of ciphertext is MAC tag
#[derive(Debug, Default, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct AesOutput {
	pub ciphertext: Vec<u8>,
	pub aad: Vec<u8>,
	pub nonce: [u8; USER_SHIELDING_KEY_NONCE_LEN], // IV
}

pub type ParentchainBlockNumber = u32;

/// Ed25519 Signature 2018, W3C, 23 July 2021, https://w3c-ccg.github.io/lds-ed25519-2018
/// May be registered in Linked Data Cryptographic Suite Registry, W3C, 29 December 2020
/// https://w3c-ccg.github.io/ld-cryptosuite-registry
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum ProofType {
	Ed25519Signature2020,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum CredentialType {
	VerifiableCredential,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
	/// ID of the data provider
	pub data_provider_id: u32,
	/// Endpoint of the data provider
	pub data_provider: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
	/// ID of the TEE Worker
	pub id: String,
	pub name: String,
	pub mrenclave: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSubject {
	/// Identifier for the only entity that the credential was issued
	pub id: String,
	pub description: String,
	#[serde(rename = "type")]
	pub types: String,
	/// (Optional) Some externally provided identifiers
	pub tag: Vec<String>,
	/// (Optional) Data source definitions for trusted data providers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<Vec<DataSource>>,
	/// Several sets of assertions.
	/// Each assertion contains multiple steps to describe how to fetch data and calculate the value
	#[serde(skip_deserializing)]
	pub assertions: Vec<AssertionLogic>,
	/// Results of each set of assertions
	pub values: Vec<bool>,
	/// The extrinsic on Parentchain for credential verification purpose
	pub endpoint: String,
}


#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Op {
	#[serde(rename = ">")]
	GreaterThan,
	#[serde(rename = "<")]
	LessThan,
	#[serde(rename = ">=")]
	GreaterEq,
	#[serde(rename = "<=")]
	LessEq,
	#[serde(rename = "==")]
	Equal,
	#[serde(rename = "!=")]
	NotEq,
}

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Eq, TypeInfo, Debug, Clone)]
#[serde(untagged)]
pub enum AssertionLogic {
	Item {
		src: String,
		op: Op,
		dst: String,
	},
	And {
		#[serde(rename = "and")]
		items: Vec<Box<AssertionLogic>>,
	},
	Or {
		#[serde(rename = "or")]
		items: Vec<Box<AssertionLogic>>,
	},
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSchema {
	/// Schema ID that is maintained by Parentchain VCMP
	pub id: String,
	/// The schema type, generally it is
	#[serde(rename = "type")]
	pub types: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	/// The block number when the signature was created
	pub created_block_number: ParentchainBlockNumber,
	/// The cryptographic signature suite that used to generate signature
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	/// Purpose of this proof, generally it is expected as a fixed value, such as 'assertionMethod'
	pub proof_purpose: String,
	/// The digital signature value(signature of hash)
	pub proof_value: String,
	/// The public key from Issuer
	pub verification_method: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
	/// Contexts defines the structure and data types of the credential
	#[serde(rename = "@context")]
	pub context: Vec<String>,
	/// The specific UUID of the credential, it is used for onchain verification
	pub id: String,
	/// Uniquely identifier of the type of the credential
	#[serde(rename = "type")]
	pub types: Vec<CredentialType>,
	/// Assertions claimed about the subjects of the credential
	pub credential_subject: CredentialSubject,
	/// The TEE enclave who issued the credential
	pub issuer: Issuer,
	pub issuance_block_number: ParentchainBlockNumber,
	/// (Optional)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_block_number: Option<ParentchainBlockNumber>,
	/// Digital proof with the signature of Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	pub proof: Option<Proof>,
	#[serde(skip_deserializing)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub credential_schema: Option<CredentialSchema>,
}

pub type Balance = u128;
type MaxStringLength = ConstU32<64>;
pub type ParameterString = BoundedVec<u8, MaxStringLength>;
pub type Network = BoundedVec<u8, MaxStringLength>;
pub type AssertionNetworks = BoundedVec<Network, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
	A1,
	A2(ParameterString),                                   // (guild_id)
	A3(ParameterString, ParameterString, ParameterString), // (guild_id, channel_id, role_id)
	A4(Balance),                                           // (minimum_amount)
	A5(ParameterString, ParameterString),                  // (twitter_account, tweet_id)
	A6,
	A7(Balance),           // (minimum_amount)
	A8(AssertionNetworks), // litentry, litmus, polkadot, kusama, khala, ethereum
	A9,
	A10(Balance), // (minimum_amount)
	A11(Balance), // (minimum_amount)
	A13(u32),     // (Karma_amount) - TODO: unsupported
}