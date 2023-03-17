use super::PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput},
    API,
};
use codec::Decode;
use sp_core::H256;
use std::sync::mpsc::channel;
use substrate_api_client::{ApiResult, StaticEvent};

/// VCIssuedEvent
#[derive(Decode, Debug)]
pub struct VCIssuedEvent {
    pub account: AccountId,
    pub vc_index: H256,
    pub vc: AesOutput,
}

impl StaticEvent for VCIssuedEvent {
    const PALLET: &'static str = PALLET_NAME;
    const EVENT: &'static str = "VCIssued";
}

pub fn wait_vc_issued_event() -> VCIssuedEvent {
    let (events_in, events_out) = channel();
    API.subscribe_events(events_in).unwrap();
    let vc_issued_event: ApiResult<VCIssuedEvent> = API.wait_for_event(&events_out);
    vc_issued_event.unwrap()
}
