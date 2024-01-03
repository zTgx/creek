// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use super::{aes::AesOutput, assertion::Assertion, AccountId};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::H256;

/// Ed25519 Signature 2018, W3C, 23 July 2021, https://w3c-ccg.github.io/lds-ed25519-2018
/// May be registered in Linked Data Cryptographic Suite Registry, W3C, 29 December 2020
/// https://w3c-ccg.github.io/ld-cryptosuite-registry
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum ProofType {
	Ed25519Signature2020,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum CredentialType {
	VerifiableCredential,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
	/// ID of the data provider
	pub data_provider_id: u32,
	/// Endpoint of the data provider
	pub data_provider: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
	/// ID of the TEE Worker
	pub id: String,
	pub name: String,
	pub mrenclave: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSubject {
	/// Identifier for the only entity that the credential was issued
	pub id: String,
	pub description: String,
	#[serde(rename = "type")]
	pub types: String,
	/// (Optional) Some externally provided identifiers
	pub tag: Vec<String>,
	/// (Optional) Data source definitions for trusted data providers
	#[serde(skip_deserializing)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<Vec<DataSource>>,
	/// Several sets of assertions.
	/// Each assertion contains multiple steps to describe how to fetch data and calculate the
	/// value
	// #[serde(skip_deserializing)]
	pub assertions: Vec<AssertionLogic>,
	/// Results of each set of assertions
	pub values: Vec<bool>,
	/// The extrinsic on Parentchain for credential verification purpose
	pub endpoint: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Op {
	#[serde(rename = ">")]
	GreaterThan,
	#[serde(rename = "<")]
	LessThan,
	#[serde(rename = ">=")]
	GreaterEq,
	#[serde(rename = "<=")]
	LessEq,
	#[serde(rename = "==")]
	Equal,
	#[serde(rename = "!=")]
	NotEq,
}

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Eq, TypeInfo, Debug, Clone)]
#[serde(untagged)]
pub enum AssertionLogic {
	Item {
		src: String,
		op: Op,
		dst: String,
	},
	And {
		#[serde(rename = "and")]
		items: Vec<Box<AssertionLogic>>,
	},
	Or {
		#[serde(rename = "or")]
		items: Vec<Box<AssertionLogic>>,
	},
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSchema {
	/// Schema ID that is maintained by Parentchain VCMP
	pub id: String,
	/// The schema type, generally it is
	#[serde(rename = "type")]
	pub types: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	/// The block number when the signature was created
	pub created_timestamp: u64,
	/// The cryptographic signature suite that used to generate signature
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	/// Purpose of this proof, generally it is expected as a fixed value, such as 'assertionMethod'
	pub proof_purpose: String,
	/// The digital signature value(signature of hash)
	pub proof_value: String,
	/// The public key from Issuer
	pub verification_method: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
	/// Contexts defines the structure and data types of the credential
	#[serde(rename = "@context")]
	pub context: Vec<String>,
	/// The specific UUID of the credential, it is used for onchain verification
	pub id: String,
	/// Uniquely identifier of the type of the credential
	#[serde(rename = "type")]
	pub types: Vec<CredentialType>,
	/// Assertions claimed about the subjects of the credential
	pub credential_subject: CredentialSubject,
	/// The TEE enclave who issued the credential
	pub issuer: Issuer,
	pub issuance_timestamp: u64,
	/// (Optional)
	#[serde(skip_serializing_if = "Option::is_none")]
	// #[serde(skip_serializing)]
	pub expiration_timestamp: Option<u64>,

	/// Digital proof with the signature of Issuer
	#[serde(skip_serializing_if = "Option::is_none")]
	// #[serde(skip_serializing)]
	pub proof: Option<Proof>,

	#[serde(skip_deserializing)]
	#[serde(skip_serializing)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub credential_schema: Option<CredentialSchema>,
}

#[derive(
	Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen,
)]
pub enum Status {
	Active,
	Disabled,
	// Revoked, // commented out for now, we can delete the VC entry when revoked
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct VCContext {
	// To be discussed: shall we make it public?
	// pros: easier for the user to disable/revoke VCs, we'll need the AccountId to verify
	//       the owner of VC. An alternative is to store such information within TEE.
	// cons: this information is then public, everyone knows e.g. ALICE owns VC ID 1234 + 4321
	// It's not bad though as it helps to verify the ownership of VC
	pub subject: AccountId,
	// requested assertion type
	pub assertion: Assertion,
	// hash of the VC, computed via blake2_256
	pub hash: H256,
	// status of the VC
	pub status: Status,
}

impl VCContext {
	pub fn new(subject: AccountId, assertion: Assertion, hash: H256) -> Self {
		Self { subject, assertion, hash, status: Status::Active }
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum ErrorDetail {
	// error when importing the parentchain blocks and executing indirect calls
	ImportError,
	// generic error when executing STF, the `ErrorString` should indicate the actual reasons
	StfError(String),
	// error when sending stf request to the receiver
	SendStfRequestFailed,
	ChallengeCodeNotFound,
	UserShieldingKeyNotFound,
	// generic parse error, can be caused by UTF8/JSON serde..
	ParseError,
	// errors when verifying identities
	DecodeHexPayloadFailed(String),
	HttpRequestFailed(String),
	InvalidIdentity,
	WrongWeb2Handle,
	UnexpectedMessage,
	WrongSignatureType,
	VerifySubstrateSignatureFailed,
	VerifyEvmSignatureFailed,
	RecoverEvmAddressFailed,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct RequestVCResult {
	pub vc_index: H256,
	pub vc_hash: H256,
	pub vc_payload: AesOutput,
}
