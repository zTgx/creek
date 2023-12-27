use codec::Encode;
use sp_core::{blake2_256, sr25519::Pair as SubstratePair, Pair};

use crate::primitives::{
	address::Address32,
	identity::{
		Identity, IdentityMultiSignature, TwitterValidationData, ValidationData, ValidationString,
		Web2ValidationData, Web3CommonValidationData, Web3ValidationData,
	},
	ChallengeCode,
};

use super::hex::hex_encode;

pub trait ValidationDataBuilder {
	fn build_vdata_substrate(
		pair: &SubstratePair,
		who: &Address32,
		identity: &Identity,
		code: &ChallengeCode,
	) -> Result<ValidationData, Vec<u8>>;

	fn build_vdata_twitter(tweet_id: &ValidationString) -> Result<ValidationData, Vec<u8>>;
}

impl ValidationDataBuilder for ValidationData {
	fn build_vdata_substrate(
		pair: &SubstratePair,
		who: &Address32,
		identity: &Identity,
		challenge_code: &ChallengeCode,
	) -> Result<ValidationData, Vec<u8>> {
		let message = get_expected_raw_message(who, identity, challenge_code);
		let sr25519_sig = pair.sign(&message);
		let signature = IdentityMultiSignature::Sr25519(sr25519_sig);
		let message = ValidationString::try_from(message)?;

		let web3_common_validation_data = Web3CommonValidationData { message, signature };
		Ok(ValidationData::Web3(Web3ValidationData::Substrate(web3_common_validation_data)))
	}

	fn build_vdata_twitter(tweet_id: &ValidationString) -> Result<ValidationData, Vec<u8>> {
		let twitter_vdata = TwitterValidationData { tweet_id: tweet_id.clone() };
		Ok(ValidationData::Web2(Web2ValidationData::Twitter(twitter_vdata)))
	}
}

fn get_expected_raw_message(who: &Address32, identity: &Identity, code: &ChallengeCode) -> Vec<u8> {
	let mut payload = code.encode();
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	blake2_256(payload.as_slice()).to_vec()
}

pub fn build_msg_web2(
	who: &Address32,
	identity: &Identity,
	challenge_code: &ChallengeCode,
) -> String {
	let message = get_expected_raw_message(who, identity, challenge_code);
	let msg = hex_encode(message.as_slice());
	msg
}
