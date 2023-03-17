use std::sync::mpsc::channel;
use codec::Decode;
use sp_core::H256;
use substrate_api_client::{StaticEvent, ApiResult};
use crate::{primitives::{AesOutput, AccountId}, API};
use super::PALLET_NAME;

/// UserShieldingKeySet
#[derive(Decode)]
struct SetUserShieldingKeyEvent {
    pub account: AccountId,
}

impl StaticEvent for SetUserShieldingKeyEvent {
    const PALLET: &'static str = PALLET_NAME;
    const EVENT: &'static str = "UserShieldingKeySet";
}

pub fn wait_user_shielding_key_set_event() -> SetUserShieldingKeyEvent {
    let (events_in, events_out) = channel();
	API.subscribe_events(events_in).unwrap();
	let event: ApiResult<SetUserShieldingKeyEvent> = API.wait_for_event(&events_out);
    event.unwrap()
}

/// SetUserShieldingKeyHandlingFailed
#[derive(Decode)]
struct SetUserShieldingKeyHandlingFailedEvent;
impl StaticEvent for SetUserShieldingKeyHandlingFailedEvent {
    const PALLET: &'static str = PALLET_NAME;
    const EVENT: &'static str = "SetUserShieldingKeyHandlingFailed";
}

pub fn wait_set_user_shielding_key_handle_failed_event() -> SetUserShieldingKeyHandlingFailedEvent {
    let (events_in, events_out) = channel();
	API.subscribe_events(events_in).unwrap();
	let event: ApiResult<SetUserShieldingKeyHandlingFailedEvent> = API.wait_for_event(&events_out);
    event.unwrap()
}

/// IdentityCreated
#[derive(Decode, Debug)]
pub struct IdentityCreatedEvent {
    pub who: AccountId,
    pub identity: AesOutput,
    pub code: AesOutput,
}

impl StaticEvent for IdentityCreatedEvent {
    const PALLET: &'static str = PALLET_NAME;
    const EVENT: &'static str = "IdentityCreated";
}

pub fn wait_identity_created_event() -> IdentityCreatedEvent {
    let (events_in, events_out) = channel();
	API.subscribe_events(events_in).unwrap();
	let event: ApiResult<IdentityCreatedEvent> = API.wait_for_event(&events_out);
    event.unwrap()
}