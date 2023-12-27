use codec::Encode;
use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
	compose_extrinsic, extrinsic::common::Batch, CallIndex, PlainTip, SubstrateDefaultSignedExtra,
	UncheckedExtrinsicV4,
};

use crate::ApiClient;

pub trait BatchPatch {
	fn batch_all<Call: Encode + Clone>(
		&self,
		calls: &[Call],
	) -> UtilityBatchAllXt<Call, SubstrateDefaultSignedExtra<PlainTip>>;
}

const UTILITY_MODULE: &str = "Utility";
const UTILITY_BATCH_ALL: &str = "batch_all";

pub type UtilityBatchAllFn<Call> = (CallIndex, Batch<Call>);
pub type UtilityBatchAllXt<Call, SignedExtra> =
	UncheckedExtrinsicV4<UtilityBatchAllFn<Call>, SignedExtra>;

impl<P> BatchPatch for ApiClient<P>
where
	P: Pair,
	MultiSignature: From<P::Signature>,
	MultiSigner: From<P::Public>,
{
	fn batch_all<Call: Encode + Clone>(
		&self,
		calls: &[Call],
	) -> UtilityBatchAllXt<Call, SubstrateDefaultSignedExtra<PlainTip>> {
		let calls = Batch { calls: calls.to_vec() };
		compose_extrinsic!(self.api.clone(), UTILITY_MODULE, UTILITY_BATCH_ALL, calls)
	}
}
