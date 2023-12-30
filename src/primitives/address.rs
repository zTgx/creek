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

use crate::if_production_or;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{ecdsa, ed25519, sr25519, ByteArray};
use sp_runtime::AccountId32;
use std::fmt::{Debug, Formatter};

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Address20([u8; 20]);

impl AsRef<[u8; 20]> for Address20 {
	fn as_ref(&self) -> &[u8; 20] {
		&self.0
	}
}

impl From<[u8; 20]> for Address20 {
	fn from(value: [u8; 20]) -> Self {
		Self(value)
	}
}

impl<'a> TryFrom<&'a [u8]> for Address20 {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<Address20, ()> {
		if x.len() == 20 {
			let mut data = [0; 20];
			data.copy_from_slice(x);
			Ok(Address20(data))
		} else {
			Err(())
		}
	}
}

impl Debug for Address20 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address20").finish(),
			f.debug_tuple("Address20").field(&self.0).finish()
		)
	}
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Address32([u8; 32]);
impl AsRef<[u8; 32]> for Address32 {
	fn as_ref(&self) -> &[u8; 32] {
		&self.0
	}
}

impl From<[u8; 32]> for Address32 {
	fn from(value: [u8; 32]) -> Self {
		Self(value)
	}
}

impl From<AccountId32> for Address32 {
	fn from(value: AccountId32) -> Self {
		let raw: [u8; 32] = value.as_slice().try_into().unwrap();
		Address32::from(raw)
	}
}

impl<'a> TryFrom<&'a [u8]> for Address32 {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<Address32, ()> {
		if x.len() == 32 {
			let mut data = [0; 32];
			data.copy_from_slice(x);
			Ok(Address32(data))
		} else {
			Err(())
		}
	}
}

impl From<Address32> for AccountId32 {
	fn from(value: Address32) -> Self {
		let raw: [u8; 32] = *value.as_ref();
		AccountId32::from(raw)
	}
}

impl From<&Address32> for AccountId32 {
	fn from(value: &Address32) -> Self {
		(*value).into()
	}
}

impl From<sr25519::Public> for Address32 {
	fn from(k: sr25519::Public) -> Self {
		k.0.into()
	}
}

impl From<ed25519::Public> for Address32 {
	fn from(k: ed25519::Public) -> Self {
		k.0.into()
	}
}

impl Debug for Address32 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address32").finish(),
			f.debug_tuple("Address32").field(&self.0).finish()
		)
	}
}

// TODO: maybe use macros to reduce verbosity
#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub struct Address33([u8; 33]);
impl AsRef<[u8; 33]> for Address33 {
	fn as_ref(&self) -> &[u8; 33] {
		&self.0
	}
}

impl Default for Address33 {
	fn default() -> Self {
		Address33([0u8; 33])
	}
}

impl From<[u8; 33]> for Address33 {
	fn from(value: [u8; 33]) -> Self {
		Self(value)
	}
}

impl<'a> TryFrom<&'a [u8]> for Address33 {
	type Error = ();
	fn try_from(x: &'a [u8]) -> Result<Address33, ()> {
		if x.len() == 33 {
			let mut data = [0; 33];
			data.copy_from_slice(x);
			Ok(Address33(data))
		} else {
			Err(())
		}
	}
}

impl From<Address33> for ecdsa::Public {
	fn from(value: Address33) -> Self {
		let raw: [u8; 33] = *value.as_ref();
		ecdsa::Public::from_raw(raw)
	}
}

impl From<&Address33> for ecdsa::Public {
	fn from(value: &Address33) -> Self {
		(*value).into()
	}
}

impl From<ecdsa::Public> for Address33 {
	fn from(k: ecdsa::Public) -> Self {
		k.0.into()
	}
}

impl Debug for Address33 {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		if_production_or!(
			f.debug_tuple("Address33").finish(),
			f.debug_tuple("Address33").field(&self.0).finish()
		)
	}
}
