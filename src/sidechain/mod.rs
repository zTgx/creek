use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};

use crate::ApiClient;

pub trait EnclaveRpc {
    fn rpc_methods(&self);
}

impl<P> EnclaveRpc for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn rpc_methods(&self) {
        
    }
}