use crate::direct_call::primitives::{Index, UserShieldingKeyType};
use codec::Decode;

#[derive(Debug)]
pub enum DIError {
    DecodeNonceError,
    DecodeUserShieldingKeyError,
}

pub fn decode_nonce(encode_nonce_in_hex: &str) -> Result<Index, DIError> {
    if let Ok(decode_hex) = hex::decode(encode_nonce_in_hex) {
        if let Ok(Some(nonce)) = Option::<Vec<u8>>::decode(&mut decode_hex.as_slice()) {
            if let Ok(nonce) = Index::decode(&mut nonce.as_slice()) {
                println!("nonce: {:?}", nonce);

                Ok(nonce)
            } else {
                Err(DIError::DecodeNonceError)
            }
        } else {
            Err(DIError::DecodeNonceError)
        }
    } else {
        Err(DIError::DecodeNonceError)
    }
}

pub fn decode_user_shielding_key(encode_key_in_hex: &str) -> Result<String, DIError> {
    match hex::decode(encode_key_in_hex) {
        Ok(decoded) => {
            if let Ok(Some(x)) = Option::<Vec<u8>>::decode(&mut decoded.as_slice()) {
                if let Ok(decoded) = UserShieldingKeyType::decode(&mut x.as_slice()) {
                    let user_shielding_key = hex::encode(decoded);
                    println!("use shielding key: {}", user_shielding_key);

                    Ok(user_shielding_key)
                } else {
                    // decode error
                    Err(DIError::DecodeUserShieldingKeyError)
                }
            } else {
                // decode error
                Err(DIError::DecodeUserShieldingKeyError)
            }
        }
        Err(_e) => Err(DIError::DecodeUserShieldingKeyError),
    }
}
