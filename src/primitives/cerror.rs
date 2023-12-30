use crate::utils::hex::Error;

#[derive(Debug)]
pub enum CError {
	APIError,

	CodecError(codec::Error),
	HexError(Error),
	DecodeJsonError(serde_json::Error),
	RSAError(rsa::errors::Error),
	
	Other(String),
}
