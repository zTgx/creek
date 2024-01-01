use crate::{
	primitives::{
		assertion::Assertion,
		cerror::CError,
		identity::Identity,
		keypair::KeyPair,
		network::Web3Network,
		signature::validation_data::{
			TwitterValidationData, ValidationData, Web2ValidationData, Web3CommonValidationData,
			Web3ValidationData,
		},
		CResult,
	},
	service::{
		impls::{get_rsa_request, stf_inner::LinkIdentityInner},
		json::{json_req, RpcReturnValue},
		wsclient::SidechainRpcRequest,
	},
	utils::{self, hex::FromHexPrefixed},
	Creek, ValidationDataBuilder, WorkerGetters, WorkerSTF,
};
use utils::identity::{get_expected_raw_message, verify_web3_identity};

use super::CreekHelper;

impl WorkerSTF for Creek {
	fn link_identity(
		&self,
		link_identity: Identity,
		networks: Vec<Web3Network>,
		vdata: ValidationData,
	) -> CResult<()> {
		let shard = self.author_get_shard()?;
		let shielding_pubkey = self.author_get_shielding_key()?;

		let trusted_call_signed =
			self.link_identity_inner(link_identity, networks, &shard, vdata)?;

		let param = get_rsa_request(shard, trusted_call_signed, shielding_pubkey);
		let jsonreq = json_req("author_submitAndWatchRsaRequest", [param], 1);
		let jsonresp = self.worker_client.request(jsonreq)?;

		let rpc_return_value =
			RpcReturnValue::from_hex(&jsonresp.result).map_err(CError::HexError)?;

		println!("[LINK IDENTITY]: {:#?}", rpc_return_value);

		Ok(())
	}

	fn request_vc(&self, assertion: Assertion) -> CResult<()> {
		let shard = self.author_get_shard()?;
		let shielding_pubkey = self.author_get_shielding_key()?;

		let trusted_call_signed = self.request_vc_inner(&shard, assertion)?;

		let param = get_rsa_request(shard, trusted_call_signed, shielding_pubkey);
		let jsonreq = json_req("author_submitAndWatchRsaRequest", [param], 1);
		let jsonresp = self.worker_client.request(jsonreq)?;

		let rpc_return_value =
			RpcReturnValue::from_hex(&jsonresp.result).map_err(CError::HexError)?;

		println!("[REQUEST VC]: {:#?}", rpc_return_value);

		Ok(())
	}
}

impl ValidationDataBuilder for Creek {
	fn twitter_vdata(&self, twitterid: &str) -> CResult<ValidationData> {
		Ok(ValidationData::Web2(Web2ValidationData::Twitter(TwitterValidationData {
			tweet_id: twitterid.to_string(),
		})))
	}

	fn web3_vdata(&self, keypair: &KeyPair) -> CResult<ValidationData> {
		let sidechain_nonce = self.get_sidechain_nonce()?;

		// 1. Get raw message
		let primary = Identity::from(self.signer.account_id());
		let identity = Identity::from(keypair.account_id());
		if identity.is_web2() {
			return Err(CError::Other("Web3 Identity supported ONLY!".to_string()))
		}

		let message_raw = get_expected_raw_message(&primary, &identity, sidechain_nonce);

		// 2. Sign raw message
		let signature = keypair.sign(&message_raw);

		// 3. Build ValidationData
		let web3_common_validation_data =
			Web3CommonValidationData { message: message_raw.clone(), signature };

		match identity {
			Identity::Substrate(_) =>
				Some(Web3ValidationData::Substrate(web3_common_validation_data)),
			Identity::Evm(_) => Some(Web3ValidationData::Evm(web3_common_validation_data)),
			Identity::Bitcoin(_) => Some(Web3ValidationData::Evm(web3_common_validation_data)),
			_ => None,
		}
		.map(|vdata| {
			// 4. Verify
			verify_web3_identity(&identity, &message_raw, &vdata)
				.expect("VerifyWeb3SignatureFailed");

			vdata
		})
		.map(ValidationData::Web3)
		.ok_or(CError::APIError)
	}
}
