pub mod identity_management;
pub mod primitives;
pub mod utils;
pub mod vc_management;

use std::sync::mpsc::channel;

use crate::primitives::{Enclave, MrEnclave};
use codec::Encode;
use primitives::RsaPublicKeyGenerator;
use rsa::RsaPublicKey;
use sp_core::{crypto::AccountId32 as AccountId, hexdisplay::HexDisplay, Pair};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
    compose_extrinsic, extrinsic::common::Batch, rpc::WsRpcClient, Api, CallIndex, Error, Events,
    FromHexString, Metadata, PlainTip, PlainTipExtrinsicParams, PlainTipExtrinsicParamsBuilder,
    StaticEvent, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4, XtStatus,
};

const ACCOUNT_SEED_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const NODE_URL: &str = "ws://127.0.0.1:9944";
pub type ApiType<P> = Api<P, WsRpcClient, PlainTipExtrinsicParams>;

#[derive(Clone)]
pub struct ApiClient<P>
where
    P: Pair,
{
    pub api: ApiType<P>,
}

impl<P> ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    pub fn new_with_signer(signer: P) -> Self {
        let client = WsRpcClient::new(NODE_URL);
        let api = ApiType::new(client)
            .map(|api| api.set_signer(signer))
            .unwrap();

        ApiClient { api }
    }

    pub fn get_signer(&self) -> Option<AccountId> {
        self.api.signer_account()
    }

    pub fn update_api(&mut self, tx_params: PlainTipExtrinsicParamsBuilder) {
        let updated_api = self.api.clone().set_extrinsic_params_builder(tx_params);
        self.api = updated_api;
    }

    pub fn send_extrinsic(&self, xthex_prefixed: String) {
        let tx_hash = self
            .api
            .send_extrinsic(xthex_prefixed, XtStatus::InBlock)
            .unwrap();
        println!(" ✅ Transaction got included. Hash: {:?}", tx_hash);
    }

    pub fn get_tee_shielding_pubkey(&self) -> RsaPublicKey {
        let enclave_count: u64 = self
            .api
            .get_storage_value("Teerex", "EnclaveCount", None)
            .unwrap()
            .unwrap();

        let enclave: Enclave<AccountId, Vec<u8>> = self
            .api
            .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
            .unwrap()
            .unwrap();

        let shielding_key = enclave.shielding_key.unwrap();
        RsaPublicKey::new_with_rsa3072_pubkey(shielding_key)
    }

    pub fn get_shard(&self) -> MrEnclave {
        let enclave_count: u64 = self
            .api
            .get_storage_value("Teerex", "EnclaveCount", None)
            .unwrap()
            .unwrap();

        let enclave: Enclave<AccountId, Vec<u8>> = self
            .api
            .get_storage_map("Teerex", "EnclaveRegistry", enclave_count, None)
            .unwrap()
            .unwrap();

        let shard = enclave.mr_enclave;
        let shard_in_hex = format!("0x{}", HexDisplay::from(&shard));

        println!("\n ✅ New shard : {}", shard_in_hex);

        shard
    }
}

/// FIXME: Maybe use this later...
pub trait ParachainMetadataApi {
    fn metadata(&self);
    fn get_total_issuance(&self);
}

impl<P> ParachainMetadataApi for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn metadata(&self) {
        let meta = Metadata::try_from(self.api.get_metadata().unwrap()).unwrap();
        meta.print_overview();
    }

    fn get_total_issuance(&self) {
        let result: u128 = self
            .api
            .get_storage_value("Balances", "TotalIssuance", None)
            .unwrap()
            .unwrap();
        println!("[+] TotalIssuance is {}", result);
    }
}

pub trait MockApiClient {
    fn mock_get_shard(&self) -> MrEnclave;
}

impl<P> MockApiClient for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn mock_get_shard(&self) -> MrEnclave {
        [
            65_u8, 56, 208, 116, 135, 54, 101, 208, 13, 173, 159, 82, 115, 60, 181, 148, 205, 211,
            71, 48, 174, 210, 172, 218, 70, 146, 182, 230, 5, 74, 110, 208,
        ]
    }
}

pub trait ApiClientPatch {
    fn batch_all<Call: Encode + Clone>(
        &self,
        calls: &[Call],
    ) -> UtilityBatchAllXt<Call, SubstrateDefaultSignedExtra<PlainTip>>;
}

const UTILITY_MODULE: &str = "Utility";
const UTILITY_BATCH_ALL: &str = "batch_all";

pub type UtilityBatchAllFn<Call> = (CallIndex, Batch<Call>);
pub type UtilityBatchAllXt<Call, SignedExtra> =
    UncheckedExtrinsicV4<UtilityBatchAllFn<Call>, SignedExtra>;

impl<P> ApiClientPatch for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn batch_all<Call: Encode + Clone>(
        &self,
        calls: &[Call],
    ) -> UtilityBatchAllXt<Call, SubstrateDefaultSignedExtra<PlainTip>> {
        let calls = Batch {
            calls: calls.to_vec(),
        };
        compose_extrinsic!(self.api.clone(), UTILITY_MODULE, UTILITY_BATCH_ALL, calls)
    }
}

pub trait SubscribeEventPatch<EventType: StaticEvent> {
    fn collect_events(&self, target_num: usize) -> Vec<EventType>;
}

impl<P, EventType> SubscribeEventPatch<EventType> for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
    EventType: StaticEvent,
{
    fn collect_events(&self, target_num: usize) -> Vec<EventType> {
        let (events_in, events_out) = channel();
        self.api.subscribe_events(events_in).unwrap();

        let mut collected = vec![];
        loop {
            if collected.len() == target_num {
                break;
            }

            let events_str = events_out.recv().unwrap();
            let event_bytes = Vec::from_hex(events_str).unwrap();
            let events = Events::new(self.api.metadata.clone(), Default::default(), event_bytes);

            for maybe_event_details in events.iter() {
                let event_details = maybe_event_details.unwrap();
                let event_metadata = event_details.event_metadata();
                println!(
                    "Found extrinsic: {:?}, {:?}",
                    event_metadata.pallet(),
                    event_metadata.event()
                );
                if event_metadata.pallet() == EventType::PALLET
                    && event_metadata.event() == EventType::EVENT
                {
                    let event = event_details
                        .as_event::<EventType>()
                        .unwrap()
                        .ok_or(Error::Other("Could not find the specific event".into()));
                    collected.push(event.unwrap());
                }
            }
        }

        collected
    }
}
