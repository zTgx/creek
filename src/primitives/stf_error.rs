use codec::{Decode, Encode};
use derive_more::Display;

use super::{AccountId, Index, error::ErrorDetail, assertion::Assertion};
pub type Nonce = Index;

#[derive(Debug, Display, PartialEq, Eq, Encode, Decode, Clone)]
pub enum StfError {
	#[codec(index = 0)]
	#[display(fmt = "Insufficient privileges {:?}, are you sure you are root?", _0)]
	MissingPrivileges(AccountId),
	#[codec(index = 1)]
	#[display(fmt = "Valid enclave signer account is required")]
	RequireEnclaveSignerAccount,
	#[codec(index = 2)]
	#[display(fmt = "Error dispatching runtime call. {:?}", _0)]
	Dispatch(String),
	#[codec(index = 3)]
	#[display(fmt = "Not enough funds to perform operation")]
	MissingFunds,
	#[codec(index = 4)]
	#[display(fmt = "Invalid Nonce {:?} != {:?}", _0, _1)]
	InvalidNonce(Nonce, Nonce),
	#[codec(index = 5)]
	StorageHashMismatch,
	#[codec(index = 6)]
	InvalidStorageDiff,
	#[codec(index = 7)]
	InvalidMetadata,
	// litentry
	#[codec(index = 8)]
	#[display(fmt = "LinkIdentityFailed: {:?}", _0)]
	LinkIdentityFailed(ErrorDetail),
	#[codec(index = 9)]
	#[display(fmt = "DeactivateIdentityFailed: {:?}", _0)]
	DeactivateIdentityFailed(ErrorDetail),
	#[codec(index = 10)]
	#[display(fmt = "ActivateIdentityFailed: {:?}", _0)]
	ActivateIdentityFailed(ErrorDetail),
	#[codec(index = 11)]
	#[display(fmt = "RequestVCFailed: {:?} {:?}", _0, _1)]
	RequestVCFailed(Assertion, ErrorDetail),
	#[codec(index = 12)]
	SetScheduledMrEnclaveFailed,
	#[codec(index = 13)]
	#[display(fmt = "SetIdentityNetworksFailed: {:?}", _0)]
	SetIdentityNetworksFailed(ErrorDetail),
	#[codec(index = 14)]
	InvalidAccount,
	#[codec(index = 15)]
	UnclassifiedError,
	#[codec(index = 16)]
	#[display(fmt = "RemovingIdentityFailed: {:?}", _0)]
	RemoveIdentityFailed(ErrorDetail),
	#[codec(index = 17)]
	EmptyIDGraph,
}
