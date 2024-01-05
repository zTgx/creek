// Copyright 2020-2023 Trust Computing GmbH.
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

// This file includes the predefined rulesets and the corresponding parameters
// when requesting VCs.

mod vip3;
pub use vip3::*;

mod error;
pub use error::*;

mod vc;
pub use vc::*;

mod oneblock;
pub use oneblock::*;

mod contest;
pub use contest::*;

mod soraquiz;
pub use soraquiz::*;

mod bnb_domain;
pub use bnb_domain::*;

mod generic_discord_role;
pub use generic_discord_role::*;

mod evm_amount_holding;
pub use evm_amount_holding::*;

use codec::{Decode, Encode};
use std::{vec, vec::Vec};

use super::{
	network::{BoundedWeb3Network, Web3Network},
	AccountId,
};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableAmountHolding {
	pub name: String,
	pub chain: Web3Network,
	pub amount: String,
	pub date: String,
	pub token: Option<String>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableAmountToken {
	pub name: String,

	// Considering the uniformity of the structure, all relevant chain structures should be changed
	// to BoundedWeb3Network. However, this would be a significant modification for the previous
	// VC. Considering the tight timeline for this New Year compain, we will temporarily only
	// change this AchainableAmountToken' chain field to BoundedWeb3Network. Afterwards, it needs
	// to be modified to be consistent.
	pub chain: BoundedWeb3Network,

	pub amount: String,
	pub token: Option<String>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableAmount {
	pub name: String,
	pub chain: Web3Network,
	pub amount: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableAmounts {
	pub name: String,
	pub chain: Web3Network,
	pub amount1: String,
	pub amount2: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableBasic {
	pub name: String,
	pub chain: Web3Network,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableBetweenPercents {
	pub name: String,
	pub chain: Web3Network,
	pub greater_than_or_equal_to: String,
	pub less_than_or_equal_to: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableClassOfYear {
	pub name: String,
	pub chain: Web3Network,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableDateInterval {
	pub name: String,
	pub chain: Web3Network,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableDatePercent {
	pub name: String,
	pub chain: Web3Network,
	pub token: String,
	pub date: String,
	pub percent: String,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableDate {
	pub name: String,
	pub chain: Web3Network,
	pub date: String,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableToken {
	pub name: String,
	pub chain: Web3Network,
	pub token: String,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AchainableMirror {
	pub name: String,
	pub chain: Web3Network,
	pub post_quantity: Option<String>,
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum AchainableParams {
	#[codec(index = 0)]
	AmountHolding(AchainableAmountHolding),
	#[codec(index = 1)]
	AmountToken(AchainableAmountToken),
	#[codec(index = 2)]
	Amount(AchainableAmount),
	#[codec(index = 3)]
	Amounts(AchainableAmounts),
	#[codec(index = 4)]
	Basic(AchainableBasic),
	#[codec(index = 5)]
	BetweenPercents(AchainableBetweenPercents),
	#[codec(index = 6)]
	ClassOfYear(AchainableClassOfYear),
	#[codec(index = 7)]
	DateInterval(AchainableDateInterval),
	#[codec(index = 8)]
	DatePercent(AchainableDatePercent),
	#[codec(index = 9)]
	Date(AchainableDate),
	#[codec(index = 10)]
	Token(AchainableToken),
	#[codec(index = 11)]
	Mirror(AchainableMirror),
}

impl AchainableParams {
	pub fn name(&self) -> String {
		match self {
			AchainableParams::AmountHolding(p) => p.name.clone(),
			AchainableParams::AmountToken(p) => p.name.clone(),
			AchainableParams::Amount(p) => p.name.clone(),
			AchainableParams::Amounts(p) => p.name.clone(),
			AchainableParams::Basic(p) => p.name.clone(),
			AchainableParams::BetweenPercents(p) => p.name.clone(),
			AchainableParams::ClassOfYear(p) => p.name.clone(),
			AchainableParams::DateInterval(p) => p.name.clone(),
			AchainableParams::DatePercent(p) => p.name.clone(),
			AchainableParams::Date(p) => p.name.clone(),
			AchainableParams::Token(p) => p.name.clone(),
			AchainableParams::Mirror(p) => p.name.clone(),
		}
	}

	pub fn chains(&self) -> Vec<Web3Network> {
		match self {
			AchainableParams::AmountHolding(arg) => vec![arg.chain],
			AchainableParams::AmountToken(arg) => arg.chain.to_vec(),
			AchainableParams::Amount(arg) => vec![arg.chain],
			AchainableParams::Amounts(arg) => vec![arg.chain],
			AchainableParams::Basic(arg) => vec![arg.chain],
			AchainableParams::BetweenPercents(arg) => vec![arg.chain],
			AchainableParams::ClassOfYear(arg) => vec![arg.chain],
			AchainableParams::DateInterval(arg) => vec![arg.chain],
			AchainableParams::DatePercent(arg) => vec![arg.chain],
			AchainableParams::Date(arg) => vec![arg.chain],
			AchainableParams::Token(arg) => vec![arg.chain],
			AchainableParams::Mirror(arg) => vec![arg.chain],
		}
	}
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum Assertion {
	#[codec(index = 0)]
	A1,
	#[codec(index = 1)]
	A2(String),                                    // (guild_id)
	#[codec(index = 2)]
	A3(String, String, String),  // (guild_id, channel_id, role_id)
	#[codec(index = 3)]
	A4(String),                                    // (minimum_amount)
	#[codec(index = 4)]
	A6,
	#[codec(index = 5)]
	A7(String),                                    // (minimum_amount)
	#[codec(index = 6)]
	A8(BoundedWeb3Network),                                 // litentry, litmus, polkadot, kusama, khala, ethereum
	#[codec(index = 7)]
	A10(String),                                   // (minimum_amount)
	#[codec(index = 8)]
	A11(String),                                   // (minimum_amount)

	// ----- begin polkadot decoded 2023 -----
	#[codec(index = 9)]
	A13(AccountId),                                         // (participant_account), can only be requested by delegatee
	#[codec(index = 10)]
	A14,
	// for Holder assertions we'll reuse A4/A7
	// ----- end polkadot decoded 2023 -----
	#[codec(index = 11)]
	Achainable(AchainableParams),

	// For EVM Version Early Bird
	#[codec(index = 12)]
	A20,

	// For Oneblock
	#[codec(index = 13)]
	Oneblock(OneBlockCourseType),

	// GenericDiscordRole
	#[codec(index = 14)]
	GenericDiscordRole(GenericDiscordRoleType),  // (generic_discord_role_type)

	// ----- begin SPACEID -----
	#[codec(index = 16)]
	BnbDomainHolding,

	#[codec(index = 17)]
	BnbDigitDomainClub(BnbDigitDomainType),
	// ----- end SPACEID -----

	#[codec(index = 18)]
	VIP3MembershipCard(VIP3MembershipCardLevel),

	#[codec(index = 19)]
	WeirdoGhostGangHolder,

	#[codec(index = 20)]
	LITStaking,

	#[codec(index = 21)]
	EVMAmountHolding(EVMTokenType),  // (evm_token_type)

	#[codec(index = 22)]
	BRC20AmountHolder,

	#[codec(index = 23)]
	CryptoSummary,
}
