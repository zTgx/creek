use super::IDENTITY_PALLET_NAME;
use crate::primitives::{crypto::AesOutput, vc::ErrorDetail, AccountId};
use codec::Decode;
use sp_core::H256;
use substrate_api_client::StaticEvent;

/// UserShieldingKeySet
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyEvent {
    pub account: AccountId,
    pub req_ext_hash: H256,
}

impl StaticEvent for SetUserShieldingKeyEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "UserShieldingKeySet";
}

/// SetUserShieldingKeyHandlingFailed
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyHandlingFailedEvent;
impl StaticEvent for SetUserShieldingKeyHandlingFailedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "SetUserShieldingKeyHandlingFailed";
}

/// IdentityCreated
#[derive(Decode, Debug)]
pub struct IdentityCreatedEvent {
    pub who: AccountId,
    pub identity: AesOutput,
    pub code: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for IdentityCreatedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityCreated";
}

/// IdentityRemoved
#[derive(Decode, Debug)]
pub struct IdentityRemovedEvent {
    pub who: AccountId,
    pub identity: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for IdentityRemovedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityRemoved";
}

/// IdentityVerified
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct IdentityVerifiedEvent {
    pub account: AccountId,
    pub identity: AesOutput,
    pub id_graph: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for IdentityVerifiedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityVerified";
}

/// DelegateeAdded
#[derive(Decode, Debug)]
pub struct DelegateeAddedEvent {
    pub account: AccountId,
}

impl StaticEvent for DelegateeAddedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "DelegateeAdded";
}

/// UnexpectedMessage
#[derive(Decode, Debug)]
pub struct UnexpectedMessageEvent;
impl StaticEvent for UnexpectedMessageEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "UnexpectedMessage";
}

/// Error
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct IdentityManagementError;
impl StaticEvent for IdentityManagementError {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "()";
}

/// IdentityCreated
#[derive(Decode, Debug)]
pub struct CreateIdentityFailedEvent {
    pub account: Option<AccountId>,
    pub detail: ErrorDetail,
    pub req_ext_hash: H256,
}

impl StaticEvent for CreateIdentityFailedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityCreated";
}
