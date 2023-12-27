pub mod attestation;
pub mod sgx_types;

use crate::{
	primitives::{enclave::Enclave, AccountId},
	ra::sgx_types::{SgxResult, SGX_PLATFORM_INFO_SIZE},
};

extern "C" {
	fn lib_c_sgx_report_att_status(platform_info: *const u8);
	fn lib_c_sgx_check_update_status(platform_info: *const u8);
	fn lib_c_sgx_verify_report(report: *const u8);
}

pub trait SafeSgxApi {
	fn safe_sgx_report_att_status(platform_info: [u8; SGX_PLATFORM_INFO_SIZE]);
	fn safe_sgx_check_update_status(platform_info: [u8; SGX_PLATFORM_INFO_SIZE]);
	fn safe_sgx_verify_report(report: Vec<u8>);
}

pub struct SafeSgx;

pub trait RaAttestationExecutor {
	fn execute(&self) -> SgxResult<()>;
}

pub struct RaAttestation {
	pub enclave_registry: Enclave<AccountId, String>,
}
