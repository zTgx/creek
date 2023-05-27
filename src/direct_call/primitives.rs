use scale_info::TypeInfo;
use sp_core::ConstU32;
use sp_core::MaxEncodedLen;
use sp_core::H256;
use sp_core::{Decode, Encode};
use sp_runtime::BoundedVec;

/// Index of a transaction in the chain.
pub type Index = u32;
pub type ShardIdentifier = H256;
pub type SidechainBlockNumber = u64;

use crate::primitives::assertion::Assertion;
use crate::primitives::USER_SHIELDING_KEY_LEN;
pub type UserShieldingKeyType = [u8; USER_SHIELDING_KEY_LEN];

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
