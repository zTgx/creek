// we use 256-bit AES-GCM as request enc/dec key
pub const REQUEST_AES_KEY_LEN: usize = 32;
pub use ring::aead::{MAX_TAG_LEN, NONCE_LEN};

pub type RequestAesKey = [u8; REQUEST_AES_KEY_LEN];
pub type RequestAesKeyNonce = [u8; NONCE_LEN];
