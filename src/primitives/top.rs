use codec::{Encode, Decode};
use std::fmt::Debug;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedOperation<TCS, G>
where
	TCS: PartialEq + Encode + Debug,
	G: PartialEq + Encode + Debug,
{
	#[codec(index = 0)]
	indirect_call(TCS),
	#[codec(index = 1)]
	direct_call(TCS),
	#[codec(index = 2)]
	get(G),
}