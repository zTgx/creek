use super::LitentryMultiSignature;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TwitterValidationData {
	pub tweet_id: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DiscordValidationData {
	pub channel_id: String,
	pub message_id: String,
	pub guild_id: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Web3CommonValidationData {
	pub message: Vec<u8>, // or String if under std
	pub signature: LitentryMultiSignature,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Web2ValidationData {
	#[codec(index = 0)]
	Twitter(TwitterValidationData),
	#[codec(index = 1)]
	Discord(DiscordValidationData),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[allow(non_camel_case_types)]
pub enum Web3ValidationData {
	#[codec(index = 0)]
	Substrate(Web3CommonValidationData),
	#[codec(index = 1)]
	Evm(Web3CommonValidationData),
	#[codec(index = 2)]
	Bitcoin(Web3CommonValidationData),
}

impl Web3ValidationData {
	pub fn message(&self) -> &Vec<u8> {
		match self {
			Self::Substrate(data) => &data.message,
			Self::Evm(data) => &data.message,
			Self::Bitcoin(data) => &data.message,
		}
	}

	pub fn signature(&self) -> &LitentryMultiSignature {
		match self {
			Self::Substrate(data) => &data.signature,
			Self::Evm(data) => &data.signature,
			Self::Bitcoin(data) => &data.signature,
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ValidationData {
	#[codec(index = 0)]
	Web2(Web2ValidationData),
	#[codec(index = 1)]
	Web3(Web3ValidationData),
}
