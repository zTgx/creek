use super::hex::hex_encode;
use crate::primitives::{
	error::ErrorDetail, identity::Identity, signature::validation_data::Web3ValidationData,
	stf_error::StfError, Index,
};
use codec::Encode;
use frame_support::ensure;
use sp_core::blake2_256;
pub type StfResult<T> = Result<T, StfError>;

// verification message format:
// ```
// blake2_256(<sidechain nonce> + <primary account> + <identity-to-be-linked>)
// ```
// where <> means SCALE-encoded
// see https://github.com/litentry/litentry-parachain/issues/1739 and P-174
pub fn get_expected_raw_message(
	who: &Identity,
	identity: &Identity,
	sidechain_nonce: Index,
) -> Vec<u8> {
	let mut payload = Vec::new();
	payload.append(&mut sidechain_nonce.encode());
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	blake2_256(payload.as_slice()).to_vec()
}

pub fn build_msg_web2(who: &Identity, identity: &Identity, sidechain_nonce: Index) -> String {
	let message = get_expected_raw_message(who, identity, sidechain_nonce);
	let msg = hex_encode(message.as_slice());
	msg
}

pub fn verify_web3_identity(
	identity: &Identity,
	raw_msg: &[u8],
	data: &Web3ValidationData,
) -> StfResult<()> {
	ensure!(
		raw_msg == data.message().as_slice(),
		StfError::LinkIdentityFailed(ErrorDetail::UnexpectedMessage)
	);

	ensure!(
		data.signature().verify(raw_msg, identity),
		StfError::LinkIdentityFailed(ErrorDetail::VerifyWeb3SignatureFailed)
	);

	Ok(())
}
