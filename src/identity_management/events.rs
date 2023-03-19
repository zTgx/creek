use super::IDENTITY_PALLET_NAME;
use crate::{
    primitives::{AccountId, AesOutput},
    ApiClient,
};
use codec::Decode;
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use std::sync::mpsc::channel;
use substrate_api_client::{ApiResult, StaticEvent};

pub trait IdentityManagementEventApi {
    fn wait_event_user_shielding_key_set(&self) -> ApiResult<SetUserShieldingKeyEvent>;
    fn wait_event_set_user_shielding_key_handle_failed(
        &self,
    ) -> ApiResult<SetUserShieldingKeyHandlingFailedEvent>;
    fn wait_event_identity_created(&self) -> ApiResult<IdentityCreatedEvent>;
}

impl<P> IdentityManagementEventApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_event_user_shielding_key_set(&self) -> ApiResult<SetUserShieldingKeyEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let event: ApiResult<SetUserShieldingKeyEvent> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_set_user_shielding_key_handle_failed(
        &self,
    ) -> ApiResult<SetUserShieldingKeyHandlingFailedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let event: ApiResult<SetUserShieldingKeyHandlingFailedEvent> =
            self.api.wait_for_event(&events_out);
        event
    }

    fn wait_event_identity_created(&self) -> ApiResult<IdentityCreatedEvent> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();
        let event: ApiResult<IdentityCreatedEvent> = self.api.wait_for_event(&events_out);
        event
    }
}

/// UserShieldingKeySet
#[derive(Decode, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyEvent {
    pub account: AccountId,
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
}

impl StaticEvent for IdentityCreatedEvent {
    const PALLET: &'static str = IDENTITY_PALLET_NAME;
    const EVENT: &'static str = "IdentityCreated";
}
