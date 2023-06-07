use crate::api_client_patch::parachain::ParachainPatch;
use crate::direct_call::primitives::Request;
use crate::direct_call::trusted_call_signed::TrustedCallSigned;
use crate::sidechain::rpc::SidechainRpcClientTrait;
use crate::sidechain::SidechainResp;
use crate::utils::crypto::encrypt_with_tee_shielding_pubkey;
use crate::utils::hex::ToHexPrefixed;
use crate::ApiClient;
use crate::MultiSignature;
use crate::MultiSigner;
use crate::Pair;
use sp_core::{Decode, Encode};
use substrate_api_client::AccountId;
use substrate_api_client::ApiResult;

use super::primitives::Getter;
use super::primitives::PublicGetter;
use super::primitives::TrustedGetterSigned;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedOperation {
    indirect_call(TrustedCallSigned),
    direct_call(TrustedCallSigned),
    get(Getter),
}

impl From<TrustedCallSigned> for TrustedOperation {
    fn from(item: TrustedCallSigned) -> Self {
        TrustedOperation::direct_call(item)
    }
}

impl From<Getter> for TrustedOperation {
    fn from(item: Getter) -> Self {
        TrustedOperation::get(item)
    }
}

impl From<TrustedGetterSigned> for TrustedOperation {
    fn from(item: TrustedGetterSigned) -> Self {
        TrustedOperation::get(item.into())
    }
}

impl From<PublicGetter> for TrustedOperation {
    fn from(item: PublicGetter) -> Self {
        TrustedOperation::get(item.into())
    }
}

impl TrustedOperation {
    pub fn to_call(&self) -> Option<&TrustedCallSigned> {
        match self {
            TrustedOperation::direct_call(c) => Some(c),
            TrustedOperation::indirect_call(c) => Some(c),
            _ => None,
        }
    }

    pub fn signed_caller_account(&self) -> Option<&AccountId> {
        match self {
            TrustedOperation::direct_call(c) => Some(c.call.sender_account()),
            TrustedOperation::indirect_call(c) => Some(c.call.sender_account()),
            _ => None,
        }
    }
}

pub trait DirectCall {
    fn send_request_di(&self, operation_call: &TrustedOperation) -> ApiResult<SidechainResp>;
    fn getter_request(&self, top: &Getter) -> ApiResult<SidechainResp>;
    fn di_request(&self, operation_call: &TrustedOperation) -> ApiResult<SidechainResp>;
}

impl<P> DirectCall for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn send_request_di(&self, top: &TrustedOperation) -> ApiResult<SidechainResp> {
        match top {
            TrustedOperation::get(getter) => self.getter_request(getter),
            _ => self.di_request(top),
        }
    }

    fn di_request(&self, operation_call: &TrustedOperation) -> ApiResult<SidechainResp> {
        let shard = self.get_shard().unwrap();
        let tee_shielding_key = self.get_tee_shielding_pubkey().unwrap();
        let operation_call_encrypted =
            encrypt_with_tee_shielding_pubkey(&tee_shielding_key, &operation_call.encode());

        // compose jsonrpc call
        let request = Request {
            shard: sp_core::H256(shard),
            cyphertext: operation_call_encrypted,
        };

        use crate::sidechain::json_req;
        let jsonreq = json_req("author_submitAndWatchExtrinsic", vec![request.to_hex()], 1);

        use crate::sidechain::json_resp;
        let res = self.sidechain.request(jsonreq).unwrap();
        let x = json_resp(res).unwrap();
        println!("x: {:?}", x);

        Ok(x)
    }

    fn getter_request(&self, getter: &Getter) -> ApiResult<SidechainResp> {
        let shard = self.get_shard().unwrap();
        let request = Request {
            shard: sp_core::H256(shard),
            cyphertext: getter.encode(),
        };
        use crate::sidechain::json_req;
        let jsonreq = json_req("state_executeGetter", vec![request.to_hex()], 1);
        use crate::sidechain::json_resp;
        let res = self.sidechain.request(jsonreq).unwrap();
        let x = json_resp(res).unwrap();
        println!("x: {:?}", x);

        Ok(x)
    }
}
