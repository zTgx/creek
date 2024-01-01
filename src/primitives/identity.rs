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

use crate::utils::hex::{decode_hex, hex_encode};
use codec::{Decode, Encode, MaxEncodedLen};
use pallet_evm::{AddressMapping, HashedAddressMapping as GenericHashedAddressMapping};
use scale_info::TypeInfo;
use sp_core::{blake2_256, ed25519, sr25519, H160};
use sp_runtime::{traits::BlakeTwo256, AccountId32};
use std::fmt::Debug;
use strum_macros::EnumIter;

pub type HashedAddressMapping = GenericHashedAddressMapping<BlakeTwo256>;

use super::{
	address::{Address20, Address32, Address33},
	keypair::AccountId,
	network::Web3Network,
	MetadataOf, ParentchainBlockNumber,
};

/// Web2 and Web3 Identity based on handle/public key
/// We only include the network categories (substrate/evm) without concrete types
/// see https://github.com/litentry/litentry-parachain/issues/1841
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, EnumIter)]
pub enum Identity {
	// web2
	#[codec(index = 0)]
	Twitter(String),
	#[codec(index = 1)]
	Discord(String),
	#[codec(index = 2)]
	Github(String),

	// web3
	#[codec(index = 3)]
	Substrate(Address32),
	#[codec(index = 4)]
	Evm(Address20),
	// bitcoin addresses are derived (one-way hash) from the pubkey
	// by using `Address33` as the Identity handle, it requires that pubkey
	// is retrievable by the wallet API when verifying the bitcoin account.
	// e.g. unisat-wallet: https://docs.unisat.io/dev/unisat-developer-service/unisat-wallet#getpublickey
	#[codec(index = 5)]
	Bitcoin(Address33),
}

impl Identity {
	pub fn is_web2(&self) -> bool {
		matches!(self, Self::Twitter(..) | Self::Discord(..) | Self::Github(..))
	}

	pub fn is_web3(&self) -> bool {
		matches!(self, Self::Substrate(..) | Self::Evm(..) | Self::Bitcoin(..))
	}

	pub fn is_substrate(&self) -> bool {
		matches!(self, Self::Substrate(..))
	}

	pub fn is_evm(&self) -> bool {
		matches!(self, Self::Evm(..))
	}

	pub fn is_bitcoin(&self) -> bool {
		matches!(self, Self::Bitcoin(..))
	}

	// check if the given web3networks match the identity
	pub fn matches_web3networks(&self, networks: &Vec<Web3Network>) -> bool {
		(self.is_substrate() && !networks.is_empty() && networks.iter().all(|n| n.is_substrate())) ||
			(self.is_evm() && !networks.is_empty() && networks.iter().all(|n| n.is_evm())) ||
			(self.is_bitcoin() &&
				!networks.is_empty() &&
				networks.iter().all(|n| n.is_bitcoin())) ||
			(self.is_web2() && networks.is_empty())
	}

	/// Currently we only support mapping from Address32/Address20 to AccountId, not opposite.
	pub fn to_account_id(&self) -> Option<AccountId> {
		match self {
			Identity::Substrate(address) => Some(address.into()),
			Identity::Evm(address) =>
				Some(HashedAddressMapping::into_account_id(H160::from_slice(address.as_ref()))),
			Identity::Bitcoin(address) => Some(blake2_256(address.as_ref()).into()),
			_ => None,
		}
	}

	// #[cfg(any(feature = "std", feature = "sgx"))]
	pub fn from_did(s: &str) -> Result<Self, std::boxed::Box<dyn std::error::Error + 'static>> {
		let did_prefix = std::string::String::from("did:litentry:");
		if s.starts_with(&did_prefix) {
			let did_suffix = &s[did_prefix.len()..];
			let v: Vec<&str> = did_suffix.split(':').collect();
			if v.len() == 2 {
				if v[0] == "substrate" {
					let handle = decode_hex(v[1])
						.unwrap()
						.as_slice()
						.try_into()
						.map_err(|_| "Address32 conversion error")?;
					return Ok(Identity::Substrate(handle))
				} else if v[0] == "evm" {
					let handle = decode_hex(v[1])
						.unwrap()
						.as_slice()
						.try_into()
						.map_err(|_| "Address20 conversion error")?;
					return Ok(Identity::Evm(handle))
				} else if v[0] == "bitcoin" {
					let handle = decode_hex(v[1])
						.unwrap()
						.as_slice()
						.try_into()
						.map_err(|_| "Address33 conversion error")?;
					return Ok(Identity::Bitcoin(handle))
				} else if v[0] == "github" {
					return Ok(Identity::Github(v[1].to_string()))
				} else if v[0] == "discord" {
					return Ok(Identity::Discord(v[1].to_string()))
				} else if v[0] == "twitter" {
					return Ok(Identity::Twitter(v[1].to_string()))
				} else {
					return Err("Unknown did type".into())
				}
			} else {
				return Err("Wrong did suffix".into())
			}
		}

		Err("Wrong did prefix".into())
	}

	// #[cfg(any(feature = "std", feature = "sgx"))]
	pub fn to_did(
		&self,
	) -> Result<std::string::String, std::boxed::Box<dyn std::error::Error + 'static>> {
		Ok(std::format!(
			"did:litentry:{}",
			match self {
				Identity::Evm(address) => std::format!("evm:{}", &hex_encode(address.as_ref())),
				Identity::Substrate(address) =>
					std::format!("substrate:{}", &hex_encode(address.as_ref())),
				Identity::Bitcoin(address) =>
					std::format!("bitcoin:{}", &hex_encode(address.as_ref())),
				Identity::Twitter(handle) => std::format!("twitter:{}", handle),
				Identity::Discord(handle) => std::format!("discord:{}", handle),
				Identity::Github(handle) => std::format!("github:{}", handle),
			}
		))
	}
}

impl From<ed25519::Public> for Identity {
	fn from(value: ed25519::Public) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<sr25519::Public> for Identity {
	fn from(value: sr25519::Public) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<AccountId32> for Identity {
	fn from(value: AccountId32) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<Address32> for Identity {
	fn from(value: Address32) -> Self {
		Identity::Substrate(value)
	}
}

impl From<Address20> for Identity {
	fn from(value: Address20) -> Self {
		Identity::Evm(value)
	}
}

impl From<Address33> for Identity {
	fn from(value: Address33) -> Self {
		Identity::Bitcoin(value)
	}
}

impl From<[u8; 32]> for Identity {
	fn from(value: [u8; 32]) -> Self {
		Identity::Substrate(value.into())
	}
}

impl From<[u8; 20]> for Identity {
	fn from(value: [u8; 20]) -> Self {
		Identity::Evm(value.into())
	}
}

impl From<[u8; 33]> for Identity {
	fn from(value: [u8; 33]) -> Self {
		Identity::Bitcoin(value.into())
	}
}

// The context associated with the (litentry-account, did) pair
// TODO: maybe we have better naming
#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen, Default)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct IdentityContext {
	// the metadata
	pub metadata: Option<MetadataOf>,
	// the block number (of parent chain) where the creation was intially requested
	pub creation_request_block: Option<ParentchainBlockNumber>,
	// the block number (of parent chain) where the verification was intially requested
	pub verification_request_block: Option<ParentchainBlockNumber>,
	// if this did is verified
	pub is_verified: bool,
}

impl IdentityContext {
	pub fn new(
		creation_request_block: ParentchainBlockNumber,
		verification_request_block: ParentchainBlockNumber,
	) -> Self {
		Self {
			metadata: None,
			creation_request_block: Some(creation_request_block),
			verification_request_block: Some(verification_request_block),
			is_verified: false,
		}
	}
}
