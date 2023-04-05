use crate::primitives::{
    address::Address32,
    identity::{Identity, ValidationData},
    MrEnclave,
};

pub mod api;
pub mod events;
pub mod fuzz;
pub mod xtbuilder;

pub const IDENTITY_PALLET_NAME: &str = "IdentityManagement";

pub trait IdentityManagementApi {
    fn set_user_shielding_key(&self, shard: &MrEnclave, user_shielding_key: &[u8]);
    fn add_delegatee(&self, account: &Address32);
    fn create_identity(
        &self,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    );
    fn create_identity_offline(
        &self,
        nonce: u32,
        shard: &MrEnclave,
        address: &Address32,
        identity: &Identity,
        ciphertext_metadata: &Option<Vec<u8>>,
    );
    fn remove_identity(&self, shard: &MrEnclave, identity: &Identity);
    fn verify_identity(&self, shard: &MrEnclave, identity: &Identity, vdata: &ValidationData);
}
