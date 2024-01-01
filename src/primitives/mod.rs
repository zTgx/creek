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
pub mod aes;
pub mod assertion;
pub mod cerror;
pub mod crypto;
pub mod enclave;
pub mod error;
pub mod getter;
pub mod identity;
pub mod keypair;
pub mod network;
pub mod signature;
pub mod stf_error;
pub mod top;
pub mod trusted_call;
pub mod vc;

use rsa::RsaPublicKey;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::{ConstU32, RuntimeDebug, H256};
use sp_runtime::BoundedVec;

pub use sp_core::{
	blake2_256,
	// crypto::AccountId32 as AccountId,
	ed25519::{Pair as Ed25519Pair, Public as Ed25519Pubkey},
	Pair,
};

use self::cerror::CError;
pub type CResult<T> = std::result::Result<T, CError>;
pub type BlockHash = sp_core::H256;

pub type VCIndex = H256;
pub type Balance = u128;
pub type UserShieldingKeyType = [u8; USER_SHIELDING_KEY_LEN];
pub type Index = u32;
pub type ShardIdentifier = H256;
pub type SidechainBlockNumber = u64;
pub type EnclaveShieldingPubKey = RsaPublicKey;

pub type ParentchainBlockNumber = u32;
type MaxMetadataLength = ConstU32<128>;
pub type MetadataOf = BoundedVec<u8, MaxMetadataLength>;

pub const SGX_MEASUREMENT_SIZE: usize = 32;
pub type MrEnclave = [u8; SGX_MEASUREMENT_SIZE];

// we use 256-bit AES-GCM as user shielding key
pub const USER_SHIELDING_KEY_LEN: usize = 32;
pub const USER_SHIELDING_KEY_NONCE_LEN: usize = 12;

pub use sp_core::ed25519::Public as Ed25519Public;

use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	MultiSignature,
};
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
use codec::{Decode, Encode};

// Litentry: use the name `RsaRequest` to differentiate from `AesRequest` (see aes_request.rs in
// tee-worker) `Rsa` implies that the payload is RSA-encrypted (using enclave's shielding key)
#[macro_export]
macro_rules! decl_rsa_request {
	($($t:meta),*) => {
		#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, $($t),*)]
		pub struct RsaRequest {
			pub shard: ShardIdentifier,
			pub payload: Vec<u8>,
		}
		impl RsaRequest {
			pub fn new(shard: ShardIdentifier, payload: Vec<u8>) -> Self {
				Self { shard, payload }
			}
		}
	};
}

decl_rsa_request!(TypeInfo, RuntimeDebug);

#[derive(Clone, Encode, Decode, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(untagged)]
pub enum Id {
	#[codec(index = 0)]
	Number(u32),
	#[codec(index = 1)]
	Text(String),
}

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
pub struct RpcRequest {
	pub jsonrpc: String,
	pub method: String,
	pub params: Vec<String>,
	pub id: Id,
}

impl RpcRequest {
	pub fn compose_jsonrpc_call(
		id: Id,
		method: String,
		params: Vec<String>,
	) -> Result<String, serde_json::Error> {
		serde_json::to_string(&RpcRequest { jsonrpc: "2.0".to_owned(), method, params, id })
	}
}
