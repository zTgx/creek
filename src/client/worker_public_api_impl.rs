use codec::Decode;
use rsa::RsaPublicKey;
use sp_core::H256;

use crate::{WorkerPublicApis, Creek, client::service::{RpcReturnValue, SidechainRpcClientTrait}, utils::hex::{json_req, remove_whitespace, FromHexPrefixed}, CResult, primitives::{ShardIdentifier, crypto::RsaPublicKeyGenerator, EnclaveShieldingPubKey}};

impl WorkerPublicApis for Creek {
    fn rpc_methods(&self) {
        let jsonreq = json_req("rpc_methods", [0_u8; 0], 1);
        let resp = self.client().request(jsonreq).unwrap();
        let mut sresult = remove_whitespace(&resp.result);
        sresult.remove_matches("methods:[");
        sresult.remove_matches("]");

        let mut supported_methods = vec![];
        let methods: Vec<&str> = sresult.split(',').collect();
        methods.iter().for_each(|m| {
            supported_methods.push(m.to_string());
        });

        println!(">> supported_methods: {:?}", supported_methods);
    }

    fn author_get_shard(&self) -> CResult<ShardIdentifier> {
        const METHOD_NAME: &str = "author_getShard";
		let jsonreq = json_req(METHOD_NAME, [0_u8; 0], 1);
		let resp = self.client().request(jsonreq).unwrap();
		let rpc_return_value = RpcReturnValue::from_hex(&resp.result).unwrap();
		let shard = H256::decode(&mut rpc_return_value.value.as_slice()).unwrap();
        println!("[MRENCLAVE]: {:?}", shard);
        Ok(shard)
    }

    fn author_get_shielding_key(&self) -> CResult<EnclaveShieldingPubKey> {
        let jsonreq = json_req("author_getShieldingKey", [0_u8; 0], 1);
		let resp = self.client().request(jsonreq)?;
		let shielding_pubkey_string = decode_from_rpc_response(&resp)?;
		let key = RsaPublicKey::new_with_rsa3072_pubkey(shielding_pubkey_string.as_bytes().to_vec())
			.unwrap();

        Ok(key)
    }
}