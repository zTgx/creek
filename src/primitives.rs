use codec::{Decode, Encode, MaxEncodedLen};
use rsa::{BigUint, RsaPublicKey};
use scale_info::TypeInfo;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sp_core::H256;
use sp_core::{ecdsa, ed25519, sr25519};
use sp_runtime::{traits::ConstU32, BoundedVec};
// use kitchensink_runtime::{Block, Header, AccountId};
pub use sp_core::crypto::AccountId32 as AccountId;

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

impl AesOutput {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.ciphertext.len() + self.aad.len() + USER_SHIELDING_KEY_NONCE_LEN
    }
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

pub type IdentityString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Address20([u8; 20]);

impl AsRef<[u8; 20]> for Address20 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl From<[u8; 20]> for Address20 {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Address32([u8; 32]);
impl AsRef<[u8; 32]> for Address32 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for Address32 {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, Hash, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SubstrateNetwork {
    Polkadot,
    Kusama,
    Litentry,
    Litmus,
    Khala,
    TestNet,
}

impl SubstrateNetwork {
    /// get the ss58 prefix, see https://github.com/paritytech/ss58-registry/blob/main/ss58-registry.json
    pub fn ss58_prefix(&self) -> u16 {
        match self {
            Self::Polkadot => 0,
            Self::Kusama => 2,
            Self::Litentry => 31,
            Self::Litmus => 131,
            Self::Khala => 30,
            Self::TestNet => 13,
        }
    }

    pub fn from_ss58_prefix(prefix: u16) -> Self {
        match prefix {
            0 => Self::Polkadot,
            2 => Self::Kusama,
            31 => Self::Litentry,
            131 => Self::Litmus,
            30 => Self::Khala,
            _ => Self::TestNet,
        }
    }
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, Hash, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EvmNetwork {
    Ethereum,
    BSC,
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Web2Network {
    Twitter,
    Discord,
    Github,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Identity {
    Substrate {
        network: SubstrateNetwork,
        address: Address32,
    },
    Evm {
        network: EvmNetwork,
        address: Address20,
    },
    Web2 {
        network: Web2Network,
        address: IdentityString,
    },
}

pub type ValidationString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IdentityMultiSignature {
    /// An Ed25519 signature.
    Ed25519(ed25519::Signature),
    /// An Sr25519 signature.
    Sr25519(sr25519::Signature),
    /// An ECDSA/SECP256k1 signature.
    Ecdsa(ecdsa::Signature),
    /// An ECDSA/keccak256 signature. An Ethereum signature. hash message with keccak256
    Ethereum(EthereumSignature),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TwitterValidationData {
    pub tweet_id: ValidationString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DiscordValidationData {
    pub channel_id: ValidationString,
    pub message_id: ValidationString,
    pub guild_id: ValidationString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Web3CommonValidationData {
    pub message: ValidationString, // or String if under std
    pub signature: IdentityMultiSignature,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Web2ValidationData {
    Twitter(TwitterValidationData),
    Discord(DiscordValidationData),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Web3ValidationData {
    Substrate(Web3CommonValidationData),
    Evm(Web3CommonValidationData),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ValidationData {
    Web2(Web2ValidationData),
    Web3(Web3ValidationData),
}

pub const CHALLENGE_CODE_SIZE: usize = 16;
pub type ChallengeCode = [u8; CHALLENGE_CODE_SIZE];

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Status {
    Active,
    Disabled,
    // Revoked, // commented out for now, we can delete the VC entry when revoked
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct VCContext {
    // To be discussed: shall we make it public?
    // pros: easier for the user to disable/revoke VCs, we'll need the AccountId to verify
    //       the owner of VC. An alternative is to store such information within TEE.
    // cons: this information is then public, everyone knows e.g. ALICE owns VC ID 1234 + 4321
    // It's not bad though as it helps to verify the ownership of VC
    pub subject: AccountId,
    // hash of the VC, computed via blake2_256
    pub hash: H256,
    // status of the VC
    pub status: Status,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub struct EthereumSignature(pub [u8; 65]);

impl TryFrom<&[u8]> for EthereumSignature {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() == 65 {
            let mut inner = [0u8; 65];
            inner.copy_from_slice(data);
            Ok(EthereumSignature(inner))
        } else {
            Err(())
        }
    }
}

impl Serialize for EthereumSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self))
    }
}

impl<'de> Deserialize<'de> for EthereumSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let signature_hex = hex::decode(String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("{:?}", e)))?;
        EthereumSignature::try_from(signature_hex.as_ref())
            .map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}

impl AsRef<[u8; 65]> for EthereumSignature {
    fn as_ref(&self) -> &[u8; 65] {
        &self.0
    }
}

impl AsRef<[u8]> for EthereumSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}
