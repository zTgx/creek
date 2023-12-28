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

pub mod address;
pub mod assertion;
pub mod crypto;
pub mod enclave;
pub mod error;
pub mod ethereum;
pub mod identity;
pub mod network;
pub mod vc;

use rsa::RsaPublicKey;
use sp_core::{ConstU32, H256};
use sp_runtime::BoundedVec;

pub use sp_core::crypto::AccountId32 as AccountId;

use self::error::CError;
pub type CResult<T> = std::result::Result<T, CError>;

pub type VCIndex = H256;
pub type Balance = u128;
pub type UserShieldingKeyType = [u8; USER_SHIELDING_KEY_LEN];
pub type Index = u32;
pub type ShardIdentifier = H256;
pub type SidechainBlockNumber = u64;
pub type EnclaveShieldingPubKey = RsaPublicKey;

pub const CHALLENGE_CODE_SIZE: usize = 16;
pub type ChallengeCode = [u8; CHALLENGE_CODE_SIZE];

type MaxStringLength = ConstU32<64>;
pub type IdentityString = BoundedVec<u8, MaxStringLength>;
pub type ErrorString = BoundedVec<u8, MaxStringLength>;

pub type ParentchainBlockNumber = u32;

type MaxMetadataLength = ConstU32<128>;
pub type MetadataOf = BoundedVec<u8, MaxMetadataLength>;

pub const SGX_MEASUREMENT_SIZE: usize = 32;
pub type MrEnclave = [u8; SGX_MEASUREMENT_SIZE];

pub type BlockHash = sp_core::H256;

// we use 256-bit AES-GCM as user shielding key
pub const USER_SHIELDING_KEY_LEN: usize = 32;
pub const USER_SHIELDING_KEY_NONCE_LEN: usize = 12;
pub const USER_SHIELDING_KEY_TAG_LEN: usize = 16;
