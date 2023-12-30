use crate::primitives::{
	aes::RequestAesKey,
	assertion::Assertion,
	getter::Getter,
	identity::Identity,
	keypair::KeyPair,
	network::Web3Network,
	signature::{validation_data::ValidationData, LitentryMultiSignature},
	top::TrustedOperation,
	Index, ShardIdentifier,
};
use codec::{Decode, Encode};
use sp_core::H256;

/// IMPORT: The order of this enum field.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	#[codec(index = 0)]
	link_identity(
		Identity,
		Identity,
		Identity,
		ValidationData,
		Vec<Web3Network>,
		Option<RequestAesKey>,
		H256,
	),
	#[codec(index = 1)]
	deactivate_identity(Identity, Identity, Identity, Option<RequestAesKey>, H256),
	#[codec(index = 2)]
	activate_identity(Identity, Identity, Identity, Option<RequestAesKey>, H256),
	#[codec(index = 3)]
	request_vc(Identity, Identity, Assertion, Option<RequestAesKey>, H256),
	#[codec(index = 4)]
	set_identity_networks(
		Identity,
		Identity,
		Identity,
		Vec<Web3Network>,
		Option<RequestAesKey>,
		H256,
	),
	#[cfg(not(feature = "production"))]
	#[codec(index = 5)]
	remove_identity(Identity, Identity, Vec<Identity>),
}

impl TrustedCall {
	pub fn sign(
		&self,
		pair: &KeyPair,
		nonce: Index,
		mrenclave: &[u8; 32],
		shard: &ShardIdentifier,
	) -> TrustedCallSigned {
		let mut payload = self.encode();
		payload.append(&mut nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		TrustedCallSigned { call: self.clone(), nonce, signature: pair.sign(payload.as_slice()) }
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]

pub struct TrustedCallSigned {
	pub call: TrustedCall,
	pub nonce: Index,
	pub signature: LitentryMultiSignature,
}

impl TrustedCallSigned {
	pub fn into_trusted_operation(
		self,
		direct: bool,
	) -> TrustedOperation<TrustedCallSigned, Getter> {
		match direct {
			true => TrustedOperation::direct_call(self),
			false => TrustedOperation::indirect_call(self),
		}
	}
}
