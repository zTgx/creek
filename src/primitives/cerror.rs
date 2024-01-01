use crate::utils::hex::Error;

#[derive(Debug)]
pub enum CError {
	APIError,
	CodecError(codec::Error),
	HexError(Error),
	FromHexError(hex::FromHexError),
	DecodeJsonError(serde_json::Error),
	RSAError(rsa::errors::Error),
	RecvError(std::sync::mpsc::RecvError),
	Other(String),
}
