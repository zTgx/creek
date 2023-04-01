use super::VC_PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput, Assertion, VCIndex},
    ApiClient,
};
use codec::Decode;
use sp_core::Pair;
use sp_core::H256;
use sp_runtime::{MultiSignature, MultiSigner};
use std::sync::mpsc::channel;
use substrate_api_client::{ApiResult, StaticEvent};

// FIXME: WAIT MULTIPLE EVENTS WHEN USING BATCH ALL
pub trait VcManagementEventApi {
    fn wait_event_vc_issued(&self) -> ApiResult<VCIssuedEvent>;
    fn wait_event_vc_disabled(&self) -> ApiResult<VCDisabledEvent>;
    fn wait_event_vc_revoked(&self) -> ApiResult<VCRevokedEvent>;
}

pub trait VcManagementErrorApi {
    fn wait_error(&self) -> ApiResult<VCManagementError>;
}

impl<P> VcManagementEventApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_event_vc_issued(&self) -> ApiResult<VCIssuedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_issued_event: ApiResult<VCIssuedEvent> = self.api.wait_for_event(&events_out);
        vc_issued_event
    }

    fn wait_event_vc_disabled(&self) -> ApiResult<VCDisabledEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_disabled_event: ApiResult<VCDisabledEvent> = self.api.wait_for_event(&events_out);
        vc_disabled_event
    }

    fn wait_event_vc_revoked(&self) -> ApiResult<VCRevokedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_disabled_event: ApiResult<VCRevokedEvent> = self.api.wait_for_event(&events_out);
        vc_disabled_event
    }
}

impl<P> VcManagementErrorApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_error(&self) -> ApiResult<VCManagementError> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_disabled_event: ApiResult<VCManagementError> = self.api.wait_for_event(&events_out);
        vc_disabled_event
    }
}

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

/// Error
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCManagementError;
impl StaticEvent for VCManagementError {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "()";
}
