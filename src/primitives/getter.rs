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

use codec::{Decode, Encode};
use sp_runtime::AccountId32;
use crate::{
	if_production_or,
	primitives::{
		identity::Identity,
		keypair::KeyPair, signature::LitentryMultiSignature,
	},
};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Getter {
	#[codec(index = 0)]
	public(PublicGetter),
	#[codec(index = 1)]
	trusted(TrustedGetterSigned),
}

impl Default for Getter {
	fn default() -> Self {
		Getter::public(PublicGetter::some_value)
	}
}
impl From<PublicGetter> for Getter {
	fn from(item: PublicGetter) -> Self {
		Getter::public(item)
	}
}

impl From<TrustedGetterSigned> for Getter {
	fn from(item: TrustedGetterSigned) -> Self {
		Getter::trusted(item)
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PublicGetter {
	#[codec(index = 0)]
	some_value,
	#[codec(index = 1)]
	nonce(Identity),
	#[codec(index = 2)]
	id_graph_hash(Identity),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedGetter {
	#[codec(index = 0)]
	free_balance(Identity),
	#[codec(index = 1)]
	reserved_balance(Identity),
	// litentry
	#[codec(index = 5)]
	id_graph(Identity),
	#[codec(index = 6)]
	id_graph_stats(Identity),
}

impl TrustedGetter {
	pub fn sender_identity(&self) -> &Identity {
		match self {
			TrustedGetter::free_balance(sender_identity) => sender_identity,
			TrustedGetter::reserved_balance(sender_identity) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_nonce(sender_identity) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_codes(sender_identity, _) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_storages(sender_identity, ..) => sender_identity,
			// litentry
			TrustedGetter::id_graph(sender_identity) => sender_identity,
			TrustedGetter::id_graph_stats(sender_identity) => sender_identity,
		}
	}

	pub fn sign(&self, pair: &KeyPair) -> TrustedGetterSigned {
		let signature = pair.sign(self.encode().as_slice());
		TrustedGetterSigned { getter: self.clone(), signature }
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct TrustedGetterSigned {
	pub getter: TrustedGetter,
	pub signature: LitentryMultiSignature,
}

impl TrustedGetterSigned {
	pub fn new(getter: TrustedGetter, signature: LitentryMultiSignature) -> Self {
		TrustedGetterSigned { getter, signature }
	}

	pub fn verify_signature(&self) -> bool {
		// in non-prod, we accept signature from Alice too
		if_production_or!(
			{
				self.signature
					.verify(self.getter.encode().as_slice(), self.getter.sender_identity())
			},
			{
				self.signature
					.verify(self.getter.encode().as_slice(), self.getter.sender_identity()) ||
					self.signature
						.verify(self.getter.encode().as_slice(), &ALICE_ACCOUNTID32.into())
			}
		)
	}
}
use hex_literal::hex;

pub const ALICE_ACCOUNTID32: AccountId32 =
	AccountId32::new(hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"]);
