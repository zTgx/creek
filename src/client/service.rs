use crate::{
	core::{getter::Getter, trusted_call::TrustedCallSigned},
	primitives::{types::TrustedOperation, RsaRequest, ShardIdentifier},
	utils::{
		crypto::encrypt_with_tee_shielding_pubkey,
		hex::{json_resp, JsonResponse, ToHexPrefixed, json_req},
	},
	CResult,
};
use codec::{Decode, Encode};
use log::*;
use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use rsa::RsaPublicKey;
use serde_json::Value;
use sp_core::H256;
use std::{
	fmt::Debug,
	sync::mpsc::{channel, Sender as ThreadOut},
};
use ws::{
	connect, util::TcpStream, CloseCode, Handler, Handshake, Message, Result as WsResult, Sender,
};

pub type BlockHash = sp_core::H256;

#[derive(Debug, Clone, PartialEq, Encode, Decode, Eq)]
pub enum DirectRequestStatus {
	/// Direct request was successfully executed
	#[codec(index = 0)]
	Ok,
	/// Trusted Call Status
	/// Litentry: embed the top hash here - TODO - use generic type?
	#[codec(index = 1)]
	TrustedOperationStatus(TrustedOperationStatus, H256),
	/// Direct request could not be executed
	#[codec(index = 2)]
	Error,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode, Eq)]
pub enum TrustedOperationStatus {
	/// TrustedOperation is submitted to the top pool.
	#[codec(index = 0)]
	Submitted,
	/// TrustedOperation is part of the future queue.
	#[codec(index = 1)]
	Future,
	/// TrustedOperation is part of the ready queue.
	#[codec(index = 2)]
	Ready,
	/// The operation has been broadcast to the given peers.
	#[codec(index = 3)]
	Broadcast,
	/// TrustedOperation has been included in block with given hash.
	#[codec(index = 4)]
	InSidechainBlock(BlockHash),
	/// The block this operation was included in has been retracted.
	#[codec(index = 5)]
	Retracted,
	/// Maximum number of finality watchers has been reached,
	/// old watchers are being removed.
	#[codec(index = 6)]
	FinalityTimeout,
	/// TrustedOperation has been finalized by a finality-gadget, e.g GRANDPA
	#[codec(index = 7)]
	Finalized,
	/// TrustedOperation has been replaced in the pool, by another operation
	/// that provides the same tags. (e.g. same (sender, nonce)).
	#[codec(index = 8)]
	Usurped,
	/// TrustedOperation has been dropped from the pool because of the limit.
	#[codec(index = 9)]
	Dropped,
	/// TrustedOperation is no longer valid in the current state.
	#[codec(index = 10)]
	Invalid,
	/// TrustedOperation has been executed.
	TopExecuted(Vec<u8>, bool),
}

#[derive(Encode, Decode, Debug, Eq, PartialEq)]
pub struct RpcReturnValue {
	pub value: Vec<u8>,
	pub do_watch: bool,
	pub status: DirectRequestStatus,
}
impl RpcReturnValue {
	pub fn new(val: Vec<u8>, watch: bool, status: DirectRequestStatus) -> Self {
		Self { value: val, do_watch: watch, status }
	}

	pub fn from_error_message(error_msg: &str) -> Self {
		RpcReturnValue {
			value: error_msg.encode(),
			do_watch: false,
			status: DirectRequestStatus::Error,
		}
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
	type ThreadMessage = Message;

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
			.unwrap()
			.unwrap();

		result.send(Message::from(result_str)).unwrap();

		Ok(())
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
		let _ = self.out.shutdown().map_err(|e| {
			error!("shutdown error: {:?}", e);
		});
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		info!("msg received = {}", msg);
		self.message_handler.handle_message(msg, self.out.clone(), self.result.clone())
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
			.map_err(|e| {
				let details = format!("{:?}", e);
				ws::Error::new(ws::ErrorKind::Internal, details)
			})?
			.use_server_name_indication(false)
			.verify_hostname(false)
			.connect("", sock)
			.map_err(From::from)
	}
}

#[derive(Debug, Clone, Default)]
pub struct SidechainRpcClient {
	url: String,
}

impl SidechainRpcClient {
	pub fn new(url: &str) -> SidechainRpcClient {
		SidechainRpcClient { url: url.to_string() }
	}

	fn direct_rpc_request<MessageHandler>(
		&self,
		jsonreq: String,
		message_handler: MessageHandler,
	) -> CResult<MessageHandler::ThreadMessage>
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
		})
		.unwrap();
		Ok(result_out.recv().unwrap())
	}
}

pub trait SidechainRpcClientTrait {
	fn request(&self, jsonreq: serde_json::Value) -> CResult<JsonResponse>;
}
impl SidechainRpcClientTrait for SidechainRpcClient {
	fn request(&self, jsonreq: Value) -> CResult<JsonResponse> {
		let message = self
			.direct_rpc_request(jsonreq.to_string(), GetSidechainRequestHandler::default())
			.unwrap();

		let json_response: JsonResponse = json_resp(message.to_string());
		Ok(json_response)
	}
}

pub trait DiRequest {
	fn di_request(
		&self,
		shard: ShardIdentifier,
		tee_shielding_key: RsaPublicKey,
		operation_call: &TrustedOperation<TrustedCallSigned, Getter>,
	) -> CResult<JsonResponse>;
}

impl DiRequest for SidechainRpcClient {
	fn di_request(
		&self,
		shard: ShardIdentifier,
		shielding_pubkey: RsaPublicKey,
		operation_call: &TrustedOperation<TrustedCallSigned, Getter>,
	) -> CResult<JsonResponse> {
		// let jsonreq = get_json_request(shard, operation_call, shielding_pubkey);

		let param = get_json_request(shard, operation_call, shielding_pubkey);
		let jsonreq = json_req("author_submitAndWatchRsaRequest", [param], 1);
		println!("jsonreq: {}", jsonreq);

		let jsonresp = self.request(serde_json::to_value(jsonreq).unwrap()).unwrap();
		Ok(jsonresp)
	}
}

pub(crate) fn get_json_request(
	shard: ShardIdentifier,
	operation_call: &TrustedOperation<TrustedCallSigned, Getter>,
	shielding_pubkey: RsaPublicKey,
) -> String {
	let operation_call_encrypted =
		encrypt_with_tee_shielding_pubkey(&shielding_pubkey, &operation_call.encode());

	// compose jsonrpc call
	let request = RsaRequest::new(shard, operation_call_encrypted);
	request.to_hex()
	// RpcRequest::compose_jsonrpc_call(
	// 	Id::Text("1".to_string()),
	// 	"author_submitAndWatchRsaRequest".to_string(),
	// 	vec![request.to_hex()],
	// )
	// .unwrap()
}
