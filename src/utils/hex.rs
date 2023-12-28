/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

//! Hex encoding utility functions.

use codec::{Decode, Encode};
use std::{boxed::Box, string::String, vec::Vec};
pub type Result<T> = core::result::Result<T, Error>;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// extrinsics factory error
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Insufficient buffer size. Actual: {0}, required: {1}")]
	InsufficientBufferSize(usize, usize),
	#[error("Could not decode from hex data: {0}")]
	Hex(hex::FromHexError),
	#[error("Parity Scale Codec: {0}")]
	Codec(codec::Error),
	#[error(transparent)]
	Other(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
}

/// Trait to encode a given value to a hex string, prefixed with "0x".
pub trait ToHexPrefixed {
	fn to_hex(&self) -> String;
}

impl<T: Encode> ToHexPrefixed for T {
	fn to_hex(&self) -> String {
		hex_encode(&self.encode())
	}
}

/// Trait to decode a hex string to a given output.
pub trait FromHexPrefixed {
	type Output;

	fn from_hex(msg: &str) -> Result<Self::Output>;
}

impl<T: Decode> FromHexPrefixed for T {
	type Output = T;

	fn from_hex(msg: &str) -> Result<Self::Output> {
		let byte_array = decode_hex(msg)?;
		Decode::decode(&mut byte_array.as_slice()).map_err(Error::Codec)
	}
}

/// Hex encodes given data and preappends a "0x".
pub fn hex_encode(data: &[u8]) -> String {
	let mut hex_str = hex::encode(data);
	hex_str.insert_str(0, "0x");
	hex_str
}

/// Helper method for decoding hex.
pub fn decode_hex<T: AsRef<[u8]>>(message: T) -> Result<Vec<u8>> {
	let mut message = message.as_ref();
	if message[..2] == [b'0', b'x'] {
		message = &message[2..]
	}
	let decoded_message = hex::decode(message).map_err(Error::Hex)?;
	Ok(decoded_message)
}

/// storage key in hex
// pub fn storage_key_challenge_code(account: &Address32, identity: &Identity) -> String {
// 	let mut entry_bytes = sp_core::twox_128("IdentityManagement".as_bytes()).to_vec();
// 	entry_bytes.extend(&sp_core::twox_128("ChallengeCodes".as_bytes())[..]);

// 	let encoded_account: &[u8] = &account.encode();
// 	let encoded_identity: &[u8] = &identity.encode();

// 	// Key1: Blake2_128Concat
// 	entry_bytes.extend(sp_core::blake2_128(encoded_account));
// 	entry_bytes.extend(encoded_account);

// 	// Key2: Blake2_128Concat
// 	entry_bytes.extend(sp_core::blake2_128(encoded_identity));
// 	entry_bytes.extend(encoded_identity);

// 	format!("0x{}", hex::encode(entry_bytes))
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse {
	pub id: String,
	pub jsonrpc: String,
	pub result: String,
}

#[derive(Clone, Encode, Decode, Debug, Serialize, Deserialize)]
pub struct RpcResponse {
	pub jsonrpc: String,
	pub result: String, // hex encoded RpcReturnValue
	pub id: u32,
}

pub fn json_req<S: Serialize>(method: &str, params: S, id: u32) -> Value {
	json!({
		"method": method,
		"params": params,
		"jsonrpc": "2.0",
		"id": id.to_string(),
	})
}

pub fn json_resp(resp: String) -> JsonResponse {
	let resp: JsonResponse = serde_json::from_str(&resp).unwrap();
	resp
}

pub fn remove_whitespace(s: &str) -> String {
	s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hex_encode_decode_works() {
		let data = "Hello World!".to_string();

		let hex_encoded_data = hex_encode(&data.encode());
		let decoded_data =
			String::decode(&mut decode_hex(hex_encoded_data).unwrap().as_slice()).unwrap();

		assert_eq!(data, decoded_data);
	}

	#[test]
	fn to_hex_from_hex_works() {
		let data = "Hello World!".to_string();

		let hex_encoded_data = data.to_hex();
		let decoded_data = String::from_hex(&hex_encoded_data).unwrap();

		assert_eq!(data, decoded_data);
	}
}
