use crate::{
	primitives::{assertion::Assertion, MrEnclave},
	vc_management::VC_PALLET_NAME,
	ApiClient,
};
use sp_core::{Pair, H256};
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
	compose_extrinsic, CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};

pub type VCRequestFn = (CallIndex, H256, Assertion);
pub type VCRequestXt<SignedExtra> = UncheckedExtrinsicV4<VCRequestFn, SignedExtra>;
pub type VCDisableFn = (CallIndex, H256);
pub type VCDisableXt<SignedExtra> = UncheckedExtrinsicV4<VCDisableFn, SignedExtra>;
pub type VCRevokeFn = (CallIndex, H256);
pub type VCRevokeXt<SignedExtra> = UncheckedExtrinsicV4<VCRevokeFn, SignedExtra>;

pub trait VcManagementXtBuilder {
	fn build_extrinsic_request_vc(
		&self,
		shard: &MrEnclave,
		assertion: &Assertion,
	) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_disable_vc(
		&self,
		vc_index: &H256,
	) -> VCDisableXt<SubstrateDefaultSignedExtra<PlainTip>>;

	fn build_extrinsic_revoke_vc(
		&self,
		vc_index: &H256,
	) -> VCRevokeXt<SubstrateDefaultSignedExtra<PlainTip>>;
}

impl<P> VcManagementXtBuilder for ApiClient<P>
where
	P: Pair,
	MultiSignature: From<P::Signature>,
	MultiSigner: From<P::Public>,
{
	fn build_extrinsic_request_vc(
		&self,
		shard: &MrEnclave,
		assertion: &Assertion,
	) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>> {
		compose_extrinsic!(
			self.api.clone(),
			VC_PALLET_NAME,
			"request_vc",
			H256::from(shard),
			assertion.clone()
		)
	}

	fn build_extrinsic_disable_vc(
		&self,
		vc_index: &H256,
	) -> VCDisableXt<SubstrateDefaultSignedExtra<PlainTip>> {
		compose_extrinsic!(self.api.clone(), VC_PALLET_NAME, "disable_vc", *vc_index)
	}

	fn build_extrinsic_revoke_vc(
		&self,
		vc_index: &H256,
	) -> VCRevokeXt<SubstrateDefaultSignedExtra<PlainTip>> {
		compose_extrinsic!(self.api.clone(), VC_PALLET_NAME, "revoke_vc", *vc_index)
	}
}
