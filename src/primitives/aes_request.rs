// Copyright 2020-2023 Trust Computing GmbH.
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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

/// A morphling of itp_types::RsaRequest which stems from teerex_primitives::RsaRequest
///
/// Instead of encrypting the TrustedCallSigned with the TEE's shielding key, we encrypt
/// it with a 32-byte ephemeral AES key which is generated from the client's side, and
/// send the encrypted payload together with the AES key encrypted using TEE's shielding key.
///
/// After the enclave gets the request, it will decrypt to get the AES key and use that key
/// to decrypt the payload and decode it to get the real TrustedCall.
///
/// The motivation of having such a struct is:
/// 1. RSA has a limitation of maximum allowed test to be encrypted. In our case, the encoded
///    `TrustedCallSigned` can exceed the limit, AES doesn't have such problem.
///
/// 2. we want to efface the shielding key setup completely to achieve a better UE.
use codec::{Decode, Encode};
use std::fmt::Debug;

use super::{
	aes::{aes_decrypt, AesOutput, RequestAesKey},
	ShardIdentifier,
};

pub trait ShieldingCryptoDecrypt {
	type Error: Debug;
	fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error>;
}

// Represent a request that can be decrypted by the enclave
// Both itp_types::RsaRequest and AesRequest should impelement this
pub trait DecryptableRequest {
	type Error;
	// the shard getter
	fn shard(&self) -> ShardIdentifier;
	// the raw payload - AFAICT only used in mock
	fn payload(&self) -> &[u8];
	// how to decrypt the payload
	fn decrypt<T: Debug>(
		&mut self,
		enclave_shielding_key: Box<dyn ShieldingCryptoDecrypt<Error = T>>,
	) -> Result<Vec<u8>, Self::Error>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct AesRequest {
	pub shard: ShardIdentifier,
	pub key: Vec<u8>,
	pub payload: AesOutput,
}

impl DecryptableRequest for AesRequest {
	type Error = ();

	fn shard(&self) -> ShardIdentifier {
		self.shard
	}

	fn payload(&self) -> &[u8] {
		self.payload.ciphertext.as_slice()
	}

	fn decrypt<T: Debug>(
		&mut self,
		enclave_shielding_key: Box<dyn ShieldingCryptoDecrypt<Error = T>>,
	) -> core::result::Result<Vec<u8>, ()> {
		let aes_key: RequestAesKey = enclave_shielding_key
			.decrypt(&self.key)
			.map_err(|_| ())?
			.try_into()
			.map_err(|_| ())?;
		aes_decrypt(&aes_key, &mut self.payload).ok_or(())
	}
}
