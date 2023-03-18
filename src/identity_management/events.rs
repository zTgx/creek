use super::IDENTITY_PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput},
    API,
};
use codec::Decode;
use std::sync::mpsc::channel;
use substrate_api_client::{ApiResult, StaticEvent};

/// UserShieldingKeySet
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyEvent {
    pub account: AccountId,
}

impl StaticEvent for SetUserShieldingKeyEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "UserShieldingKeySet";
}

pub fn wait_user_shielding_key_set_event() -> SetUserShieldingKeyEvent {
    let (events_in, events_out) = channel();
    API.subscribe_events(events_in).unwrap();
    let event: ApiResult<SetUserShieldingKeyEvent> = API.wait_for_event(&events_out);
    event.unwrap()
}

/// SetUserShieldingKeyHandlingFailed
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyHandlingFailedEvent;
impl StaticEvent for SetUserShieldingKeyHandlingFailedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
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
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityCreated";
}

pub fn wait_identity_created_event() -> IdentityCreatedEvent {
    let (events_in, events_out) = channel();
    API.subscribe_events(events_in).unwrap();
    let event: ApiResult<IdentityCreatedEvent> = API.wait_for_event(&events_out);
    event.unwrap()
}
