use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use std::sync::mpsc::channel;
use substrate_api_client::{ApiClientError, ApiResult, Error, Events, FromHexString, StaticEvent};

use crate::ApiClient;

pub trait SubscribeEventPatch {
    // For subscribe specific event
    fn wait_event<EventType: StaticEvent>(&self) -> ApiResult<EventType>;

    // For subscribe batch call events
    fn wait_events<EventType: StaticEvent>(&self, target_num: usize) -> ApiResult<Vec<EventType>>;

    // For wait error
    fn wait_error<EventType: StaticEvent>(&self) -> ApiResult<EventType>;
}

impl<P> SubscribeEventPatch for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn wait_event<EventType: StaticEvent>(&self) -> ApiResult<EventType> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in)?;

        let event: ApiResult<EventType> = self.api.wait_for_event(&events_out);
        event
    }

    fn wait_events<EventType: StaticEvent>(&self, target_num: usize) -> ApiResult<Vec<EventType>> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in)?;

        let mut collected_events = vec![];
        loop {
            if collected_events.len() == target_num {
                break;
            }

            let events_str = events_out.recv()?;
            let event_bytes = Vec::from_hex(events_str)?;
            let events = Events::new(self.api.metadata.clone(), Default::default(), event_bytes);

            for maybe_event_details in events.iter() {
                let event_details = maybe_event_details?;
                let event_metadata = event_details.event_metadata();
                println!(
                    "Found extrinsic: {:?}, {:?}",
                    event_metadata.pallet(),
                    event_metadata.event()
                );

                let pallet_name = event_metadata.pallet();
                let pallet_event = event_metadata.event();

                if pallet_name == "System" && pallet_event == "ExtrinsicFailed" {
                    return Err(ApiClientError::Other(
                        format!("System ExtrinsicFailed: {:?}", event_metadata).into(),
                    ));
                }

                if pallet_name == EventType::PALLET && pallet_event == EventType::EVENT {
                    println!("meta: {:?}", event_metadata);
                    let event = event_details
                        .as_event::<EventType>()?
                        .ok_or(Error::Other("Could not find the specific event".into()))?;
                    collected_events.push(event);
                }
            }
        }

        Ok(collected_events)
    }

    fn wait_error<EventType: StaticEvent>(&self) -> ApiResult<EventType> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in)?;

        let vc_disabled_event: ApiResult<EventType> = self.api.wait_for_event(&events_out);
        vc_disabled_event
    }
}
