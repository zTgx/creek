use super::aes_request::{DecryptableRequest, ShieldingCryptoDecrypt};
use crate::primitives::ShardIdentifier;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use std::fmt::Debug;

// Litentry: use the name `RsaRequest` to differentiate from `AesRequest` (see aes_request.rs in
// tee-worker) `Rsa` implies that the payload is RSA-encrypted (using enclave's shielding key)
#[macro_export]
macro_rules! decl_rsa_request {
	($($t:meta),*) => {
		#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, $($t),*)]
		pub struct RsaRequest {
			pub shard: ShardIdentifier,
			pub payload: Vec<u8>,
		}
		impl RsaRequest {
			pub fn new(shard: ShardIdentifier, payload: Vec<u8>) -> Self {
				Self { shard, payload }
			}
		}
	};
}

decl_rsa_request!(TypeInfo, RuntimeDebug);

impl DecryptableRequest for RsaRequest {
	type Error = ();

	fn shard(&self) -> ShardIdentifier {
		self.shard
	}

	fn payload(&self) -> &[u8] {
		self.payload.as_slice()
	}

	fn decrypt<T: Debug>(
		&mut self,
		enclave_shielding_key: Box<dyn ShieldingCryptoDecrypt<Error = T>>,
	) -> core::result::Result<Vec<u8>, ()> {
		enclave_shielding_key.decrypt(self.payload.as_slice()).map_err(|_| ())
	}
}
