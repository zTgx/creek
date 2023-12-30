use crate::utils::hex::Error;

#[derive(Debug)]
pub enum CError {
	APIError,

	CodecError(codec::Error),
	HexError(Error),

	Other(String),
}
