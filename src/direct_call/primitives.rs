use scale_info::TypeInfo;
use sp_core::{ConstU32, Decode, Encode, MaxEncodedLen, H256};
use sp_runtime::BoundedVec;

/// Index of a transaction in the chain.
pub type Index = u32;
pub type ShardIdentifier = H256;
pub type SidechainBlockNumber = u64;

use crate::primitives::{assertion::Assertion, identity::Identity, USER_SHIELDING_KEY_LEN};

use super::types::{AccountId, KeyPair};
pub type UserShieldingKeyType = [u8; USER_SHIELDING_KEY_LEN];
use sp_runtime::{traits::Verify, MultiSignature};
pub type Signature = MultiSignature;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Request {
	pub shard: ShardIdentifier,
	pub cyphertext: Vec<u8>,
}

// Identity Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum IMPError {
	// errors when executing individual error
	SetUserShieldingKeyFailed(ErrorDetail),
	CreateIdentityFailed(ErrorDetail),
	RemoveIdentityFailed(ErrorDetail),
	VerifyIdentityFailed(ErrorDetail),
	// scheduled encalve import error
	ImportScheduledEnclaveFailed,

	// should be unreached, but just to be on the safe side
	// we should classify the error if we ever get this
	UnclassifiedError(ErrorDetail),
}

pub type MaxStringLength = ConstU32<100>;
pub type ErrorString = BoundedVec<u8, MaxStringLength>;

// enum to reflect the error detail from TEE-worker processing
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum ErrorDetail {
	// error when importing the parentchain blocks and executing indirect calls
	ImportError,
	// the direct or indirect request comes from an unauthorised signer
	UnauthorisedSigner,
	// generic error when executing STF, the `ErrorString` should indicate the actual reason
	StfError(ErrorString),
	// error when sending stf request to the receiver fails
	SendStfRequestFailed,
	// error when the challenge code can not be found
	ChallengeCodeNotFound,
	// error when the user shielding key can not be found
	UserShieldingKeyNotFound,
	// generic parse error, can be caused by UTF8/JSON serde..
	ParseError,
	// errors when communicating with data provider, e.g. HTTP error
	DataProviderError(ErrorString),
	InvalidIdentity,
	WrongWeb2Handle,
	UnexpectedMessage,
	WrongSignatureType,
	VerifySubstrateSignatureFailed,
	VerifyEvmSignatureFailed,
	RecoverEvmAddressFailed,
}
// Verified Credential(VC) Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum VCMPError {
	RequestVCFailed(Assertion, ErrorDetail),
	// should be unreached, but just to be on the safe side
	// we should classify the error if we ever get this
	UnclassifiedError(ErrorDetail),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Getter {
	public(PublicGetter),
	trusted(TrustedGetterSigned),
}

impl From<PublicGetter> for Getter {
	fn from(item: PublicGetter) -> Self {
		Getter::public(item)
	}
}

impl From<TrustedGetterSigned> for Getter {
	fn from(item: TrustedGetterSigned) -> Self {
		Getter::trusted(item)
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PublicGetter {
	some_value,
	nonce(AccountId),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct TrustedGetterSigned {
	pub getter: TrustedGetter,
	pub signature: Signature,
}

impl TrustedGetterSigned {
	pub fn new(getter: TrustedGetter, signature: Signature) -> Self {
		TrustedGetterSigned { getter, signature }
	}

	pub fn verify_signature(&self) -> bool {
		self.signature
			.verify(self.getter.encode().as_slice(), self.getter.sender_account())
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedGetter {
	free_balance(AccountId),
	reserved_balance(AccountId),
	#[cfg(feature = "evm")]
	evm_nonce(AccountId),
	#[cfg(feature = "evm")]
	evm_account_codes(AccountId, H160),
	#[cfg(feature = "evm")]
	evm_account_storages(AccountId, H160, H256),
	// litentry
	user_shielding_key(AccountId),
	id_graph(AccountId),
	challenge_code(AccountId, Identity),
	id_graph_stats(AccountId),
}

impl TrustedGetter {
	pub fn sender_account(&self) -> &AccountId {
		match self {
			TrustedGetter::free_balance(sender_account) => sender_account,
			TrustedGetter::reserved_balance(sender_account) => sender_account,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_nonce(sender_account) => sender_account,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_codes(sender_account, _) => sender_account,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_storages(sender_account, ..) => sender_account,
			// litentry
			TrustedGetter::user_shielding_key(account) => account,
			TrustedGetter::id_graph(account) => account,
			TrustedGetter::challenge_code(account, _) => account,
			TrustedGetter::id_graph_stats(account) => account,
		}
	}

	pub fn sign(&self, pair: &KeyPair) -> TrustedGetterSigned {
		let signature = pair.sign(self.encode().as_slice());
		TrustedGetterSigned { getter: self.clone(), signature }
	}
}
