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
use codec::Compact;
use core::fmt::Debug;
use sp_core::{blake2_256, crypto::AccountId32, ed25519, sr25519, Decode, Encode, Pair, H256};
use sp_runtime::{
	traits::Verify,
	transaction_validity::{TransactionValidityError, ValidTransaction},
	MultiSignature,
};

use crate::core::trusted_call::LitentryMultiSignature;

use super::{identity::Identity, Index};

pub type Signature = MultiSignature;
pub type AuthorityId = <Signature as Verify>::Signer;
pub type AccountId = AccountId32;
pub type Hash = H256;
pub type BalanceTransferFn = ([u8; 2], AccountId, Compact<u128>);
pub type ShardIdentifier = H256;

#[derive(Clone)]
pub enum KeyPair {
	Sr25519(Box<sr25519::Pair>),
	Ed25519(Box<ed25519::Pair>),
}

impl KeyPair {
	pub fn sign(&self, payload: &[u8]) -> LitentryMultiSignature {
		match self {
			Self::Sr25519(pair) => pair.sign(payload).into(),
			Self::Ed25519(pair) => pair.sign(payload).into(),
		}
	}

	pub fn account_id(&self) -> AccountId {
		match self {
			Self::Sr25519(pair) => pair.public().into(),
			Self::Ed25519(pair) => pair.public().into(),
		}
	}
}

impl From<ed25519::Pair> for KeyPair {
	fn from(x: ed25519::Pair) -> Self {
		KeyPair::Ed25519(Box::new(x))
	}
}

impl From<sr25519::Pair> for KeyPair {
	fn from(x: sr25519::Pair) -> Self {
		KeyPair::Sr25519(Box::new(x))
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedOperation<TCS, G>
where
	TCS: PartialEq + Encode + Debug,
	G: PartialEq + Encode + Debug,
{
	#[codec(index = 0)]
	indirect_call(TCS),
	#[codec(index = 1)]
	direct_call(TCS),
	#[codec(index = 2)]
	get(G),
}

impl<TCS, G> From<G> for TrustedOperation<TCS, G>
where
	TCS: PartialEq + Encode + Debug,
	G: PartialEq + Encode + Debug,
{
	fn from(item: G) -> Self {
		TrustedOperation::get(item)
	}
}

impl<TCS, G> TrustedOperation<TCS, G>
where
	TCS: PartialEq + TrustedCallVerification + Encode + Debug,
	G: PartialEq + Encode + Debug,
{
	pub fn to_call(&self) -> Option<&TCS> {
		match self {
			TrustedOperation::direct_call(c) => Some(c),
			TrustedOperation::indirect_call(c) => Some(c),
			_ => None,
		}
	}

	pub fn signed_caller_account(&self) -> Option<AccountId> {
		match self {
			TrustedOperation::direct_call(c) => c.sender_identity().to_account_id(),
			TrustedOperation::indirect_call(c) => c.sender_identity().to_account_id(),
			_ => None,
		}
	}

	fn validate_trusted_call(trusted_call_signed: &TCS) -> ValidTransaction {
		let from = trusted_call_signed.sender_identity();
		let requires = vec![];
		let provides = vec![(from, trusted_call_signed.nonce()).encode()];

		ValidTransaction { priority: 1 << 20, requires, provides, longevity: 64, propagate: true }
	}

	pub fn hash(&self) -> H256 {
		blake2_256(&self.encode()).into()
	}
}

impl<TCS, G> PoolTransactionValidation for TrustedOperation<TCS, G>
where
	TCS: PartialEq + TrustedCallVerification + Encode + Debug,
	G: PartialEq + Encode + PoolTransactionValidation + Debug,
{
	fn validate(&self) -> Result<ValidTransaction, TransactionValidityError> {
		match self {
			TrustedOperation::direct_call(trusted_call_signed) =>
				Ok(Self::validate_trusted_call(trusted_call_signed)),
			TrustedOperation::indirect_call(trusted_call_signed) =>
				Ok(Self::validate_trusted_call(trusted_call_signed)),
			TrustedOperation::get(getter) => getter.validate(),
		}
	}
}

/// enables TrustedCallSigned verification
pub trait TrustedCallVerification {
	fn sender_identity(&self) -> &Identity;

	fn nonce(&self) -> Index;

	fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool;

	// Litentry: extend the trait for metric statistic purpose
	fn metric_name(&self) -> &'static str;
}

/// validation for top pool
pub trait PoolTransactionValidation {
	fn validate(&self) -> Result<ValidTransaction, TransactionValidityError>;
}
