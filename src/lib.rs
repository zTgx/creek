#![feature(string_remove_matches)]

pub mod identity_management;
pub mod primitives;
pub mod sidechain;
pub mod utils;
pub mod vc_management;

use crate::primitives::{enclave::Enclave, MrEnclave};
use codec::Encode;
use log::*;
use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use primitives::crypto::RsaPublicKeyGenerator;
use rsa::RsaPublicKey;
use serde_json::Value;
use sp_core::ed25519;
use sp_core::{crypto::AccountId32 as AccountId, hexdisplay::HexDisplay, Pair};
use sp_runtime::{MultiSignature, MultiSigner};
use std::fmt::Debug;
use std::sync::mpsc::{channel, Sender as ThreadOut};
use substrate_api_client::{
    compose_extrinsic,
    extrinsic::common::Batch,
    rpc::{ws_client::RpcMessage, RpcClientError, WsRpcClient},
    Api, ApiResult, CallIndex, Error, Events, FromHexString, Metadata, PlainTip,
    PlainTipExtrinsicParams, PlainTipExtrinsicParamsBuilder, StaticEvent,
    SubstrateDefaultSignedExtra, UncheckedExtrinsicV4, XtStatus,
};
use utils::address::vec_to_u8_array;
use ws::{
    connect, util::TcpStream, CloseCode, Handler, Handshake, Message, Result as WsResult, Sender,
};

const NODE_URL: &str = "ws://127.0.0.1:9944";
const WORKER_URL: &str = "wss://localhost:2000";

const ACCOUNT_SEED_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
pub type ApiType<P> = Api<P, WsRpcClient, PlainTipExtrinsicParams>;

#[derive(Clone)]
pub struct ApiClient<P>
where
    P: Pair,
{
    pub api: ApiType<P>,
    pub sidechain: SidechainRpcClient,
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

        let sidechain = SidechainRpcClient::new(WORKER_URL);

        ApiClient { api, sidechain }
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

    pub fn get_vc_pubkey(&self) -> ed25519::Public {
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

        let vc_pubkey = enclave.vc_pubkey.expect("vc pubkey");
        ed25519::Public(vec_to_u8_array::<32>(vc_pubkey))
    }

    /// There're two methos to get the mrenclave
    /// 1. Online -> to use this method `get_shard` or
    /// 2. Offline -> to `litentry-parachain/tee-worker` run `make enclave`
    /// Both should be display exactly same value.
    ///
    /// TODO:
    /// But there's a question, what's the difference betwwen `mrenclave` and `shard`?
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

#[allow(clippy::result_large_err)]
pub trait SidechainHandleMessage {
    type ThreadMessage;

    fn handle_message(
        &self,
        msg: Message,
        out: Sender,
        result: ThreadOut<Self::ThreadMessage>,
    ) -> WsResult<()>;
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct GetSidechainRequestHandler;
impl SidechainHandleMessage for GetSidechainRequestHandler {
    type ThreadMessage = RpcMessage;

    fn handle_message(
        &self,
        msg: Message,
        out: Sender,
        result: ThreadOut<Self::ThreadMessage>,
    ) -> WsResult<()> {
        out.close(CloseCode::Normal)
            .unwrap_or_else(|_| warn!("Could not close Websocket normally"));

        info!("Got get_request_msg {}", msg);
        // let result_str = serde_json::from_str(msg.as_text()?)
        // 	.map(|v: serde_json::Value| Some(v["result"].to_string()))
        // 	.map_err(RpcClientError::Serde);
        let result_str = serde_json::from_str(msg.as_text()?)
            .map(|v: serde_json::Value| Some(v.to_string()))
            .map_err(RpcClientError::Serde);

        result
            .send(result_str)
            .map_err(|e| Box::new(RpcClientError::Send(format!("{:?}", e))).into())
    }
}

pub struct SidechainClient<MessageHandler, ThreadMessage> {
    pub out: ws::Sender,
    pub request: String,
    pub result: ThreadOut<ThreadMessage>,
    pub message_handler: MessageHandler,
}

impl<MessageHandler: SidechainHandleMessage> Handler
    for SidechainClient<MessageHandler, MessageHandler::ThreadMessage>
{
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        info!("sending request: {}", self.request);
        self.out.send(self.request.clone())?;
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        info!("Connection closing due to ({:?}) {}", code, reason);
        self.out.shutdown().unwrap();
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        info!("msg received = {}", msg);
        self.message_handler
            .handle_message(msg, self.out.clone(), self.result.clone())
    }

    fn upgrade_ssl_client(
        &mut self,
        sock: TcpStream,
        _: &url::Url,
    ) -> ws::Result<SslStream<TcpStream>> {
        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| {
            ws::Error::new(
                ws::ErrorKind::Internal,
                format!("Failed to upgrade client to SSL: {}", e),
            )
        })?;
        builder.set_verify(SslVerifyMode::empty());

        let connector = builder.build();
        connector
            .configure()
            .unwrap()
            .use_server_name_indication(false)
            .verify_hostname(false)
            .connect("", sock)
            .map_err(From::from)
    }
}

#[derive(Debug, Clone)]
pub struct SidechainRpcClient {
    url: String,
}

impl SidechainRpcClient {
    pub fn new(url: &str) -> SidechainRpcClient {
        SidechainRpcClient {
            url: url.to_string(),
        }
    }

    fn direct_rpc_request<MessageHandler>(
        &self,
        jsonreq: String,
        message_handler: MessageHandler,
    ) -> ApiResult<MessageHandler::ThreadMessage>
    where
        MessageHandler: SidechainHandleMessage + Clone + Send + 'static,
        MessageHandler::ThreadMessage: Send + Sync + Debug,
    {
        let (result_in, result_out) = channel();
        connect(self.url.as_str(), |out| SidechainClient {
            out,
            request: jsonreq.clone(),
            result: result_in.clone(),
            message_handler: message_handler.clone(),
        })?;
        Ok(result_out.recv()?)
    }
}

pub trait SidechainRpcClientTrait {
    fn request(&self, jsonreq: serde_json::Value) -> ApiResult<String>;
}
impl SidechainRpcClientTrait for SidechainRpcClient {
    fn request(&self, jsonreq: Value) -> ApiResult<String> {
        Ok(self
            .direct_rpc_request(jsonreq.to_string(), GetSidechainRequestHandler::default())??
            .unwrap_or_default())
    }
}
