// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..
#![allow(non_camel_case_types)]

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub const SGX_PLATFORM_INFO_SIZE: size_t = 101;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum sgx_status_t {
    SGX_SUCCESS = 0x0000_0000,

    SGX_ERROR_UNEXPECTED = 0x0000_0001, /* Unexpected error */
    SGX_ERROR_INVALID_PARAMETER = 0x0000_0002, /* The parameter is incorrect */
    SGX_ERROR_OUT_OF_MEMORY = 0x0000_0003, /* Not enough memory is available to complete this operation */
    SGX_ERROR_ENCLAVE_LOST = 0x0000_0004, /* Enclave lost after power transition or used in child process created by linux:fork() */
    SGX_ERROR_INVALID_STATE = 0x0000_0005, /* SGX API is invoked in incorrect order or state */
    SGX_ERROR_FEATURE_NOT_SUPPORTED = 0x0000_0008, /* Feature is not supported on this platform */
    SGX_PTHREAD_EXIT = 0x0000_0009,       /* Enclave is exited with pthread_exit() */
    SGX_ERROR_MEMORY_MAP_FAILURE = 0x0000_000A, /* Failed to reserve memory for the enclave */

    SGX_ERROR_INVALID_FUNCTION = 0x0000_1001, /* The ecall/ocall index is invalid */
    SGX_ERROR_OUT_OF_TCS = 0x0000_1003,       /* The enclave is out of TCS */
    SGX_ERROR_ENCLAVE_CRASHED = 0x0000_1006,  /* The enclave is crashed */
    SGX_ERROR_ECALL_NOT_ALLOWED = 0x0000_1007, /* The ECALL is not allowed at this time, e.g. ecall is blocked by the dynamic entry table, or nested ecall is not allowed during initialization */
    SGX_ERROR_OCALL_NOT_ALLOWED = 0x0000_1008, /* The OCALL is not allowed at this time, e.g. ocall is not allowed during exception handling */
    SGX_ERROR_STACK_OVERRUN = 0x0000_1009,     /* The enclave is running out of stack */

    SGX_ERROR_UNDEFINED_SYMBOL = 0x0000_2000, /* The enclave image has undefined symbol. */
    SGX_ERROR_INVALID_ENCLAVE = 0x0000_2001,  /* The enclave image is not correct. */
    SGX_ERROR_INVALID_ENCLAVE_ID = 0x0000_2002, /* The enclave id is invalid */
    SGX_ERROR_INVALID_SIGNATURE = 0x0000_2003, /* The signature is invalid */
    SGX_ERROR_NDEBUG_ENCLAVE = 0x0000_2004, /* The enclave is signed as product enclave, and can not be created as debuggable enclave. */
    SGX_ERROR_OUT_OF_EPC = 0x0000_2005,     /* Not enough EPC is available to load the enclave */
    SGX_ERROR_NO_DEVICE = 0x0000_2006,      /* Can't open SGX device */
    SGX_ERROR_MEMORY_MAP_CONFLICT = 0x0000_2007, /* Page mapping failed in driver */
    SGX_ERROR_INVALID_METADATA = 0x0000_2009, /* The metadata is incorrect. */
    SGX_ERROR_DEVICE_BUSY = 0x0000_200C,    /* Device is busy, mostly EINIT failed. */
    SGX_ERROR_INVALID_VERSION = 0x0000_200D, /* Metadata version is inconsistent between uRTS and sgx_sign or uRTS is incompatible with current platform. */
    SGX_ERROR_MODE_INCOMPATIBLE = 0x0000_200E, /* The target enclave 32/64 bit mode or sim/hw mode is incompatible with the mode of current uRTS. */
    SGX_ERROR_ENCLAVE_FILE_ACCESS = 0x0000_200F, /* Can't open enclave file. */
    SGX_ERROR_INVALID_MISC = 0x0000_2010,      /* The MiscSelct/MiscMask settings are not correct.*/
    SGX_ERROR_INVALID_LAUNCH_TOKEN = 0x0000_2011, /* The launch token is not correct.*/

    SGX_ERROR_MAC_MISMATCH = 0x0000_3001, /* Indicates verification error for reports, sealed datas, etc */
    SGX_ERROR_INVALID_ATTRIBUTE = 0x0000_3002, /* The enclave is not authorized, e.g., requesting invalid attribute or launch key access on legacy SGX platform without FLC.  */
    SGX_ERROR_INVALID_CPUSVN = 0x0000_3003,    /* The cpu svn is beyond platform's cpu svn value */
    SGX_ERROR_INVALID_ISVSVN = 0x0000_3004, /* The isv svn is greater than the enclave's isv svn */
    SGX_ERROR_INVALID_KEYNAME = 0x0000_3005, /* The key name is an unsupported value */

    SGX_ERROR_SERVICE_UNAVAILABLE = 0x0000_4001, /* Indicates aesm didn't respond or the requested service is not supported */
    SGX_ERROR_SERVICE_TIMEOUT = 0x0000_4002,     /* The request to aesm timed out */
    SGX_ERROR_AE_INVALID_EPIDBLOB = 0x0000_4003, /* Indicates epid blob verification error */
    SGX_ERROR_SERVICE_INVALID_PRIVILEGE = 0x0000_4004, /*  Enclave not authorized to run, .e.g. provisioning enclave hosted in an app without access rights to /dev/sgx_provision */
    SGX_ERROR_EPID_MEMBER_REVOKED = 0x0000_4005,       /* The EPID group membership is revoked. */
    SGX_ERROR_UPDATE_NEEDED = 0x0000_4006,             /* SGX needs to be updated */
    SGX_ERROR_NETWORK_FAILURE = 0x0000_4007, /* Network connecting or proxy setting issue is encountered */
    SGX_ERROR_AE_SESSION_INVALID = 0x0000_4008, /* Session is invalid or ended by server */
    SGX_ERROR_BUSY = 0x0000_400A,            /* The requested service is temporarily not availabe */
    SGX_ERROR_MC_NOT_FOUND = 0x0000_400C, /* The Monotonic Counter doesn't exist or has been invalided */
    SGX_ERROR_MC_NO_ACCESS_RIGHT = 0x0000_400D, /* Caller doesn't have the access right to specified VMC */
    SGX_ERROR_MC_USED_UP = 0x0000_400E,         /* Monotonic counters are used out */
    SGX_ERROR_MC_OVER_QUOTA = 0x0000_400F,      /* Monotonic counters exceeds quota limitation */
    SGX_ERROR_KDF_MISMATCH = 0x0000_4011, /* Key derivation function doesn't match during key exchange */
    SGX_ERROR_UNRECOGNIZED_PLATFORM = 0x0000_4012, /* EPID Provisioning failed due to platform not recognized by backend server*/
    SGX_ERROR_UNSUPPORTED_CONFIG = 0x0000_4013, /* The config for trigging EPID Provisiong or PSE Provisiong&LTP is invalid*/

    SGX_ERROR_NO_PRIVILEGE = 0x0000_5002, /* Not enough privilege to perform the operation */

    /* SGX Protected Code Loader Error codes*/
    SGX_ERROR_PCL_ENCRYPTED = 0x0000_6001, /* trying to encrypt an already encrypted enclave */
    SGX_ERROR_PCL_NOT_ENCRYPTED = 0x0000_6002, /* trying to load a plain enclave using sgx_create_encrypted_enclave */
    SGX_ERROR_PCL_MAC_MISMATCH = 0x0000_6003, /* section mac result does not match build time mac */
    SGX_ERROR_PCL_SHA_MISMATCH = 0x0000_6004, /* Unsealed key MAC does not match MAC of key hardcoded in enclave binary */
    SGX_ERROR_PCL_GUID_MISMATCH = 0x0000_6005, /* GUID in sealed blob does not match GUID hardcoded in enclave binary */

    /* SGX errors are only used in the file API when there is no appropriate EXXX (EINVAL, EIO etc.) error code */
    SGX_ERROR_FILE_BAD_STATUS = 0x0000_7001, /* The file is in bad status, run sgx_clearerr to try and fix it */
    SGX_ERROR_FILE_NO_KEY_ID = 0x0000_7002, /* The Key ID field is all zeros, can't re-generate the encryption key */
    SGX_ERROR_FILE_NAME_MISMATCH = 0x0000_7003, /* The current file name is different then the original file name (not allowed, substitution attack) */
    SGX_ERROR_FILE_NOT_SGX_FILE = 0x0000_7004,  /* The file is not an SGX file */
    SGX_ERROR_FILE_CANT_OPEN_RECOVERY_FILE = 0x0000_7005, /* A recovery file can't be opened, so flush operation can't continue (only used when no EXXX is returned)  */
    SGX_ERROR_FILE_CANT_WRITE_RECOVERY_FILE = 0x0000_7006, /* A recovery file can't be written, so flush operation can't continue (only used when no EXXX is returned)  */
    SGX_ERROR_FILE_RECOVERY_NEEDED = 0x0000_7007, /* When openeing the file, recovery is needed, but the recovery process failed */
    SGX_ERROR_FILE_FLUSH_FAILED = 0x0000_7008, /* fflush operation (to disk) failed (only used when no EXXX is returned) */
    SGX_ERROR_FILE_CLOSE_FAILED = 0x0000_7009, /* fclose operation (to disk) failed (only used when no EXXX is returned) */

    SGX_ERROR_UNSUPPORTED_ATT_KEY_ID = 0x0000_8001, /* platform quoting infrastructure does not support the key.*/
    SGX_ERROR_ATT_KEY_CERTIFICATION_FAILURE = 0x0000_8002, /* Failed to generate and certify the attestation key.*/
    SGX_ERROR_ATT_KEY_UNINITIALIZED = 0x0000_8003, /* The platform quoting infrastructure does not have the attestation key available to generate quote.*/
    SGX_ERROR_INVALID_ATT_KEY_CERT_DATA = 0x0000_8004, /* TThe data returned by the platform library's sgx_get_quote_config() is invalid.*/
    SGX_ERROR_PLATFORM_CERT_UNAVAILABLE = 0x0000_8005, /* The PCK Cert for the platform is not available.*/

    SGX_INTERNAL_ERROR_ENCLAVE_CREATE_INTERRUPTED = 0x0000_F001, /* The ioctl for enclave_create unexpectedly failed with EINTR. */

    SGX_ERROR_WASM_BUFFER_TOO_SHORT = 0x0F00_F001, /* sgxwasm output buffer not long enough */
    SGX_ERROR_WASM_INTERPRETER_ERROR = 0x0F00_F002, /* sgxwasm interpreter error */
    SGX_ERROR_WASM_LOAD_MODULE_ERROR = 0x0F00_F003, /* sgxwasm loadmodule error */
    SGX_ERROR_WASM_TRY_LOAD_ERROR = 0x0F00_F004,   /* sgxwasm tryload error */
    SGX_ERROR_WASM_REGISTER_ERROR = 0x0F00_F005,   /* sgxwasm register error */
    SGX_ERROR_FAAS_BUFFER_TOO_SHORT = 0x0F00_E001, /* faas output buffer not long enough */
    SGX_ERROR_FAAS_INTERNAL_ERROR = 0x0F00_E002,   /* faas exec internal error */
}

pub type SgxResult<T> = std::result::Result<T, sgx_status_t>;

pub type sgx_epid_group_id_t = [uint8_t; 4];
pub type sgx_key_128bit_t = [uint8_t; 16];
pub type sgx_isv_svn_t = uint16_t;
pub type sgx_config_svn_t = uint16_t;
pub type sgx_config_id_t = [uint8_t; SGX_CONFIGID_SIZE];

pub struct sgx_basename_t {
    pub name: [uint8_t; 32],
}

pub struct sgx_cpu_svn_t {
    pub svn: [uint8_t; SGX_CPUSVN_SIZE],
}
pub type sgx_misc_select_t = uint32_t;
pub const SGX_TARGET_INFO_RESERVED1_BYTES: size_t = 2;
pub const SGX_TARGET_INFO_RESERVED2_BYTES: size_t = 8;
pub const SGX_TARGET_INFO_RESERVED3_BYTES: size_t = 384;

pub const SGX_REPORT_BODY_RESERVED1_BYTES: size_t = 12;
pub const SGX_REPORT_BODY_RESERVED2_BYTES: size_t = 32;
pub const SGX_REPORT_BODY_RESERVED3_BYTES: size_t = 32;
pub const SGX_REPORT_BODY_RESERVED4_BYTES: size_t = 42;

pub const SGX_REPORT_DATA_SIZE: size_t = 64;

pub const SGX_ISVEXT_PROD_ID_SIZE: size_t = 16;
pub const SGX_ISV_FAMILY_ID_SIZE: size_t = 16;

pub type sgx_prod_id_t = uint16_t;
pub type sgx_isvext_prod_id_t = [uint8_t; SGX_ISVEXT_PROD_ID_SIZE];

pub const SGX_KEYID_SIZE: size_t = 32;
pub const SGX_CPUSVN_SIZE: size_t = 16;
pub const SGX_CONFIGID_SIZE: size_t = 64;
pub const SGX_KEY_REQUEST_RESERVED2_BYTES: size_t = 434;

pub struct sgx_attributes_t {
    pub flags: uint64_t,
    pub xfrm: uint64_t,
}

pub struct sgx_misc_attribute_t {
    pub secs_attr: sgx_attributes_t,
    pub misc_select: sgx_misc_select_t,
}

pub struct sgx_measurement_t {
    pub m: [uint8_t; SGX_HASH_SIZE],
}

pub const SGX_HASH_SIZE: size_t = 32;
pub const SGX_MAC_SIZE: size_t = 16;

pub type sgx_isvfamily_id_t = [uint8_t; SGX_ISV_FAMILY_ID_SIZE];

pub struct sgx_report_data_t {
    pub d: [uint8_t; SGX_REPORT_DATA_SIZE],
}

pub struct sgx_report_body_t {
    pub cpu_svn: sgx_cpu_svn_t,
    pub misc_select: sgx_misc_select_t,
    pub reserved1: [uint8_t; SGX_REPORT_BODY_RESERVED1_BYTES],
    pub isv_ext_prod_id: sgx_isvext_prod_id_t,
    pub attributes: sgx_attributes_t,
    pub mr_enclave: sgx_measurement_t,
    pub reserved2: [uint8_t; SGX_REPORT_BODY_RESERVED2_BYTES],
    pub mr_signer: sgx_measurement_t,
    pub reserved3: [uint8_t; SGX_REPORT_BODY_RESERVED3_BYTES],
    pub config_id: sgx_config_id_t,
    pub isv_prod_id: sgx_prod_id_t,
    pub isv_svn: sgx_isv_svn_t,
    pub config_svn: sgx_config_svn_t,
    pub reserved4: [uint8_t; SGX_REPORT_BODY_RESERVED4_BYTES],
    pub isv_family_id: sgx_isvfamily_id_t,
    pub report_data: sgx_report_data_t,
}

pub struct sgx_quote_t {
    pub version: uint16_t,                  /* 0   */
    pub sign_type: uint16_t,                /* 2   */
    pub epid_group_id: sgx_epid_group_id_t, /* 4   */
    pub qe_svn: sgx_isv_svn_t,              /* 8   */
    pub pce_svn: sgx_isv_svn_t,             /* 10  */
    pub xeid: uint32_t,                     /* 12  */
    pub basename: sgx_basename_t,           /* 16  */
    pub report_body: sgx_report_body_t,     /* 48  */
    pub signature_len: uint32_t,            /* 432 */
    pub signature: [uint8_t; 0],            /* 436 */
}
