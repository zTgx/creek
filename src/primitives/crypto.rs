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

use rsa::{
	errors::{Error as RsaError, Result as RsaResult},
	BigUint, RsaPublicKey,
};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

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
