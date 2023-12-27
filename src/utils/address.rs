use crate::primitives::address::{Address20, Address32};
use aes_gcm::aead::OsRng;
use rand::{Rng, RngCore};
use sp_core::{
	crypto::{PublicError, Ss58Codec},
	sr25519, Pair,
};

const ACCOUNT_SEED_CHARSET: &[u8] =
	b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// How public key transimit>>>
/// [u8; 32] -> Pair::Public
/// [u8; 32] -> hex::encode -> Public key
/// [u8; 32] -> Pair::Public -> Ss58Codec -> ss58 address
/// Online check: https://ss58.org/
pub fn pubkey_to_address32(pubkey: &str) -> Result<Address32, String> {
	if !pubkey.starts_with("0x") && pubkey.len() != 62 {
		return Err("Incorrect hex account format!".into())
	}

	let decoded_account =
		hex::decode(&pubkey[2..]).map_err(|e| format!("decode error: {:?}", e))?;
	let bytes = vec_to_u8_array::<32>(decoded_account);
	Ok(Address32::from(bytes))
}

pub fn pubkey_to_address20(pubkey: &str) -> Result<Address20, String> {
	if !pubkey.starts_with("0x") && pubkey.len() != 42 {
		return Err("Incorrect hex account format!".into())
	}

	let decoded_account =
		hex::decode(&pubkey[2..]).map_err(|e| format!("decode error: {:?}", e))?;
	let bytes = vec_to_u8_array::<20>(decoded_account);

	Ok(Address20::from(bytes))
}

/// sr25519 pubkey -> ss58 address
pub fn sr25519_public_to_ss58(pubkey: &sr25519::Public) -> String {
	pubkey.to_ss58check()
}

pub fn sr25519_public_from_ss58(ss58_address: &str) -> Result<sr25519::Public, PublicError> {
	sr25519::Public::from_ss58check(ss58_address)
}

pub fn public_to_address32(public: &sr25519::Public) -> Address32 {
	let bytes = public.as_array_ref();
	Address32::from(*bytes)
}

pub fn vec_to_u8_array<const LEN: usize>(input: Vec<u8>) -> [u8; LEN] {
	assert_eq!(input.len(), LEN);

	let mut bytes = [0u8; LEN];
	bytes[..LEN].clone_from_slice(&input);

	bytes
}

/// Interprets the string `s` in order to generate a key Pair. Returns both the pair and an
/// optional seed, in the case that the pair can be expressed as a direct derivation from a seed
/// (some cases, such as Sr25519 derivations with path components, cannot).
///
/// This takes a helper function to do the key generation from a phrase, password and
/// junction iterator.
///
/// - If `s` is a possibly `0x` prefixed 64-digit hex string, then it will be interpreted
/// directly as a `MiniSecretKey` (aka "seed" in `subkey`).
/// - If `s` is a valid BIP-39 key phrase of 12, 15, 18, 21 or 24 words, then the key will
/// be derived from it. In this case:
///   - the phrase may be followed by one or more items delimited by `/` characters.
///   - the path may be followed by `///`, in which case everything after the `///` is treated
/// as a password.
/// - If `s` begins with a `/` character it is prefixed with the Substrate public `DEV_PHRASE` and
/// interpreted as above.
///
/// In this case they are interpreted as HDKD junctions; purely numeric items are interpreted as
/// integers, non-numeric items as strings. Junctions prefixed with `/` are interpreted as soft
/// junctions, and with `//` as hard junctions.
///
/// There is no correspondence mapping between SURI strings and the keys they represent.
/// Two different non-identical strings can actually lead to the same secret being derived.
/// Notably, integer junction indices may be legally prefixed with arbitrary number of zeros.
/// Similarly an empty password (ending the SURI with `///`) is perfectly valid and will
/// generally be equivalent to no password at all.
///
/// `None` is returned if no matches are found.
/// See `from_string_with_seed` for more details in substrate.
pub fn create_a_random_account_seed(len: usize) -> String {
	let mut rng = rand::thread_rng();

	let account_str: String = (0..len)
		.map(|_| {
			let idx = rng.gen_range(0..ACCOUNT_SEED_CHARSET.len());
			ACCOUNT_SEED_CHARSET[idx] as char
		})
		.collect();

	account_str
}

pub fn create_n_random_sr25519_address(num: usize) -> Result<Vec<sr25519::Pair>, String> {
	let mut addresses = vec![];
	let mut index = 0;
	while index < num {
		let mut account_seed = create_a_random_account_seed(3);
		account_seed.insert_str(0, "//");
		let account_pair = sr25519::Pair::from_string(&account_seed, None)
			.map_err(|e| format!("Pair from error: {:?}", e))?;
		// let address: Address32 = account_pair.public().0.into();

		addresses.push(account_pair);

		index += 1;
	}

	Ok(addresses)
}

pub fn create_a_random_u32() -> u32 {
	let mut os_rng = OsRng;
	os_rng.next_u32()
}
