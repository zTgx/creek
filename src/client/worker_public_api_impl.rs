use codec::Decode;
use frame_metadata::RuntimeMetadataPrefixed;
use rsa::RsaPublicKey;
use sp_core::H256;

use crate::{
	client::service::{RpcReturnValue, SidechainRpcClientTrait},
	primitives::{
		crypto::RsaPublicKeyGenerator, AccountId, Ed25519Pubkey, EnclaveShieldingPubKey, Index,
		MrEnclave, ShardIdentifier,
	},
	utils::{
		decode_rpc_methods,
		hex::{json_req, FromHexPrefixed},
	},
	CResult, Creek, WorkerPublicApis,
};

pub trait WorkerApiWatcher {
	fn add(&mut self);
	fn tick(&self) -> bool; // If need upgrade or not
}

impl WorkerPublicApis for Creek {
	fn rpc_methods(&self) -> CResult<Vec<String>> {
		let jsonreq = json_req("rpc_methods", [0_u8; 0], 1);
		let resp = self.client().request(jsonreq)?;

		let methods = decode_rpc_methods(&resp);
		println!("[RPC-METHODS]: {:#?}", methods);
		Ok(methods)
	}

	/// { id: "1", jsonrpc: "2.0", result: "hello, world" }
	fn system_version(&self) -> CResult<String> {
		let jsonreq = json_req("system_version", [0_u8; 0], 1);
		let resp = self.client().request(jsonreq)?;
		Ok(resp.result)
	}

	fn system_name(&self) -> CResult<String> {
		let jsonreq = json_req("system_name", [0_u8; 0], 1);
		let resp = self.client().request(jsonreq)?;
		Ok(resp.result)
	}

	fn system_health(&self) -> CResult<String> {
		let jsonreq = json_req("system_health", [0_u8; 0], 1);
		let resp = self.client().request(jsonreq)?;
		Ok(resp.result)
	}

	fn state_get_mrenclave(&self) -> CResult<MrEnclave> {
		let jsonreq = json_req("state_getMrenclave", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();

		let mrenclave = MrEnclave::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		println!("[MRENCLAVE in hex]: {:?}", hex::encode(mrenclave));
		Ok(mrenclave)
	}

	fn state_get_runtime_version(&self) -> CResult<String> {
		let jsonreq = json_req("state_getRuntimeVersion", [0_u8; 0], 1);
		let resp = self.client().request(jsonreq)?;
		Ok(resp.result)
	}

	fn state_get_metadata(&self) -> CResult<RuntimeMetadataPrefixed> {
		let jsonreq = json_req("state_getMetadata", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();

		let metadata =
			RuntimeMetadataPrefixed::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		Ok(metadata)
	}

	fn state_get_storage(
		&self,
		mrenclave_in_base58: String,
		storage_key_in_hex: String,
	) -> CResult<Vec<u8>> {
		let jsonreq = json_req("state_getStorage", [mrenclave_in_base58, storage_key_in_hex], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();
		Ok(rpc_return_value.value)
	}

	fn author_get_untrusted_url(&self) -> CResult<String> {
		let jsonreq = json_req("author_getUntrustedUrl", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();
		let untrusted_url = String::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		println!("[Untrusted-URL]: {:?}", untrusted_url);
		Ok(untrusted_url)
	}

	/// "localhost:3443"
	/// {"id":"1","jsonrpc":"2.0","result":"0x3c386c6f63616c686f73743a333434330000"}
	fn author_get_mu_ra_url(&self) -> CResult<String> {
		let jsonreq = json_req("author_getMuRaUrl", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();
		let mu_ra_url = String::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		println!("[MU-RA-URL]: {:?}", mu_ra_url);
		Ok(mu_ra_url)
	}

	fn author_get_shard(&self) -> CResult<ShardIdentifier> {
		const METHOD_NAME: &str = "author_getShard";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let resp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&resp.result).unwrap();
		let shard = H256::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		println!("[SHARD]: {:?}", shard);
		Ok(shard)
	}

	fn author_get_shielding_key(&self) -> CResult<EnclaveShieldingPubKey> {
		let jsonreq = json_req("author_getShieldingKey", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();

		let rsa_pubkey_json = String::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		let key =
			RsaPublicKey::new_with_rsa3072_pubkey(rsa_pubkey_json.as_bytes().to_vec()).unwrap();

		Ok(key)
	}

	fn author_get_shard_vault(&self) -> CResult<AccountId> {
		const METHOD_NAME: &str = "author_getShardVault";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let resp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&resp.result).unwrap();
		let shard_vault = AccountId::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		println!("[SHARD-Vault]: {:?}", shard_vault);
		Ok(shard_vault)
	}

	fn author_get_enclave_signer_account(&self) -> CResult<Ed25519Pubkey> {
		const METHOD_NAME: &str = "author_getEnclaveSignerAccount";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let resp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&resp.result).unwrap();
		let enclave_signer_public_key =
			String::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		let enclave_signer_public_key =
			Ed25519Pubkey::from_hex(&enclave_signer_public_key).unwrap();
		println!("[enclave_signer_public_key]: {:?}", enclave_signer_public_key);
		Ok(enclave_signer_public_key)
	}

	fn author_get_next_nonce(
		&self,
		shard_in_base58: String,
		account_in_hex: String,
	) -> CResult<Index> {
		const METHOD_NAME: &str = "author_getNextNonce";
		let jsonreq = json_req(METHOD_NAME, (shard_in_base58, account_in_hex), 1);
		let jsonresp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&jsonresp.result).unwrap();
		let next_nonce = Index::decode(&mut rpc_return_value.value.as_slice()).unwrap();
		Ok(next_nonce)
	}
}
