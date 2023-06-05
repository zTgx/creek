use codec::{Decode, Encode};
use litentry_api_client::{
    primitives::address::Address32,
    utils::{
        address::{
            create_n_random_sr25519_address, pubkey_to_address32, sr25519_public_from_ss58,
            sr25519_public_to_ss58,
        },
        crypto::{encrypt_with_user_shielding_key, generate_user_shielding_key},
        enclave::mock_a_shard,
    },
};
use sp_core::{sr25519, Pair};

#[cfg(target_arch = "x86_64")]
use litentry_api_client::ra::{SafeSgx, SafeSgxApi};

#[cfg(not(test))]
fn tc_decrypt_vc_works() {
    use litentry_api_client::utils::{
        crypto::decrypt_vc_with_user_shielding_key, primitives::crypto::AesOutput, print_passed,
    };

    let ciphertext = [
        51_u8, 182, 3, 28, 178, 61, 104, 214, 132, 255, 156, 249, 131, 25, 58, 63, 199, 46, 219,
        45, 43, 119, 168, 50, 158, 143, 49, 172, 127, 8, 27, 34, 62, 166, 126, 31, 87, 247, 42, 89,
        77, 141, 206, 70, 30, 191, 114, 30, 47, 85, 242, 18, 118, 125, 101, 169, 70, 182, 99, 16,
        186, 88, 126, 118, 169, 206, 55, 228, 55, 169, 221, 28, 180, 49, 22, 56, 17, 222, 107, 59,
        51, 32, 7, 7, 7, 69, 34, 207, 72, 244, 116, 68, 129, 83, 180, 87, 93, 108, 205, 27, 68,
        167, 197, 189, 55, 38, 112, 235, 52, 135, 91, 238, 220, 40, 190, 131, 32, 23, 218, 15, 235,
        69, 154, 12, 108, 223, 211, 112, 93, 181, 11, 53, 203, 251, 214, 79, 185, 32, 76, 226, 241,
        134, 127, 240, 37, 240, 208, 56, 190, 205, 246, 147, 151, 202, 98, 51, 238, 167, 101, 210,
        41, 199, 68, 228, 35, 24, 128, 172, 29, 229, 55, 226, 242, 123, 178, 208, 22, 199, 60, 20,
        30, 137, 40, 114, 21, 80, 137, 19, 253, 143, 216, 216, 59, 128, 26, 57, 83, 65, 116, 98,
        170, 10, 47, 130, 109, 209, 148, 160, 212, 11, 122, 249, 148, 60, 23, 237, 9, 187, 235,
        192, 216, 71, 27, 68, 43, 75, 199, 93, 43, 41, 150, 140, 138, 112, 249, 96, 85, 85, 216,
        225, 210, 157, 208, 169, 227, 167, 51, 9, 200, 40, 199, 235, 33, 144, 29, 217, 103, 44, 91,
        107, 182, 39, 193, 177, 52, 255, 174, 251, 126, 99, 37, 242, 229, 198, 175, 15, 209, 180,
        151, 118, 122, 151, 192, 196, 148, 148, 138, 33, 248, 9, 133, 216, 5, 159, 186, 139, 212,
        187, 97, 3, 62, 224, 167, 201, 65, 242, 192, 244, 35, 207, 70, 114, 25, 13, 109, 16, 221,
        114, 54, 92, 77, 19, 109, 178, 108, 181, 190, 202, 129, 43, 84, 119, 148, 156, 16, 242, 25,
        182, 66, 159, 91, 148, 150, 40, 175, 26, 102, 47, 231, 96, 142, 98, 34, 71, 199, 175, 128,
        154, 34, 194, 22, 59, 155, 3, 171, 236, 58, 87, 19, 202, 217, 131, 217, 184, 170, 99, 247,
        183, 127, 86, 130, 9, 137, 237, 160, 170, 168, 164, 212, 7, 132, 139, 156, 121, 25, 249,
        49, 120, 39, 82, 111, 233, 111, 213, 135, 78, 187, 92, 234, 220, 234, 153, 214, 189, 174,
        109, 162, 159, 7, 170, 160, 236, 86, 240, 5, 210, 63, 109, 124, 143, 158, 25, 176, 182,
        184, 52, 91, 144, 212, 6, 45, 45, 27, 242, 74, 91, 196, 190, 60, 145, 225, 205, 48, 68,
        218, 104, 157, 120, 215, 175, 172, 227, 210, 146, 154, 103, 127, 46, 10, 192, 116, 179, 51,
        94, 137, 224, 160, 27, 172, 9, 188, 71, 172, 44, 10, 58, 90, 148, 218, 66, 39, 48, 138,
        247, 141, 93, 18, 8, 185, 82, 92, 72, 240, 176, 184, 205, 172, 185, 210, 197, 96, 253, 163,
        55, 153, 230, 226, 137, 30, 102, 218, 245, 170, 22, 159, 8, 33, 109, 65, 85, 82, 68, 95,
        144, 167, 102, 118, 197, 157, 58, 228, 60, 85, 190, 176, 244, 219, 26, 90, 60, 224, 96,
        211, 201, 9, 23, 115, 208, 114, 22, 64, 8, 209, 80, 105, 55, 91, 195, 228, 67, 23, 9, 75,
        196, 94, 224, 230, 141, 213, 102, 81, 244, 118, 246, 43, 57, 227, 54, 9, 221, 204, 242, 36,
        231, 250, 44, 228, 116, 31, 64, 135, 131, 39, 70, 173, 195, 55, 121, 132, 181, 183, 214,
        85, 221, 222, 152, 39, 97, 199, 228, 65, 173, 226, 198, 50, 237, 178, 135, 139, 192, 139,
        111, 20, 201, 66, 161, 194, 175, 35, 174, 177, 2, 216, 54, 221, 83, 244, 95, 56, 95, 99, 9,
        8, 174, 122, 205, 0, 66, 105, 138, 158, 88, 121, 122, 160, 13, 227, 21, 137, 22, 167, 58,
        219, 71, 221, 252, 60, 140, 3, 52, 6, 175, 177, 253, 34, 233, 42, 0, 153, 63, 66, 97, 221,
        177, 170, 96, 5, 94, 195, 248, 1, 64, 199, 64, 93, 134, 81, 170, 39, 249, 33, 216, 104, 77,
        123, 103, 182, 249, 177, 111, 235, 174, 125, 171, 153, 159, 160, 129, 231, 172, 48, 42,
        144, 153, 121, 184, 112, 32, 132, 133, 217, 132, 88, 129, 117, 50, 232, 85, 87, 77, 221,
        140, 209, 240, 224, 247, 85, 21, 15, 198, 106, 76, 21, 127, 108, 166, 203, 178, 24, 176,
        248, 113, 79, 247, 68, 196, 135, 60, 102, 84, 12, 50, 197, 130, 57, 17, 147, 208, 202, 166,
        84, 129, 83, 133, 214, 7, 51, 41, 37, 29, 202, 219, 101, 105, 87, 168, 227, 51, 80, 245,
        195, 211, 255, 38, 87, 199, 149, 178, 29, 13, 229, 7, 132, 10, 30, 255, 88, 233, 171, 189,
        26, 215, 18, 64, 207, 88, 139, 79, 176, 71, 144, 95, 43, 4, 253, 112, 214, 12, 47, 222, 3,
        235, 176, 146, 183, 234, 28, 228, 191, 202, 45, 246, 57, 13, 27, 194, 243, 231, 147, 35,
        105, 201, 184, 175, 197, 93, 127, 36, 98, 157, 63, 201, 151, 120, 245, 221, 52, 27, 135,
        255, 244, 225, 49, 29, 82, 57, 101, 157, 86, 200, 236, 116, 253, 229, 217, 178, 192, 167,
        221, 240, 62, 47, 163, 23, 234, 143, 56, 76, 65, 217, 22, 179, 214, 154, 250, 57, 202, 255,
        175, 141, 59, 68, 33, 77, 155, 248, 47, 15, 113, 34, 241, 205, 79, 1, 247, 184, 46, 206,
        111, 54, 46, 69, 133, 221, 253, 214, 21, 64, 125, 153, 41, 202, 31, 70, 87, 119, 181, 62,
        57, 62, 135, 13, 41, 24, 222, 224, 208, 8, 219, 204, 121, 62, 222, 139, 6, 238, 16, 88,
        131, 14, 13, 61, 169, 231, 114, 237, 121, 140, 220, 148, 209, 179, 89, 199, 81, 207, 17,
        127, 115, 183, 80, 186, 166, 198, 232, 53, 227, 63, 85, 172, 227, 254, 235, 68, 54, 188,
        188, 174, 27, 102, 39, 6, 145, 101, 194, 251, 27, 24, 137, 216, 131, 131, 75, 244, 166,
        143, 50, 42, 176, 100, 245, 153, 220, 97, 52, 149, 242, 72, 160, 98, 151, 165, 152, 112,
        93, 128, 33, 3, 175, 144, 211, 13, 110, 144, 64, 170, 184, 238, 6, 171, 41, 140, 131, 252,
        46, 145, 120, 140, 100, 67, 97, 230, 17, 192, 204, 75, 66, 60, 18, 178, 213, 111, 164, 222,
        55, 49, 187, 84, 42, 184, 82, 200, 157, 27, 96, 132, 163, 92, 183, 46, 155, 87, 210, 207,
        219, 58, 193, 24, 184, 165, 34, 136, 181, 196, 249, 230, 108, 144, 230, 81,
    ];

    let nonce = [0_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let encrypted_vc = AesOutput {
        ciphertext: ciphertext.to_vec(),
        aad: vec![],
        nonce,
    };
    let user_shielding_key = [
        108_u8, 147, 20, 114, 131, 151, 174, 219, 250, 32, 33, 128, 30, 19, 203, 8, 3, 163, 236,
        122, 62, 92, 65, 224, 234, 83, 156, 112, 95, 183, 140, 120,
    ];

    let vc = decrypt_vc_with_user_shielding_key(&user_shielding_key, encrypted_vc);
    assert!(vc.is_ok());

    print_passed();
}

/*
Random Address in Hex:

"0x443b7f4d965fc060b9ecae120d662bac3fad02b170d93f644297e53f44370751", "0xd2c9039be3da6809b1e885067afda4b88d93fc9a8ba13eeb407eb8dacce7b360", "0xbe82fa0c9ef8e48ab04616b472b8b9972a240ba9205668e833a607b1d1907c24", "0x96abc84cdfed1e40e049ca6e5b4593a7491980550ce28c0e63c17e7cb8b02008", "0xa2f8d097f8494a9dacc76ee38aeb177b0d0abb65d0df21b79e6e731409eb6419", "0xdc6b27ab83d735513053221eeebb9a6f2b01eb5c472db70923c9a044865c191d", "0x34d5c848c0a0082f51fafbd2008717f5fef5d2f37e21ce613fd2730a3e0ba300", "0x06d47769f43b09ba305f4bc3f566bef6d008bb7e8849f46701951b9e5848b811", "0x785360ec3dd882fdf0455089dd52d25509aecbe60f48b346d5f03ffda2660235", "0x16c681c7b1a00edb8fb4c9a6d163e3eadfb83742dfc56f41fe27e46c9639fd7b",
 */
#[test]
fn tc_test_create_n_random_sr25519_pair() {
    let address = create_n_random_sr25519_address(10).unwrap();

    let mut ret = String::new();
    address.iter().for_each(|pair| {
        let address: Address32 = pair.public().0.into();

        let mut hex_addr = hex::encode(address.as_ref().to_vec());
        hex_addr.insert_str(0, "\"0x");
        hex_addr.push_str("\", ");

        ret.push_str(&hex_addr)
    });

    println!("all addresses: {}", ret);
}

#[test]
fn tc_test_encrypt_size_works() {
    let user_shielding_key = generate_user_shielding_key();

    let source = "abc123321";
    let encrypted_source =
        encrypt_with_user_shielding_key(&user_shielding_key, source.as_bytes()).unwrap();
    assert_ne!(source.len(), encrypted_source.len());

    let x = serde_json::to_vec(source);
    println!("x: {} / {}", x.unwrap().len(), source.as_bytes().len());

    let x = source.to_string().encode();
    println!("Encode = {} / {}", x.len(), source.len());
}

#[test]
fn tc_mock_shard_works() {
    let shard = mock_a_shard();
    println!("mock shard: {:?}", shard);
}

#[test]
fn tc_pubkey_to_address32_works() {
    let pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let pubkey = pair.public();

    let alice = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
    let alice = pubkey_to_address32(alice).unwrap();

    assert_eq!(pubkey.as_array_ref(), alice.as_ref());
}

#[test]
fn tc_pubkey_to_ss58_works() {
    let pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    let pubkey_source = pair.public();

    let address = sr25519_public_to_ss58(&pubkey_source);
    let pubkey_end = sr25519_public_from_ss58(&address).unwrap();

    assert_eq!(pubkey_source, pubkey_end);
}

#[test]
fn tc_hex_decode_works() {
    let platform_blob = "1502006504000100000A0A0202FF010C0000000000000000000D00000C000000020000000000000C2BCB11AFD9DB7BC5D66F8378B628F2F4AEAF1A50D46EABE7F32A22D4EDDF19C097600958FB39998750C07D988EF78CEB2A935D1DD4E82087DB9602A36E2872A303";
    let bytes = hex::decode(&platform_blob);
    println!("bytes: {:?}", bytes.unwrap().len());
}

#[test]
#[cfg(target_arch = "x86_64")]
fn tc_sgx_report_att_status_works() {
    // from staging server
    let platform_info_blob = [
        4_u8, 0, 1, 0, 0, 10, 10, 2, 2, 255, 1, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 12, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 12, 43, 203, 17, 175, 217, 219, 123, 197, 214, 111, 131, 120, 182,
        40, 242, 244, 174, 175, 26, 80, 212, 110, 171, 231, 243, 42, 34, 212, 237, 223, 25, 192,
        151, 96, 9, 88, 251, 57, 153, 135, 80, 192, 125, 152, 142, 247, 140, 235, 42, 147, 93, 29,
        212, 232, 32, 135, 219, 150, 2, 163, 110, 40, 114, 163, 3,
    ];

    SafeSgx::safe_sgx_report_att_status(platform_info_blob);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn tc_sgx_check_update_status_works() {
    let platform_info_blob = [
        4_u8, 0, 1, 0, 0, 10, 10, 2, 2, 255, 1, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 12, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 12, 43, 203, 17, 175, 217, 219, 123, 197, 214, 111, 131, 120, 182,
        40, 242, 244, 174, 175, 26, 80, 212, 110, 171, 231, 243, 42, 34, 212, 237, 223, 25, 192,
        151, 96, 9, 88, 251, 57, 153, 135, 80, 192, 125, 152, 142, 247, 140, 235, 42, 147, 93, 29,
        212, 232, 32, 135, 219, 150, 2, 163, 110, 40, 114, 163, 3,
    ];

    SafeSgx::safe_sgx_check_update_status(platform_info_blob);
}

/**
 * 
 * Need to move decode nonce to di mode
 * 
 */
#[test]
fn tc_decode_nonce_works() {
    {
        let u = hex::decode("22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12").unwrap();
        let u = u.encode();
        let u = Some(u).encode();
        let u = hex::encode(u);
        println!("u>>>:{}", u);

        // return;

        let u = hex::decode("01848022fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12").unwrap();
        let u: Option<Vec<u8>> = Option::decode(&mut u.as_slice()).ok().unwrap();
        if let Some(u) = u {
            let u = Vec::<u8>::decode(&mut u.as_slice()).unwrap();
            let u = hex::encode(u);
            println!("use shielding key: {}", u);
        }

        return;
    }
    {
        let nonce = 2u32.encode();
        let nonce = Some(nonce);
        let x = nonce.encode();
        let y = hex::encode(x);
        println!("y: {}", y);

        return;
    }
    // RpcReturnValue.value
    // 0x011002000000
    let hex = "011002000000".to_string();
    println!("hex: {:?}", hex);

    let decode_hex = hex::decode(hex).unwrap();
    println!("decode_hex: {:?}", decode_hex);
    let x: Option<Vec<u8>> = Option::decode(&mut decode_hex.as_slice())
        .map_err(|e| {
            println!("Failed to decode return value: {:?}", e);
            e
        })
        .ok()
        .unwrap();

    if let Some(x) = x {
        let nonce = u32::decode(&mut x.as_slice()).unwrap();
        println!("nonce: {:?}", nonce);
    }
}
