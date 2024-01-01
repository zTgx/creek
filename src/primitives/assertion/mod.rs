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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use std::{str, vec, vec::Vec};

use super::{
	network::{BoundedWeb3Network, Web3Network},
	AccountId,
};

pub type ParameterString = BoundedVec<u8, ConstU32<64>>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmountHolding {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount: ParameterString,
	pub date: ParameterString,
	pub token: Option<ParameterString>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmountToken {
	pub name: ParameterString,

	// Considering the uniformity of the structure, all relevant chain structures should be changed
	// to BoundedWeb3Network. However, this would be a significant modification for the previous
	// VC. Considering the tight timeline for this New Year compain, we will temporarily only
	// change this AchainableAmountToken' chain field to BoundedWeb3Network. Afterwards, it needs
	// to be modified to be consistent.
	pub chain: BoundedWeb3Network,

	pub amount: ParameterString,
	pub token: Option<ParameterString>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmount {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmounts {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub amount1: ParameterString,
	pub amount2: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableBasic {
	pub name: ParameterString,
	pub chain: Web3Network,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableBetweenPercents {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub greater_than_or_equal_to: ParameterString,
	pub less_than_or_equal_to: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableClassOfYear {
	pub name: ParameterString,
	pub chain: Web3Network,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDateInterval {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub start_date: ParameterString,
	pub end_date: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDatePercent {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub token: ParameterString,
	pub date: ParameterString,
	pub percent: ParameterString,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDate {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub date: ParameterString,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableToken {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub token: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableMirror {
	pub name: ParameterString,
	pub chain: Web3Network,
	pub post_quantity: Option<ParameterString>,
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
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
	pub fn name(&self) -> ParameterString {
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
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum Assertion {
	#[codec(index = 0)]
	A1,
	#[codec(index = 1)]
	A2(ParameterString),                                    // (guild_id)
	#[codec(index = 2)]
	A3(ParameterString, ParameterString, ParameterString),  // (guild_id, channel_id, role_id)
	#[codec(index = 3)]
	A4(ParameterString),                                    // (minimum_amount)
	#[codec(index = 4)]
	A6,
	#[codec(index = 5)]
	A7(ParameterString),                                    // (minimum_amount)
	#[codec(index = 6)]
	A8(BoundedWeb3Network),                                 // litentry, litmus, polkadot, kusama, khala, ethereum
	#[codec(index = 7)]
	A10(ParameterString),                                   // (minimum_amount)
	#[codec(index = 8)]
	A11(ParameterString),                                   // (minimum_amount)

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
}

impl Assertion {
	// Given an assertion enum type, retrieve the supported web3 networks.
	// So we limit the network types on the assertion definition level.
	//
	// The final networks used for assertion building are the common set of:
	// - "assertion supported networks" which are defined here, and
	// - "identity networks" which are defined by the user and stored in `IdentityContext`
	//
	// returns a vector of `Web3Network` guarantees it's a subnet of
	// the broader `Web3Network` (see network.rs)
	pub fn get_supported_web3networks(&self) -> Vec<Web3Network> {
		match self {
			// LIT holder, not including `LitentryRococo` as it's not supported by any data provider
			Self::A4(..) => vec![Web3Network::Litentry, Web3Network::Litmus, Web3Network::Ethereum],
			// DOT holder
			Self::A7(..) => vec![Web3Network::Polkadot],
			// WBTC/ETH holder
			Self::A10(..) |
			Self::A11(..) |
			Self::VIP3MembershipCard(..) |
			Self::WeirdoGhostGangHolder => vec![Web3Network::Ethereum],
			// total tx over `networks`
			Self::A8(network) => network.to_vec(),
			// polkadot paticipation
			Self::A14 => vec![Web3Network::Polkadot],
			// Achainable Assertions
			Self::Achainable(arg) => arg.chains(),
			// Oneblock Assertion
			Self::Oneblock(..) => vec![Web3Network::Polkadot, Web3Network::Kusama],
			// SPACEID Assertions
			Self::BnbDomainHolding | Self::BnbDigitDomainClub(..) => vec![Web3Network::Bsc],
			// LITStaking
			Self::LITStaking => vec![Web3Network::Litentry],
			// EVM Amount Holding
			Self::EVMAmountHolding(_) => vec![Web3Network::Ethereum, Web3Network::Bsc],
			// BRC20 Holder
			Self::BRC20AmountHolder => vec![
				Web3Network::BitcoinP2tr,
				Web3Network::BitcoinP2pkh,
				Web3Network::BitcoinP2sh,
				Web3Network::BitcoinP2wpkh,
				Web3Network::BitcoinP2wsh,
			],
			// we don't care about any specific web3 network
			Self::A1 |
			Self::A2(..) |
			Self::A3(..) |
			Self::A6 |
			Self::A13(..) |
			Self::A20 |
			Self::GenericDiscordRole(..) => vec![],
		}
	}
}

pub const ASSERTION_DATE_LEN: usize = 15;
pub const ASSERTION_FROM_DATE: [&str; ASSERTION_DATE_LEN] = [
	"2017-01-01",
	"2017-07-01",
	"2018-01-01",
	"2018-07-01",
	"2019-01-01",
	"2019-07-01",
	"2020-01-01",
	"2020-07-01",
	"2021-01-01",
	"2021-07-01",
	"2022-01-01",
	"2022-07-01",
	"2023-01-01",
	"2023-07-01",
	// In order to address the issue of the community encountering a false query for WBTC in
	// November, the product team feels that adding this date temporarily solves this problem.
	"2023-12-01",
];

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum AmountHoldingTimeType {
	#[codec(index = 0)]
	LIT,
	#[codec(index = 1)]
	DOT,
	#[codec(index = 2)]
	WBTC,
	#[codec(index = 3)]
	ETH,
}
