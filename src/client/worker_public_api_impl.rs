use codec::Decode;
use frame_metadata::RuntimeMetadataPrefixed;
use rsa::RsaPublicKey;
use serde::{Serialize, Deserialize};
use sp_core::H256;

use crate::{WorkerPublicApis, Creek, client::service::{RpcReturnValue, SidechainRpcClientTrait}, utils::{hex::{json_req, FromHexPrefixed}, decode_rpc_methods}, CResult, primitives::{ShardIdentifier, crypto::RsaPublicKeyGenerator, EnclaveShieldingPubKey, MrEnclave}};

#[derive(Debug, Serialize, Deserialize)]
pub struct M {
    pub methods: Vec<String>
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

		let metadata = RuntimeMetadataPrefixed::decode(&mut rpc_return_value.value.as_slice()).unwrap();
        Ok(metadata)
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
		let key = RsaPublicKey::new_with_rsa3072_pubkey(rsa_pubkey_json.as_bytes().to_vec())
			.unwrap();

        Ok(key)
    }


}