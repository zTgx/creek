use log::*;
use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use serde_json::Value;
use std::{
	fmt::Debug,
	sync::mpsc::{channel, Sender as ThreadOut},
};
use substrate_api_client::{
	rpc::{ws_client::RpcMessage, RpcClientError},
	ApiResult,
};
use ws::{
	connect, util::TcpStream, CloseCode, Handler, Handshake, Message, Result as WsResult, Sender,
};

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

#[derive(Debug, Clone)]
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
