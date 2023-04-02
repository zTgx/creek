#![no_main]

use libfuzzer_sys::fuzz_target;
use litentry_api_client::vc_management::fuzz::fuzz_request_vc_a4;

fuzz_target!(|_data: &[u8]| {
    // fuzzed code goes here
    
    let data = _data.to_vec();
    if data.len() == 16 {

        let mut bytes = [0_u8; 16];
        bytes[..16].clone_from_slice(&data);

        let value = u128::from_le_bytes(bytes);
        fuzz_request_vc_a4(value);
    }
});
