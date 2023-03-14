use litentry_test_suit::vc_management::lit_request_vc::*;
use litentry_test_suit::identity_management::lit_set_user_shielding_key::set_user_shielding_key;

#[test]
fn tc_vm_00() {
    set_user_shielding_key();
    tc_vm_00_request_vc();
}
