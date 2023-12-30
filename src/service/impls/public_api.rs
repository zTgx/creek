use crate::{
	primitives::{
		crypto::RsaPublicKeyGenerator, AccountId, Ed25519Pubkey, EnclaveShieldingPubKey, Index,
		MrEnclave, ShardIdentifier,
	},
	service::{json::json_req, wsclient::SidechainRpcClientTrait},
	utils::{
		hex::FromHexPrefixed,
		public_api::{
			decode_accountid, decode_mr_enclave, decode_nonce, decode_rpc_methods,
			decode_rpc_return_value, decode_runtime_metadata, decode_shard_identifier,
			decode_string,
		},
	},
	CResult, Creek, WorkerPublicApis,
};
use frame_metadata::RuntimeMetadataPrefixed;
use rsa::RsaPublicKey;

impl WorkerPublicApis for Creek {
	fn rpc_methods(&self) -> CResult<Vec<String>> {
		let jsonreq = json_req("rpc_methods", [0; 0], 1);
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
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let mrenclave = decode_mr_enclave(&rpc_return_value)?;
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
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let metadata = decode_runtime_metadata(&rpc_return_value)?;
		Ok(metadata)
	}

	fn state_get_storage(
		&self,
		mrenclave_in_base58: String,
		storage_key_in_hex: String,
	) -> CResult<Vec<u8>> {
		let jsonreq = json_req("state_getStorage", [mrenclave_in_base58, storage_key_in_hex], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		Ok(rpc_return_value.value)
	}

	fn author_get_untrusted_url(&self) -> CResult<String> {
		let jsonreq = json_req("author_getUntrustedUrl", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let untrusted_url = decode_string(&rpc_return_value)?;
		println!("[Untrusted-URL]: {:?}", untrusted_url);
		Ok(untrusted_url)
	}

	/// "localhost:3443"
	/// {"id":"1","jsonrpc":"2.0","result":"0x3c386c6f63616c686f73743a333434330000"}
	fn author_get_mu_ra_url(&self) -> CResult<String> {
		let jsonreq = json_req("author_getMuRaUrl", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let mu_ra_url = decode_string(&rpc_return_value)?;
		println!("[MU-RA-URL]: {:?}", mu_ra_url);
		Ok(mu_ra_url)
	}

	fn author_get_shard(&self) -> CResult<ShardIdentifier> {
		const METHOD_NAME: &str = "author_getShard";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let shard = decode_shard_identifier(&rpc_return_value)?;
		println!("[SHARD]: {:?}", shard);
		Ok(shard)
	}

	fn author_get_shielding_key(&self) -> CResult<EnclaveShieldingPubKey> {
		let jsonreq = json_req("author_getShieldingKey", [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq)?;
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let rsa_pubkey_json = decode_string(&rpc_return_value)?;
		println!("[RSA PUBKEY]: {}", rsa_pubkey_json);
		let key =
			RsaPublicKey::new_with_rsa3072_pubkey(rsa_pubkey_json.as_bytes().to_vec()).unwrap();

		Ok(key)
	}

	fn author_get_shard_vault(&self) -> CResult<AccountId> {
		const METHOD_NAME: &str = "author_getShardVault";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let shard_vault = decode_accountid(&rpc_return_value)?;
		println!("[SHARD-Vault]: {:?}", shard_vault);
		Ok(shard_vault)
	}

	fn author_get_enclave_signer_account(&self) -> CResult<Ed25519Pubkey> {
		const METHOD_NAME: &str = "author_getEnclaveSignerAccount";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let jsonresp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let enclave_signer_public_key = decode_string(&rpc_return_value)?;
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
		let rpc_return_value = decode_rpc_return_value(&jsonresp)?;
		let next_nonce = decode_nonce(&rpc_return_value)?;
		println!("[SIDECHAIN NONCE]: {}", next_nonce);
		Ok(next_nonce)
	}
}
