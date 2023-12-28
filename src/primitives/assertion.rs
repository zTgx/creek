// Copyright 2020-2023 Litentry Technologies GmbH.
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

use super::{
	network::{BoundedWeb3Network, Web3Network},
	MaxStringLength,
};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{AccountId32, BoundedVec};

pub type ParameterString = BoundedVec<u8, MaxStringLength>;
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmountHolding {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub amount: ParameterString,
	pub date: ParameterString,
	pub token: Option<ParameterString>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmountToken {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub amount: ParameterString,
	pub token: Option<ParameterString>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmount {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub amount: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableAmounts {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub amount1: ParameterString,
	pub amount2: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableBasic {
	pub name: ParameterString,
	pub chain: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableBetweenPercents {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub greater_than_or_equal_to: ParameterString,
	pub less_than_or_equal_to: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableClassOfYear {
	pub name: ParameterString,
	pub chain: ParameterString, // supported network, see function web3_network_to_chain
	pub date1: ParameterString, // date format: 2017-01-01
	pub date2: ParameterString, // date format: 2017-01-01
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDateInterval {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub start_date: ParameterString,
	pub end_date: ParameterString,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDatePercent {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub token: ParameterString,
	pub date: ParameterString,
	pub percent: ParameterString,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableDate {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub date: ParameterString,
}
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub struct AchainableToken {
	pub name: ParameterString,
	pub chain: ParameterString,
	pub token: ParameterString,
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum AchainableParams {
	AmountHolding(AchainableAmountHolding),
	AmountToken(AchainableAmountToken),
	Amount(AchainableAmount),
	Amounts(AchainableAmounts),
	Basic(AchainableBasic),
	BetweenPercents(AchainableBetweenPercents),
	ClassOfYear(AchainableClassOfYear),
	DateInterval(AchainableDateInterval),
	DatePercent(AchainableDatePercent),
	Date(AchainableDate),
	Token(AchainableToken),
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
		}
	}
}

#[rustfmt::skip]
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
	A1,
	A2(ParameterString),                                    // (guild_id)
	A3(ParameterString, ParameterString, ParameterString),  // (guild_id, channel_id, role_id)
	A4(ParameterString),                                    // (minimum_amount)
	A6,
	A7(ParameterString),                                    // (minimum_amount)
	A8(BoundedWeb3Network),                                 // litentry, litmus, polkadot, kusama, khala, ethereum
	A9,
	A10(ParameterString),                                   // (minimum_amount)
	A11(ParameterString),                                   // (minimum_amount)

	// ----- begin polkadot decoded 2023 -----
	A13(AccountId32),                                         // (participant_account), can only be requested by delegatee
	A14,
	// for Holder assertions we'll reuse A4/A7
	// ----- end polkadot decoded 2023 -----

	Achainable(AchainableParams),
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
			Self::A10(..) | Self::A11(..) => vec![Web3Network::Ethereum],
			// total tx over `networks`
			Self::A8(network) => network.to_vec(),
			// polkadot paticipation
			Self::A14 => vec![Web3Network::Polkadot],
			// Achainable Assertions
			Self::Achainable(a) => {
				let name = &a.name();
				achainable_networks(name)
			},
			// we don't care about any specific web3 network
			_ => vec![],
		}
	}
}

pub const ASSERTION_FROM_DATE: [&str; 14] = [
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
];

fn achainable_networks(name: &ParameterString) -> Vec<Web3Network> {
	let name = &name.clone().to_vec();
	let name = std::str::from_utf8(name).unwrap_or("");

	if name == "Validator" ||
		name == "TreasuryProposalBeneficiary" ||
		name == "TipFinder" ||
		name == "TipBeneficiary" ||
		name == "OpenGovProposer" ||
		name == "FellowshipProposer" ||
		name == "FellowshipMember" ||
		name == "ExCouncilor" ||
		name == "Councilor" ||
		name == "BountyCurator" ||
		name == "Balance between percents"
	{
		return vec![Web3Network::Litmus, Web3Network::Polkadot]
	} else if name == "Account found on {chain}" {
		return vec![
			Web3Network::Litentry,
			Web3Network::Litmus,
			Web3Network::Ethereum,
			Web3Network::Polkadot,
			Web3Network::Kusama,
			Web3Network::Khala,
			Web3Network::BSC,
		]
	} else if name == "Account total transactions under {amount}" ||
		name == "Balance under {amount}" ||
		name == "Balance over {amount}" ||
		name == "Balance over {amount} dollars" ||
		name == "Balance between {amounts}" ||
		name == "Account created after {date}" ||
		name == "Account created before {date}" ||
		name == "Account created between {dates}" ||
		name == "Balance hodling {amount} since {date}"
	{
		return vec![
			Web3Network::Litentry,
			Web3Network::Litmus,
			Web3Network::Ethereum,
			Web3Network::Polkadot,
			Web3Network::Kusama,
			Web3Network::Khala,
		]
	} else if name == "ERC20 balance over {amount}" ||
		name == "Uniswap V2 liquidity provider" ||
		name == "Uniswap V3 liquidity provider" ||
		name == "Curve Trader" ||
		name == "Curve Liquidity Provider" ||
		name == "MetaMask trader" ||
		name == "Uniswap V2 trader" ||
		name == "Uniswap V3 trader" ||
		name == "Uniswap V2 {token} liquidity provider" ||
		name == "Uniswap V3 {token} liquidity provider" ||
		name == "Aave V2 Lender" ||
		name == "Aave V2 Borrower" ||
		name == "Aave V3 Lender" ||
		name == "Aave V3 Borrower" ||
		name == "ERC20 hodling {amount} of {token} since {date}"
	{
		return vec![Web3Network::Ethereum]
	} else if name == "BEP20 balance over {amount}" {
		return vec![Web3Network::BSC]
	} else if name == "Balance dropped {percent} since {date}" {
		return vec![Web3Network::Ethereum, Web3Network::BSC]
	}

	vec![]
}
