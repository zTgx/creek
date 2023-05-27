use crate::api_client_patch::parachain::ParachainPatch;
use crate::direct_call::primitives::Request;
use crate::direct_call::primitives::ShardIdentifier;
use crate::direct_call::trusted_call_signed::TrustedCallSigned;
use crate::sidechain::RpcRequest;
use crate::sidechain::rpc::SidechainRpcClientTrait;
use crate::utils::crypto::encrypt_with_tee_shielding_pubkey;
use crate::utils::hex::ToHexPrefixed;
use crate::ApiClient;
use crate::MultiSignature;
use crate::MultiSigner;
use crate::Pair;
use sp_core::{Decode, Encode};
use substrate_api_client::AccountId;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedOperation {
    // indirect_call(TrustedCallSigned),
    direct_call(TrustedCallSigned),
    // get(Getter),
}

impl From<TrustedCallSigned> for TrustedOperation {
    fn from(item: TrustedCallSigned) -> Self {
        TrustedOperation::direct_call(item)
    }
}

// impl From<Getter> for TrustedOperation {
// 	fn from(item: Getter) -> Self {
// 		TrustedOperation::get(item)
// 	}
// }

// impl From<TrustedGetterSigned> for TrustedOperation {
// 	fn from(item: TrustedGetterSigned) -> Self {
// 		TrustedOperation::get(item.into())
// 	}
// }

// impl From<PublicGetter> for TrustedOperation {
// 	fn from(item: PublicGetter) -> Self {
// 		TrustedOperation::get(item.into())
// 	}
// }

impl TrustedOperation {
    pub fn to_call(&self) -> Option<&TrustedCallSigned> {
        match self {
            TrustedOperation::direct_call(c) => Some(c),
            // TrustedOperation::indirect_call(c) => Some(c),
            // _ => None,
        }
    }

    pub fn signed_caller_account(&self) -> Option<&AccountId> {
        match self {
            TrustedOperation::direct_call(c) => Some(c.call.sender_account()),
            // TrustedOperation::indirect_call(c) => Some(c.call.sender_account()),
            // _ => None,
        }
    }
}

// let top: TrustedOperation =
// TrustedGetter::evm_account_storages(sender_acc, execution_address, H256::zero())
// 	.sign(&KeyPair::Sr25519(Box::new(sender)))
// 	.into();
// let res = perform_trusted_operation(cli, trusted_args, &top);

pub(crate) fn perform_trusted_operation(
    shard: &ShardIdentifier,
    top: &TrustedOperation,
) -> Option<Vec<u8>> {
    match top {
        // TrustedOperation::indirect_call(_) => send_request(cli, trusted_args, top),
        TrustedOperation::direct_call(_) => send_direct_request(shard, top),
        // TrustedOperation::get(getter) => execute_getter_from_cli_args(cli, trusted_args, getter),
    }
}

/// sends a rpc watch request to the worker api server
fn send_direct_request(
    _shard: &ShardIdentifier,
    _operation_call: &TrustedOperation,
) -> Option<Vec<u8>> {
    None
    // let encryption_key = self.get_tee_shielding_key();
    // let jsonrpc_call: String = get_json_request(shard, operation_call);

    // debug!("setup sender and receiver");
    // let resp = self.sidechain.request(jsonreq);

    // debug!("waiting for rpc response");
    // loop {
    // 	match receiver.recv() {
    // 		Ok(response) => {
    // 			debug!("received response");
    // 			let response: RpcResponse = serde_json::from_str(&response).unwrap();
    // 			if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
    // 				debug!("successfully decoded rpc response: {:?}", return_value);
    // 				match return_value.status {
    // 					DirectRequestStatus::Error => {
    // 						debug!("request status is error");
    // 						if let Ok(value) = String::decode(&mut return_value.value.as_slice()) {
    // 							println!("[Error] {}", value);
    // 						}
    // 						direct_api.close().unwrap();
    // 						return None
    // 					},
    // 					DirectRequestStatus::TrustedOperationStatus(status) => {
    // 						debug!("request status is: {:?}", status);
    // 						if let Ok(value) = Hash::decode(&mut return_value.value.as_slice()) {
    // 							println!("Trusted call {:?} is {:?}", value, status);
    // 						}
    // 						if connection_can_be_closed(status) {
    // 							direct_api.close().unwrap();
    // 						}
    // 					},
    // 					_ => {
    // 						debug!("request status is ignored");
    // 						direct_api.close().unwrap();
    // 						return None
    // 					},
    // 				}
    // 				if !return_value.do_watch {
    // 					debug!("do watch is false, closing connection");
    // 					direct_api.close().unwrap();
    // 					return None
    // 				}
    // 			};
    // 		},
    // 		Err(e) => {
    // 			error!("failed to receive rpc response: {:?}", e);
    // 			direct_api.close().unwrap();
    // 			return None
    // 		},
    // 	};
    // }
}

// pub(crate) fn get_json_request(
// 	shard: ShardIdentifier,
// 	operation_call: &TrustedOperation,
// ) -> String {
// 	let tee_shielding_key = get_tee_shielding_pubkey().unwrap();
// 	let operation_call_encrypted = encrypt_with_tee_shielding_pubkey(&tee_shielding_key, &operation_call.encode());

// 	// let operation_call_encrypted = shielding_pubkey.encrypt(&operation_call.encode()).unwrap();

// 	// compose jsonrpc call
// 	let request = Request { shard, cyphertext: operation_call_encrypted };
// 	RpcRequest::compose_jsonrpc_call(
// 		"author_submitAndWatchExtrinsic".to_string(),
// 		vec![request.to_hex()],
// 	)
// 	.unwrap()
// }

pub trait DirectCall {
    fn send_request_di(&self, operation_call: &TrustedOperation);
}

impl<P> DirectCall for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn send_request_di(&self, operation_call: &TrustedOperation) {
        let shard = self.get_shard().unwrap();
        let tee_shielding_key = self.get_tee_shielding_pubkey().unwrap();
        let operation_call_encrypted =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_key, &operation_call.encode());

        // let operation_call_encrypted = shielding_pubkey.encrypt(&operation_call.encode()).unwrap();

        // compose jsonrpc call
        let request = Request {
            shard: sp_core::H256(shard),
            cyphertext: operation_call_encrypted,
        };
        
		// let r = RpcRequest::compose_jsonrpc_call(
        //     "author_submitAndWatchExtrinsic".to_string(),
        //     vec![request.to_hex()],
        // )
        // .unwrap();

		use crate::sidechain::json_req;
        let jsonreq = json_req("author_submitAndWatchExtrinsic", vec![request.to_hex()], 1);

		use crate::sidechain::json_resp;
		let res = self.sidechain.request(jsonreq).unwrap();
		let x = json_resp(res).unwrap();
		println!("x: {:?}", x);

    }
}