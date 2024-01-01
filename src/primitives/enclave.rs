use codec::{Decode, Encode};
use scale_info::TypeInfo;

use super::MrEnclave;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Enclave<PubKey, Url> {
	pub pubkey: PubKey, // FIXME: this is redundant information
	pub mr_enclave: MrEnclave,
	// Todo: make timestamp: Moment
	pub timestamp: u64,                 // unix epoch in milliseconds
	pub url: Url,                       // utf8 encoded url
	pub shielding_key: Option<Vec<u8>>, // JSON serialised enclave shielding key
	pub vc_pubkey: Option<Vec<u8>>,
	pub sgx_mode: SgxBuildMode,
	pub sgx_metadata: SgxEnclaveMetadata,
}

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Default, sp_core::RuntimeDebug)]
pub struct SgxEnclaveMetadata {
	pub quote: Vec<u8>,
	pub quote_sig: Vec<u8>,
	pub quote_cert: Vec<u8>,
}

#[derive(Encode, Decode, Copy, Clone, Default, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
	Debug,

	#[default]
	Production,
}
