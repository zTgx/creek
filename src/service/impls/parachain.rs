use crate::{
	primitives::{
		address::Address32, cerror::CError, crypto::RsaPublicKeyGenerator, enclave::Enclave,
		vc::VCContext, AccountId, CResult, MrEnclave,
	},
	utils::address::vec_to_u8_array,
	Creek, ParachainOp,
};
use rsa::RsaPublicKey;
use sp_core::{ed25519::Public as Ed25519Public, hexdisplay::HexDisplay};
use substrate_api_client::{ac_primitives::StorageKey, GetStorage};

const TEEREX_STORAGE_PREFIX_NAME: &str = "Teerex";
const IDENTITY_PALLET_NAME: &str = "IdentityMangement";

impl ParachainOp for Creek {
	fn delegatee(&self, account: Address32) -> CResult<Option<()>> {
		self.parachain_client
			.api
			.get_storage_map(IDENTITY_PALLET_NAME, "Delegatee", account, None)
			.map_err(|_| CError::APIError)
	}

	fn enclave_count(&self) -> CResult<Option<u64>> {
		self.parachain_client
			.api
			.get_storage(TEEREX_STORAGE_PREFIX_NAME, "EnclaveCount", None)
			.map_err(|_| CError::APIError)
	}

	fn enclave(&self, enclave_count: u64) -> CResult<Option<Enclave<AccountId, String>>> {
		self.parachain_client
			.api
			.get_storage_map(TEEREX_STORAGE_PREFIX_NAME, "EnclaveRegistry", enclave_count, None)
			.map_err(|_| CError::APIError)
	}

	fn get_tee_shielding_pubkey(&self) -> CResult<RsaPublicKey> {
		let enclave_count: Option<u64> = self.enclave_count()?;
		let enclave_count = enclave_count
			.ok_or_else(|| CError::Other("[+] get enclave count error".to_string()))?;

		let enclave: Option<Enclave<AccountId, String>> = self.enclave(enclave_count)?;
		let enclave = enclave.ok_or_else(|| CError::Other("[+] get enclave error".to_string()))?;

		let shielding_key = enclave
			.shielding_key
			.ok_or_else(|| CError::Other("[+] get tee shielding pubkey error".to_string()))?;

		RsaPublicKey::new_with_rsa3072_pubkey(shielding_key)
			.map_err(|e| CError::Other(format!("Generate Rsa pubkey error: {:?}", e)))
	}

	fn get_vc_pubkey(&self) -> CResult<Ed25519Public> {
		let enclave_count: Option<u64> = self.enclave_count()?;
		let enclave_count = enclave_count
			.ok_or_else(|| CError::Other("[+] get enclave count error".to_string()))?;

		let enclave: Option<Enclave<AccountId, String>> = self.enclave(enclave_count)?;
		let enclave = enclave.ok_or_else(|| CError::Other("[+] get enclave error".to_string()))?;

		let vc_pubkey = enclave
			.vc_pubkey
			.ok_or_else(|| CError::Other("[+] get vc_pubkey error".to_string()))?;

		Ok(Ed25519Public(vec_to_u8_array::<32>(vc_pubkey)))
	}

	/// There're two methos to get the mrenclave
	/// 1. Online -> to use this method `get_shard` or
	/// 2. Offline -> to `litentry-parachain/tee-worker` run `make enclave`
	/// Both should be display exactly same value.
	fn get_shard(&self) -> CResult<MrEnclave> {
		let enclave_count: Option<u64> = self.enclave_count()?;
		let enclave_count = enclave_count
			.ok_or_else(|| CError::Other("[+] get enclave count error".to_string()))?;

		let enclave: Option<Enclave<AccountId, String>> = self.enclave(enclave_count)?;
		let enclave = enclave.ok_or_else(|| CError::Other("[+] get enclave error".to_string()))?;

		let shard = enclave.mr_enclave;
		let shard_in_hex = format!("0x{}", HexDisplay::from(&shard));

		println!("\n ✅ Get shard from parachain : {}", shard_in_hex);

		Ok(shard)
	}

	fn vc_registry(&self) -> CResult<Vec<VCContext>> {
		let vcregistry_encoded_keys =
			"b8806b89e4f9af656f87b35e6112ee1bda2e7b4c5a367debe17c26748ec6b3e6";
		let storage_key = hex::decode(vcregistry_encoded_keys).map_err(CError::FromHexError)?;
		let keys = self
			.parachain_client
			.api
			.get_keys(StorageKey(storage_key), None)
			.map_err(|_| CError::APIError)?;

		let mut contexts = vec![];
		if let Some(keys) = keys {
			for key in keys {
				let storage_key = hex::decode(&key[2..]).map_err(CError::FromHexError)?;
				let v: Option<VCContext> = self
					.parachain_client
					.api
					.get_storage_by_key(StorageKey(storage_key), None)
					.map_err(|_| CError::APIError)?;

				if let Some(context) = v {
					contexts.push(context);
				}
			}
		}

		Ok(contexts)
	}
}
