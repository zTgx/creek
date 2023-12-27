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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{ecdsa, ed25519, sr25519};
use sp_runtime::BoundedVec;

use super::{
	address::{Address20, Address32},
	ethereum::EthereumSignature,
	IdentityString, MaxStringLength, MetadataOf, ParentchainBlockNumber,
};

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, Hash, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SubstrateNetwork {
	Polkadot,
	Kusama,
	Litentry,
	Litmus,
	LitentryRococo,
	Khala,
	TestNet, // when we launch it with standalone (integritee-)node
}

impl SubstrateNetwork {
	/// get the ss58 prefix, see https://github.com/paritytech/ss58-registry/blob/main/ss58-registry.json
	pub fn ss58_prefix(&self) -> u16 {
		match self {
			Self::Polkadot => 0,
			Self::Kusama => 2,
			Self::Litentry => 31,
			Self::Litmus => 131,
			Self::LitentryRococo => 42,
			Self::Khala => 30,
			Self::TestNet => 13,
		}
	}

	pub fn from_ss58_prefix(prefix: u16) -> Self {
		match prefix {
			0 => Self::Polkadot,
			2 => Self::Kusama,
			31 => Self::Litentry,
			131 => Self::Litmus,
			42 => Self::LitentryRococo,
			30 => Self::Khala,
			_ => Self::TestNet,
		}
	}
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, Hash, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum EvmNetwork {
	Ethereum,
	BSC,
}

#[derive(Encode, Decode, Copy, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Web2Network {
	Twitter,
	Discord,
	Github,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Identity {
	Substrate { network: SubstrateNetwork, address: Address32 },
	Evm { network: EvmNetwork, address: Address20 },
	Web2 { network: Web2Network, address: IdentityString },
}

pub type ValidationString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum IdentityMultiSignature {
	/// An Ed25519 signature.
	Ed25519(ed25519::Signature),
	/// An Sr25519 signature.
	Sr25519(sr25519::Signature),
	/// An ECDSA/SECP256k1 signature.
	Ecdsa(ecdsa::Signature),
	/// An ECDSA/keccak256 signature. An Ethereum signature. hash message with keccak256
	Ethereum(EthereumSignature),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TwitterValidationData {
	pub tweet_id: ValidationString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DiscordValidationData {
	pub channel_id: ValidationString,
	pub message_id: ValidationString,
	pub guild_id: ValidationString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Web3CommonValidationData {
	pub message: ValidationString, // or String if under std
	pub signature: IdentityMultiSignature,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Web2ValidationData {
	Twitter(TwitterValidationData),
	Discord(DiscordValidationData),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Web3ValidationData {
	Substrate(Web3CommonValidationData),
	Evm(Web3CommonValidationData),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ValidationData {
	Web2(Web2ValidationData),
	Web3(Web3ValidationData),
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
