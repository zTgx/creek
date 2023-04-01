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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::BoundedVec;

use super::{Balance, MaxStringLength};

pub type ParameterString = BoundedVec<u8, MaxStringLength>;
pub type IndexingNetworks = BoundedVec<IndexingNetwork, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum IndexingNetwork {
    Litentry,
    Litmus,
    Polkadot,
    Kusama,
    Khala,
    Ethereum,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
    A1,
    A2(ParameterString),                                   // (guild_id)
    A3(ParameterString, ParameterString, ParameterString), // (guild_id, channel_id, role_id)
    A4(Balance),                                           // (minimum_amount)
    A5(ParameterString, ParameterString),                  // (twitter_account, tweet_id)
    A6,
    A7(Balance),          // (minimum_amount)
    A8(IndexingNetworks), // litentry, litmus, polkadot, kusama, khala, ethereum
    A9,
    A10(Balance), // (minimum_amount)
    A11(Balance), // (minimum_amount)
    A13(u32),     // (Karma_amount) - TODO: unsupported
}
