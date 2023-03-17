use std::sync::mpsc::channel;
use codec::Decode;
use sp_core::H256;
use substrate_api_client::{StaticEvent, ApiResult};
use crate::{primitives::{AesOutput, AccountId}, API};
use super::PALLET_NAME;

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
