use super::VC_PALLET_NAME;
use crate::primitives::{
    assertion::Assertion, crypto::AesOutput, vc::ErrorDetail, AccountId, VCIndex,
};
use codec::Decode;
use sp_core::H256;
use substrate_api_client::StaticEvent;

/// VCIssuedEvent
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCIssuedEvent {
    pub account: AccountId,
    pub assertion: Assertion,
    pub index: VCIndex,
    pub vc: AesOutput,
    pub req_ext_hash: H256,
}

impl StaticEvent for VCIssuedEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCIssued";
}

/// VCDisabled
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCDisabledEvent {
    pub account: AccountId,
    pub index: VCIndex,
}

impl StaticEvent for VCDisabledEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCDisabled";
}

/// VCRevoked
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCRevokedEvent {
    pub account: AccountId,
    pub index: VCIndex,
}

impl StaticEvent for VCRevokedEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCRevoked";
}

/// VCRevoked
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct RequestVCFailedEvent {
    pub account: Option<AccountId>,
    pub assertion: Assertion,
    pub detail: ErrorDetail,
    pub req_ext_hash: H256,
}

impl StaticEvent for RequestVCFailedEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "RequestVCFailed";
}

/// Error
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCManagementError;
impl StaticEvent for VCManagementError {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "()";
}
