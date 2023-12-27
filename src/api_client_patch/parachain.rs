use crate::{
	identity_management::IDENTITY_PALLET_NAME,
	primitives::{
		address::Address32, crypto::RsaPublicKeyGenerator, enclave::Enclave, vc::VCContext,
		AccountId, MrEnclave, VCIndex,
	},
	utils::address::vec_to_u8_array,
	vc_management::VC_PALLET_NAME,
	ApiClient,
};
use rsa::RsaPublicKey;
use sp_core::{ed25519::Public as Ed25519Public, hexdisplay::HexDisplay, Pair};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{ApiClientError, ApiResult};

pub const TEEREX_STORAGE_PREFIX_NAME: &str = "Teerex";

pub trait ParachainPatch {
	// IdentityManagement pallet
	fn delegatee(&self, account: Address32) -> ApiResult<Option<()>>;

	// Teerex pallet
	fn enclave_count(&self) -> ApiResult<Option<u64>>;
	fn enclave(&self, enclave_count: u64) -> ApiResult<Option<Enclave<AccountId, String>>>;
	fn get_shard(&self) -> ApiResult<MrEnclave>;
	fn get_tee_shielding_pubkey(&self) -> ApiResult<RsaPublicKey>;
	fn get_vc_pubkey(&self) -> ApiResult<Ed25519Public>;
	fn get_vc_context(&self, vc_index: &VCIndex) -> ApiResult<Option<VCContext>>;
}

impl<P> ParachainPatch for ApiClient<P>
where
	P: Pair,
	MultiSignature: From<P::Signature>,
	MultiSigner: From<P::Public>,
{
	/**
	 * pallet IdentityManagement Apis
	 */
	fn delegatee(&self, account: Address32) -> ApiResult<Option<()>> {
		self.api.get_storage_map(IDENTITY_PALLET_NAME, "Delegatee", account, None)
	}

	/**
	 * pallet Teerex Apis
	 */
	fn enclave_count(&self) -> ApiResult<Option<u64>> {
		self.api.get_storage_value(TEEREX_STORAGE_PREFIX_NAME, "EnclaveCount", None)
	}

	fn enclave(&self, enclave_count: u64) -> ApiResult<Option<Enclave<AccountId, String>>> {
		self.api
			.get_storage_map(TEEREX_STORAGE_PREFIX_NAME, "EnclaveRegistry", enclave_count, None)
	}

	fn get_tee_shielding_pubkey(&self) -> ApiResult<RsaPublicKey> {
		let enclave_count: Option<u64> = self.enclave_count()?;
		let enclave_count = enclave_count.ok_or_else(|| {
			ApiClientError::Other("[+] get enclave count error".to_string().into())
		})?;

		let enclave: Option<Enclave<AccountId, String>> = self.enclave(enclave_count)?;
		let enclave = enclave
			.ok_or_else(|| ApiClientError::Other("[+] get enclave error".to_string().into()))?;

		let shielding_key = enclave.shielding_key.ok_or_else(|| {
			ApiClientError::Other("[+] get tee shielding pubkey error".to_string().into())
		})?;

		RsaPublicKey::new_with_rsa3072_pubkey(shielding_key).map_err(|e| {
			ApiClientError::Other(format!("Generate Rsa pubkey error: {:?}", e).into())
		})
	}

	fn get_vc_pubkey(&self) -> ApiResult<Ed25519Public> {
		let enclave_count: Option<u64> = self.enclave_count()?;
		let enclave_count = enclave_count.ok_or_else(|| {
			ApiClientError::Other("[+] get enclave count error".to_string().into())
		})?;

		let enclave: Option<Enclave<AccountId, String>> = self.enclave(enclave_count)?;
		let enclave = enclave
			.ok_or_else(|| ApiClientError::Other("[+] get enclave error".to_string().into()))?;

		let vc_pubkey = enclave
			.vc_pubkey
			.ok_or_else(|| ApiClientError::Other("[+] get vc_pubkey error".to_string().into()))?;

		Ok(Ed25519Public(vec_to_u8_array::<32>(vc_pubkey)))
	}

	/// There're two methos to get the mrenclave
	/// 1. Online -> to use this method `get_shard` or
	/// 2. Offline -> to `litentry-parachain/tee-worker` run `make enclave`
	/// Both should be display exactly same value.
	///
	/// TODO:
	/// But there's a question, what's the difference betwwen `mrenclave` and `shard`?
	fn get_shard(&self) -> ApiResult<MrEnclave> {
		let enclave_count: Option<u64> = self.enclave_count()?;
		let enclave_count = enclave_count.ok_or_else(|| {
			ApiClientError::Other("[+] get enclave count error".to_string().into())
		})?;

		let enclave: Option<Enclave<AccountId, String>> = self.enclave(enclave_count)?;
		let enclave = enclave
			.ok_or_else(|| ApiClientError::Other("[+] get enclave error".to_string().into()))?;

		let shard = enclave.mr_enclave;
		let shard_in_hex = format!("0x{}", HexDisplay::from(&shard));

		println!("\n ✅ Get shard from parachain : {}", shard_in_hex);

		Ok(shard)
	}

	fn get_vc_context(&self, vc_index: &VCIndex) -> ApiResult<Option<VCContext>> {
		self.api.get_storage_map(VC_PALLET_NAME, "VCRegistry", vc_index, None)
	}
}
