use sp_core::H256;

use crate::primitives::{Assertion, MrEnclave, VCContext};

pub mod api;
pub mod events;
pub mod fuzz;
pub mod verify;
pub mod xtbuilder;

pub const VC_PALLET_NAME: &str = "VCManagement";

pub trait VcManagementApi {
    fn request_vc(&self, shard: &MrEnclave, assertion: &Assertion);
    fn disable_vc(&self, vc_index: &H256);
    fn revoke_vc(&self, vc_index: &H256);
}

pub trait VcManagementQueryApi {
    fn vc_registry(&self, vc_index: &H256) -> Option<VCContext>;
}
