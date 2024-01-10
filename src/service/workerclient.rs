use crate::{
	primitives::cerror::CError,
	service::json::{DirectRequestStatus, JsonResponse, RpcReturnValue, TrustedOperationStatus},
	utils::hex::FromHexPrefixed,
	CResult,
};
use codec::Decode;
use log::*;
use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use serde_json::Value;
use sp_core::H256 as Hash;
use std::{
	sync::{
		mpsc::{channel, Sender as MpscSender},
		Arc, Mutex,
	},
	thread::{self, JoinHandle},
};
use ws::{connect, util::TcpStream, CloseCode, Handler, Handshake, Message, Sender};

/// Control a registered web-socket client.
#[derive(Default)]
pub struct WsClientControl {
	subscriber: Mutex<Option<Sender>>,
}

impl Clone for WsClientControl {
	fn clone(&self) -> Self {
		WsClientControl { subscriber: Mutex::new(self.subscriber.lock().unwrap().clone()) }
	}
}

impl WsClientControl {
	pub fn close_connection(&self) -> ws::Result<()> {
		if let Some(s) = self.subscriber.lock().unwrap().as_ref() {
			debug!("Closing connection");
			s.close(CloseCode::Normal)?;
			debug!("Connection is closed");
		}
		Ok(())
	}

	fn subscribe_sender(&self, sender: Sender) -> CResult<()> {
		let mut subscriber_lock = self.subscriber.lock().unwrap();
		*subscriber_lock = Some(sender);
		Ok(())
	}

	pub fn send(&self, request: &str) -> ws::Result<()> {
		if let Some(s) = self.subscriber.lock().unwrap().as_ref() {
			s.send(request)?;
			Ok(())
		} else {
			// Err(Error::Custom("Sender not initialized".into()))
			todo!()
		}
	}
}

#[derive(Clone)]
pub struct WsClient {
	web_socket: Sender,
	request: String,
	result: MpscSender<String>,
	do_watch: bool,
}

impl WsClient {
	/// Connect a web-socket client for multiple request/responses.
	///
	/// Control over the connection is done using the provided client control.
	/// (e.g. shutdown has to be initiated explicitly).
	#[allow(clippy::result_large_err)]
	pub fn connect_watch_with_control(
		url: &str,
		request: &str,
		result: &MpscSender<String>,
		control: Arc<WsClientControl>,
	) -> ws::Result<()> {
		debug!("Connecting web-socket connection with watch");
		connect(url.to_string(), |out| {
			control.subscribe_sender(out.clone()).expect("Failed sender subscription");
			WsClient::new(out, request.to_string(), result.clone(), true)
		})
	}

	/// Connects a web-socket client for a one-shot request.
	#[allow(clippy::result_large_err)]
	pub fn connect_one_shot(
		url: &str,
		request: &str,
		result: MpscSender<String>,
	) -> ws::Result<()> {
		debug!("Connecting one-shot web-socket connection");
		connect(url.to_string(), |out| {
			debug!("Create new web-socket client");
			WsClient::new(out, request.to_string(), result.clone(), false)
		})
	}

	fn new(
		web_socket: Sender,
		request: String,
		result: MpscSender<String>,
		do_watch: bool,
	) -> WsClient {
		WsClient { web_socket, request, result, do_watch }
	}
}

impl Handler for WsClient {
	fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
		debug!("sending request: {:?}", self.request.clone());
		match self.web_socket.send(self.request.clone()) {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
	}

	fn on_message(&mut self, msg: Message) -> ws::Result<()> {
		trace!("got message");
		trace!("{}", msg);
		trace!("sending result to MpscSender..");
		self.result.send(msg.to_string()).expect("Failed to send");
		if !self.do_watch {
			debug!("do_watch is false, closing connection");
			self.web_socket.close(CloseCode::Normal).expect("Failed to close connection");
			debug!("Connection close requested");
		}
		debug!("on_message successful, returning");
		Ok(())
	}

	fn on_close(&mut self, _code: CloseCode, _reason: &str) {
		debug!("Web-socket close");
		self.web_socket.shutdown().expect("Failed to shutdown")
	}

	/// we are overriding the `upgrade_ssl_client` method in order to disable hostname verification
	/// this is taken from https://github.com/housleyjk/ws-rs/blob/master/examples/unsafe-ssl-client.rs
	/// TODO: hostname verification should probably be enabled again for production?
	fn upgrade_ssl_client(
		&mut self,
		sock: TcpStream,
		_: &url::Url,
	) -> ws::Result<SslStream<TcpStream>> {
		let mut builder = SslConnector::builder(SslMethod::tls_client()).map_err(|e| {
			ws::Error::new(
				ws::ErrorKind::Internal,
				format!("Failed to upgrade client to SSL: {}", e),
			)
		})?;
		builder.set_verify(SslVerifyMode::empty());

		let connector = builder.build();
		connector
			.configure()
			.expect("Invalid connection config")
			.use_server_name_indication(false)
			.verify_hostname(false)
			.connect("", sock)
			.map_err(From::from)
	}
}

// pub struct SidechainClient<MessageHandler, ThreadMessage> {
// 	pub out: Sender,
// 	pub request: String,
// 	pub result: ThreadOut<ThreadMessage>,
// 	pub message_handler: MessageHandler,
// }

// impl<MessageHandler: SidechainHandleMessage> Handler
// 	for SidechainClient<MessageHandler, MessageHandler::ThreadMessage>
// {
// 	fn on_open(&mut self, _: Handshake) -> WsResult<()> {
// 		info!("sending request: {}", self.request);
// 		self.out.send(self.request.clone())?;
// 		Ok(())
// 	}

// 	fn on_close(&mut self, code: CloseCode, reason: &str) {
// 		info!("Connection closing due to ({:?}) {}", code, reason);
// 		let _ = self.out.shutdown().map_err(|e| {
// 			error!("shutdown error: {:?}", e);
// 		});
// 	}

// 	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
// 		info!("msg received = {}", msg);
// 		self.message_handler.handle_message(msg, self.out.clone(), self.result.clone())
// 	}

// 	fn upgrade_ssl_client(
// 		&mut self,
// 		sock: TcpStream,
// 		_: &url::Url,
// 	) -> ws::Result<SslStream<TcpStream>> {
// 		let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| {
// 			ws::Error::new(
// 				ws::ErrorKind::Internal,
// 				format!("Failed to upgrade client to SSL: {}", e),
// 			)
// 		})?;
// 		builder.set_verify(SslVerifyMode::empty());

// 		let connector = builder.build();
// 		connector
// 			.configure()
// 			.map_err(|e| {
// 				let details = format!("{:?}", e);
// 				ws::Error::new(ws::ErrorKind::Internal, details)
// 			})?
// 			.use_server_name_indication(false)
// 			.verify_hostname(false)
// 			.connect("", sock)
// 			.map_err(From::from)
// 	}
// }

// #[derive(Debug, Clone, Default)]
// pub struct SidechainRpcClient {
// 	url: String,
// }

// impl SidechainRpcClient {
// 	pub fn new(url: &str) -> SidechainRpcClient {
// 		SidechainRpcClient { url: url.to_string() }
// 	}

// 	fn send<MessageHandler>(
// 		&self,
// 		result_in: Sender,
// 		jsonreq: String,
// 		message_handler: MessageHandler,
// 	) -> CResult<()>
// 	where
// 		MessageHandler: SidechainHandleMessage + Clone + Send + 'static,
// 		MessageHandler::ThreadMessage: Send + Sync + Debug,
// 	{
// 		connect(self.url.as_str(), |out| SidechainClient {
// 			out,
// 			request: jsonreq.clone(),
// 			result: result_in.clone(),
// 			message_handler: message_handler.clone(),
// 		})
// 		.map_err(|_| CError::APIError)

// 		// let message = result_out.recv().map_err(CError::RecvError)?;

// 		// Ok(message)
// 	}
// }

pub trait DirectApi {
	fn get(&self, request: &str) -> CResult<String>;
	fn watch(&self, request: String, sender: MpscSender<String>) -> JoinHandle<()>;
	fn send(&self, request: &str) -> CResult<()>;
	fn close(&self) -> CResult<()>;
}

#[derive(Clone)]
pub struct DirectClient {
	url: String,
	web_socket_control: Arc<WsClientControl>,
}

impl DirectClient {
	pub fn new(url: String) -> Self {
		Self { url, web_socket_control: Default::default() }
	}
}

impl DirectApi for DirectClient {
	fn get(&self, request: &str) -> CResult<String> {
		let (port_in, port_out) = channel();

		info!("[WorkerApi Direct]: (get) Sending request: {:?}", request);
		WsClient::connect_one_shot(&self.url, request, port_in)
			.map_err(|e| CError::Other(format!("{:?}", e)))?;

		debug!("Waiting for web-socket result..");
		port_out.recv().map_err(|e| CError::Other(format!("{:?}", e)))
	}

	fn watch(&self, request: String, sender: MpscSender<String>) -> JoinHandle<()> {
		info!("[WorkerApi Direct]: (watch) Sending request: {:?}", request);
		let url = self.url.clone();

		let web_socket_control = self.web_socket_control.clone();
		// Unwrap is fine here, because JoinHandle can be used to handle a Thread panic.
		thread::spawn(move || {
			WsClient::connect_watch_with_control(&url, &request, &sender, web_socket_control)
				.expect("Connection failed")
		})
	}

	fn send(&self, request: &str) -> CResult<()> {
		self.web_socket_control
			.send(request)
			.map_err(|e| CError::Other(format!("{:?}", e)))
	}

	fn close(&self) -> CResult<()> {
		self.web_socket_control
			.close_connection()
			.map_err(|e| CError::Other(format!("{:?}", e)))
	}
}

pub trait SidechainRpcRequest {
	fn request(&self, jsonreq: serde_json::Value) -> CResult<JsonResponse>;
}

impl SidechainRpcRequest for DirectClient {
	fn request(&self, jsonreq: Value) -> CResult<JsonResponse> {
		let (sender, receiver) = channel::<String>();
		self.watch(jsonreq.to_string(), sender);

		loop {
			match receiver.recv() {
				Ok(response) => {
					let response: JsonResponse = serde_json::from_str(&response).unwrap();
					if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
						match return_value.status {
							DirectRequestStatus::Error => {
								if let Ok(value) =
									String::decode(&mut return_value.value.as_slice())
								{
									println!("[Error] {}", value);
								}

								return Err(CError::Other(
									"[Error] DirectRequestStatus::Error".to_string(),
								))
							},
							DirectRequestStatus::TrustedOperationStatus(status, top_hash) => {
								println!(
									"request status is: {:?}, top_hash: {:?}",
									status, top_hash
								);

								if matches!(status, TrustedOperationStatus::Invalid) {
									return Err(CError::Other(
										"[Error] Error occurred while executing trusted call"
											.to_string(),
									))
								}

								if let Ok(value) = Hash::decode(&mut return_value.value.as_slice())
								{
									println!("Trusted call {:?} is {:?}", value, status);
								}

								if !return_value.do_watch {
									self.close().unwrap();
									return Ok(response)
								}
							},
							DirectRequestStatus::Ok => {
								self.close().unwrap();
								return Ok(response)
							},
						}
					};
				},
				_ => todo!(),
			};
		}
	}
}
