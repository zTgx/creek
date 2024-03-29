pub mod bitcoin_signature;
pub mod ethereum_signature;
pub mod validation_data;

use self::{bitcoin_signature::BitcoinSignature, ethereum_signature::EthereumSignature};
use super::identity::Identity;
use crate::{
	primitives::address::{Address20, Address32, Address33},
	utils::hex::hex_encode,
};
use bitcoin::{
	secp256k1,
	sign_message::{signed_msg_hash, MessageSignature},
};
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
use sp_core::{blake2_256, ecdsa, ed25519, keccak_256, sr25519, ByteArray, Decode, Encode};
use sp_runtime::traits::Verify;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum LitentryMultiSignature {
	/// An Ed25519 signature.
	#[codec(index = 0)]
	Ed25519(ed25519::Signature),
	/// An Sr25519 signature.
	#[codec(index = 1)]
	Sr25519(sr25519::Signature),
	/// An ECDSA/SECP256k1 signature.
	#[codec(index = 2)]
	Ecdsa(ecdsa::Signature),
	/// An ECDSA/keccak256 signature. An Ethereum signature. hash message with keccak256
	#[codec(index = 3)]
	Ethereum(EthereumSignature),
	/// Same as the above, but the payload bytes are hex-encoded and prepended with a readable
	/// prefix
	#[codec(index = 4)]
	EthereumPrettified(EthereumSignature),
	/// Bitcoin signed message
	#[codec(index = 5)]
	Bitcoin(BitcoinSignature),
}

impl From<ed25519::Signature> for LitentryMultiSignature {
	fn from(x: ed25519::Signature) -> Self {
		Self::Ed25519(x)
	}
}

impl From<sr25519::Signature> for LitentryMultiSignature {
	fn from(x: sr25519::Signature) -> Self {
		Self::Sr25519(x)
	}
}

impl From<ecdsa::Signature> for LitentryMultiSignature {
	fn from(x: ecdsa::Signature) -> Self {
		Self::Ecdsa(x)
	}
}

impl LitentryMultiSignature {
	pub fn verify(&self, msg: &[u8], signer: &Identity) -> bool {
		match signer {
			Identity::Substrate(address) =>
				self.verify_substrate(substrate_wrap(msg).as_slice(), address) ||
					self.verify_substrate(msg, address),
			Identity::Evm(address) => self.verify_evm(msg, address),
			Identity::Bitcoin(address) => self.verify_bitcoin(msg, address),
			_ => false,
		}
	}

	fn verify_substrate(&self, msg: &[u8], signer: &Address32) -> bool {
		match (self, signer) {
			(Self::Ed25519(ref sig), who) => match ed25519::Public::from_slice(who.as_ref()) {
				Ok(signer) => sig.verify(msg, &signer),
				Err(()) => false,
			},
			(Self::Sr25519(ref sig), who) => match sr25519::Public::from_slice(who.as_ref()) {
				Ok(signer) => sig.verify(msg, &signer),
				Err(()) => false,
			},
			(Self::Ecdsa(ref sig), who) => {
				let m = blake2_256(msg);
				match secp256k1_ecdsa_recover_compressed(sig.as_ref(), &m) {
					Ok(pubkey) =>
						&blake2_256(pubkey.as_ref()) == <dyn AsRef<[u8; 32]>>::as_ref(who),
					_ => false,
				}
			},
			_ => false,
		}
	}

	fn verify_evm(&self, msg: &[u8], signer: &Address20) -> bool {
		match self {
			Self::Ethereum(ref sig) => {
				let data = msg;
				return verify_evm_signature(evm_eip191_wrap(data).as_slice(), sig, signer) ||
					verify_evm_signature(data, sig, signer)
			},
			Self::EthereumPrettified(ref sig) => {
				let user_readable_message =
					"Litentry authorization token: ".to_string() + &hex_encode(msg);
				let data = user_readable_message.as_bytes();
				return verify_evm_signature(evm_eip191_wrap(data).as_slice(), sig, signer) ||
					verify_evm_signature(data, sig, signer)
			},
			_ => false,
		}
	}

	fn verify_bitcoin(&self, msg: &[u8], signer: &Address33) -> bool {
		match self {
			Self::Bitcoin(ref sig) => verify_bitcoin_signature(msg, sig, signer),
			_ => false,
		}
	}
}

pub fn verify_evm_signature(data: &[u8], sig: &EthereumSignature, who: &Address20) -> bool {
	let digest = keccak_256(data);
	return match recover_evm_address(&digest, sig.as_ref()) {
		Ok(recovered_evm_address) => recovered_evm_address == who.as_ref().as_slice(),
		Err(_e) => {
			println!("Could not verify evm signature msg: {:?}, signer {:?}", data, who);
			false
		},
	}
}

pub fn verify_bitcoin_signature(msg: &[u8], sig: &BitcoinSignature, who: &Address33) -> bool {
	if let Ok(msg_sig) = MessageSignature::from_slice(sig.as_ref()) {
		let msg_hash = signed_msg_hash(hex::encode(msg).as_str());
		let secp = secp256k1::Secp256k1::new();
		return match msg_sig.recover_pubkey(&secp, msg_hash) {
			Ok(recovered_pub_key) => &recovered_pub_key.inner.serialize() == who.as_ref(),
			Err(_) => {
				println!("Could not verify bitcoin signature msg: {:?}, signer {:?}", msg, who);
				false
			},
		}
	}

	false
}

// see https://github.com/litentry/litentry-parachain/issues/1137
fn substrate_wrap(msg: &[u8]) -> Vec<u8> {
	["<Bytes>".as_bytes(), msg, "</Bytes>".as_bytes()].concat()
}

// see https://github.com/litentry/litentry-parachain/issues/1970
fn evm_eip191_wrap(msg: &[u8]) -> Vec<u8> {
	["\x19Ethereum Signed Message:\n".as_bytes(), msg.len().to_string().as_bytes(), msg].concat()
}

pub fn secp256k1_ecdsa_recover(
	sig: &[u8; 65],
	msg: &[u8; 32],
) -> Result<[u8; 64], EcdsaVerifyError> {
	let rs = libsecp256k1::Signature::parse_standard_slice(&sig[0..64])
		.map_err(|_| EcdsaVerifyError::BadRS)?;
	let v = libsecp256k1::RecoveryId::parse(if sig[64] > 26 { sig[64] - 27 } else { sig[64] })
		.map_err(|_| EcdsaVerifyError::BadV)?;
	let pubkey = libsecp256k1::recover(&libsecp256k1::Message::parse(msg), &rs, &v)
		.map_err(|_| EcdsaVerifyError::BadSignature)?;
	let mut res = [0u8; 64];
	res.copy_from_slice(&pubkey.serialize()[1..65]);

	Ok(res)
}

pub fn secp256k1_ecdsa_recover_compressed(
	sig: &[u8; 65],
	msg: &[u8; 32],
) -> Result<[u8; 33], EcdsaVerifyError> {
	let rs = libsecp256k1::Signature::parse_standard_slice(&sig[0..64])
		.map_err(|_| EcdsaVerifyError::BadRS)?;
	let v = libsecp256k1::RecoveryId::parse(if sig[64] > 26 { sig[64] - 27 } else { sig[64] })
		.map_err(|_| EcdsaVerifyError::BadV)?;
	let pubkey = libsecp256k1::recover(&libsecp256k1::Message::parse(msg), &rs, &v)
		.map_err(|_| EcdsaVerifyError::BadSignature)?;
	Ok(pubkey.serialize_compressed())
}

pub fn recover_evm_address(msg: &[u8; 32], sig: &[u8; 65]) -> Result<[u8; 20], EcdsaVerifyError> {
	let pubkey = secp256k1_ecdsa_recover(sig, msg)?;
	let hashed_pk = keccak_256(&pubkey);

	let mut addr = [0u8; 20];
	addr[..20].copy_from_slice(&hashed_pk[12..32]);
	Ok(addr)
}

/// Error verifying ECDSA signature
#[derive(Encode, Decode)]
pub enum EcdsaVerifyError {
	/// Incorrect value of R or S
	BadRS,
	/// Incorrect value of V
	BadV,
	/// Invalid signature
	BadSignature,
}
