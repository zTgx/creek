use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sp_core::H256;
use crate::primitives::{BlockHash, cerror::CError, CResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse {
	pub id: String,
	pub jsonrpc: String,
	pub result: String,
}

pub fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
	json!({
		"method": method,
		"params": params,
		"jsonrpc": "2.0",
		"id": id.to_string(),
	})
}

pub fn json_resp(resp: String) -> CResult<JsonResponse> {
	serde_json::from_str(&resp).map_err(CError::DecodeJsonError)
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
