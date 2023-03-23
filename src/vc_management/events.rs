use super::VC_PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput},
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
    fn wait_error(&self) -> ApiResult<VCDisabledEvent>;
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

    fn wait_error(&self) -> ApiResult<VCDisabledEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let vc_disabled_event: ApiResult<VCDisabledEvent> = self.api.wait_for_event(&events_out);
        vc_disabled_event
    }
}

/// VCIssuedEvent
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCIssuedEvent {
    pub account: AccountId,
    pub vc_index: H256,
    pub vc: AesOutput,
}

impl StaticEvent for VCIssuedEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCIssued";
}

/// VCDisabled
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCDisabledEvent {
    pub vc_index: H256,
}

impl StaticEvent for VCDisabledEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCDisabled";
}

/// VCRevoked
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct VCRevokedEvent {
    pub vc_index: H256,
}

impl StaticEvent for VCRevokedEvent {
    const PALLET: &'static str = VC_PALLET_NAME;
    const EVENT: &'static str = "VCRevoked";
}
