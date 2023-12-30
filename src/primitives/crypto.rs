// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use codec::{Decode, Encode};
use rsa::{
	errors::{Error as RsaError, Result as RsaResult},
	BigUint, RsaPublicKey,
};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

use super::{aes::RequestAesKeyNonce, BlockHash, USER_SHIELDING_KEY_NONCE_LEN};

#[derive(
	Serialize, Deserialize, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo,
)]
pub struct Rsa3072Pubkey {
	pub n: Vec<u8>,
	pub e: Vec<u8>,
}

pub trait RsaPublicKeyGenerator {
	type Input;

	fn new_with_rsa3072_pubkey(shielding_key: Self::Input) -> RsaResult<RsaPublicKey>;
}

impl RsaPublicKeyGenerator for RsaPublicKey {
	type Input = Vec<u8>;

	fn new_with_rsa3072_pubkey(shielding_key: Self::Input) -> RsaResult<RsaPublicKey> {
		let key: Rsa3072Pubkey =
			serde_json::from_slice(&shielding_key).map_err(|_| RsaError::InvalidPaddingScheme)?;
		let b = BigUint::from_radix_le(&key.n, 256).ok_or(RsaError::InvalidCoefficient)?;
		let a = BigUint::from_radix_le(&key.e, 256).ok_or(RsaError::InvalidCoefficient)?;

		RsaPublicKey::new(b, a)
	}
}

// all-in-one struct containing the encrypted ciphertext with user's
// shielding key and other metadata that is required for decryption
//
// by default a postfix tag is used => last 16 bytes of ciphertext is MAC tag
#[derive(Debug, Default, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct AesOutput {
	pub ciphertext: Vec<u8>,
	pub aad: Vec<u8>,
	pub nonce: RequestAesKeyNonce, // IV
}

impl AesOutput {
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn len(&self) -> usize {
		self.ciphertext.len() + self.aad.len() + USER_SHIELDING_KEY_NONCE_LEN
	}
}

#[derive(Encode, Decode, Debug)]
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

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum DirectRequestStatus {
	/// Direct request was successfully executed
	Ok,
	/// Trusted Call Status
	TrustedOperationStatus(TrustedOperationStatus),
	/// Direct request could not be executed
	Error,
}
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub enum TrustedOperationStatus {
	/// TrustedOperation is submitted to the top pool.
	Submitted,
	/// TrustedOperation is part of the future queue.
	Future,
	/// TrustedOperation is part of the ready queue.
	Ready,
	/// The operation has been broadcast to the given peers.
	Broadcast,
	/// TrustedOperation has been included in block with given hash.
	InSidechainBlock(BlockHash),
	/// The block this operation was included in has been retracted.
	Retracted,
	/// Maximum number of finality watchers has been reached,
	/// old watchers are being removed.
	FinalityTimeout,
	/// TrustedOperation has been finalized by a finality-gadget, e.g GRANDPA
	Finalized,
	/// TrustedOperation has been replaced in the pool, by another operation
	/// that provides the same tags. (e.g. same (sender, nonce)).
	Usurped,
	/// TrustedOperation has been dropped from the pool because of the limit.
	Dropped,
	/// TrustedOperation is no longer valid in the current state.
	Invalid,
}
