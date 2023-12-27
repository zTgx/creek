use crate::{
	direct_call::{
		primitives::{
			IMPError, Index, ShardIdentifier, SidechainBlockNumber, UserShieldingKeyType, VCMPError,
		},
		top::TrustedOperation,
		types::{AccountId, KeyPair, Signature},
	},
	primitives::{
		assertion::Assertion,
		identity::{Identity, ValidationData},
		Balance, ChallengeCode, MetadataOf, MrEnclave, ParentchainBlockNumber,
	},
};
use sp_core::{Decode, Encode, H256};
use sp_runtime::traits::Verify;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	balance_set_balance(AccountId, AccountId, Balance, Balance),
	balance_transfer(AccountId, AccountId, Balance),
	balance_unshield(AccountId, AccountId, Balance, ShardIdentifier), /* (AccountIncognito,
	                                                                   * BeneficiaryPublicAccount,
	                                                                   * Amount, Shard) */
	balance_shield(AccountId, AccountId, Balance), // (Root, AccountIncognito, Amount)
	#[cfg(feature = "evm")]
	evm_withdraw(AccountId, H160, Balance), // (Origin, Address EVM Account, Value)
	// (Origin, Source, Target, Input, Value, Gas limit, Max fee per gas, Max priority fee per
	// gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_call(
		AccountId,
		H160,
		H160,
		Vec<u8>,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// (Origin, Source, Init, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce,
	// Access list)
	#[cfg(feature = "evm")]
	evm_create(
		AccountId,
		H160,
		Vec<u8>,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// (Origin, Source, Init, Salt, Value, Gas limit, Max fee per gas, Max priority fee per gas,
	// Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create2(
		AccountId,
		H160,
		Vec<u8>,
		H256,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// litentry
	set_user_shielding_key(AccountId, AccountId, UserShieldingKeyType, H256),
	create_identity(
		AccountId,
		AccountId,
		Identity,
		Option<MetadataOf>,
		ParentchainBlockNumber,
		H256,
	),
	remove_identity(AccountId, AccountId, Identity, H256),
	verify_identity(AccountId, AccountId, Identity, ValidationData, ParentchainBlockNumber, H256),
	request_vc(AccountId, AccountId, Assertion, H256),
	// the following trusted calls should not be requested directly from external
	// they are guarded by the signature check (either root or enclave_signer_account)
	verify_identity_callback(AccountId, AccountId, Identity, ParentchainBlockNumber, H256),
	request_vc_callback(AccountId, AccountId, Assertion, [u8; 32], [u8; 32], Vec<u8>, H256),
	handle_imp_error(AccountId, Option<AccountId>, IMPError, H256),
	handle_vcmp_error(AccountId, Option<AccountId>, VCMPError, H256),
	set_challenge_code(AccountId, AccountId, Identity, ChallengeCode, H256),
	send_erroneous_parentchain_call(AccountId),
	set_scheduled_mrenclave(AccountId, SidechainBlockNumber, MrEnclave),
}

impl TrustedCall {
	pub fn sender_account(&self) -> &AccountId {
		match self {
			TrustedCall::balance_set_balance(sender_account, ..) => sender_account,
			TrustedCall::balance_transfer(sender_account, ..) => sender_account,
			TrustedCall::balance_unshield(sender_account, ..) => sender_account,
			TrustedCall::balance_shield(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_call(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create2(sender_account, ..) => sender_account,
			// litentry
			TrustedCall::set_user_shielding_key(account, ..) => account,
			TrustedCall::create_identity(account, ..) => account,
			TrustedCall::remove_identity(account, ..) => account,
			TrustedCall::verify_identity(account, ..) => account,
			TrustedCall::request_vc(account, ..) => account,
			TrustedCall::verify_identity_callback(account, ..) => account,
			TrustedCall::request_vc_callback(account, ..) => account,
			TrustedCall::set_challenge_code(account, ..) => account,
			TrustedCall::handle_imp_error(account, ..) => account,
			TrustedCall::handle_vcmp_error(account, ..) => account,
			TrustedCall::send_erroneous_parentchain_call(account) => account,
			TrustedCall::set_scheduled_mrenclave(account, ..) => account,
		}
	}

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
	pub signature: Signature,
}

impl TrustedCallSigned {
	pub fn new(call: TrustedCall, nonce: Index, signature: Signature) -> Self {
		TrustedCallSigned { call, nonce, signature }
	}

	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut self.nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());
		self.signature.verify(payload.as_slice(), self.call.sender_account())
	}

	pub fn into_trusted_operation(self) -> TrustedOperation {
		TrustedOperation::direct_call(self)
	}
}
