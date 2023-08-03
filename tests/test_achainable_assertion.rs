use litentry_api_client::{
    api_client_patch::parachain::ParachainPatch,
    identity_management::IdentityManagementApi,
    primitives::assertion::{
        AchainableAmount, AchainableBasic, AchainableClassOfYear, AchainableParams, Assertion,
        ParameterString,
    },
    utils::{crypto::generate_user_shielding_key, print_passed},
    vc_management::VcManagementApi,
    ApiClient,
};
use sp_core::{sr25519, Pair};

#[test]
fn tc_achainable_class_of_year_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let name = "Account created between {dates}";
    let name = ParameterString::try_from(name.as_bytes().to_vec()).unwrap();
    let chain = "ethereum";
    let chain = ParameterString::try_from(chain.as_bytes().to_vec()).unwrap();
    let date1 = "2020-01-01";
    let date1 = ParameterString::try_from(date1.as_bytes().to_vec()).unwrap();
    let date2 = "2021-01-01";
    let date2 = ParameterString::try_from(date2.as_bytes().to_vec()).unwrap();

    let class_of_year = AchainableClassOfYear {
        name,
        chain,
        date1,
        date2,
    };
    let assertion = Assertion::Achainable(AchainableParams::ClassOfYear(class_of_year));
    println!("[Assertion]>>> {:?}", assertion);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion ClassOfYear <<<<<<<<<<<<<<<<<<<<<<<< ");

    api_client.request_vc(&shard, &assertion);

    print_passed();
}

// #[test]
fn tc_achainable_eth_holder_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let name = "Balance over {amount}";
    let name = ParameterString::try_from(name.as_bytes().to_vec()).unwrap();
    let chain = "ethereum";
    let chain = ParameterString::try_from(chain.as_bytes().to_vec()).unwrap();
    let amount = "0";
    let amount = ParameterString::try_from(amount.as_bytes().to_vec()).unwrap();

    let eth_holder = AchainableAmount {
        name,
        chain,
        amount,
    };
    let assertion = Assertion::Achainable(AchainableParams::Amount(eth_holder));
    println!("[Assertion]>>> {:?}", assertion);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion ETH holder <<<<<<<<<<<<<<<<<<<<<<<< ");

    api_client.request_vc(&shard, &assertion);

    print_passed();
}

// #[test]
fn tc_achainable_lit_holder_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let name = "Balance over {amount}";
    let name = ParameterString::try_from(name.as_bytes().to_vec()).unwrap();
    let chain = "litentry";
    let chain = ParameterString::try_from(chain.as_bytes().to_vec()).unwrap();
    let amount = "0";
    let amount = ParameterString::try_from(amount.as_bytes().to_vec()).unwrap();

    let eth_holder = AchainableAmount {
        name,
        chain,
        amount,
    };
    let assertion = Assertion::Achainable(AchainableParams::Amount(eth_holder));
    println!("[Assertion]>>> {:?}", assertion);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion LIT holder <<<<<<<<<<<<<<<<<<<<<<<< ");

    api_client.request_vc(&shard, &assertion);

    print_passed();
}

// #[test]
fn tc_achainable_dot_holder_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let name = "Balance over {amount}";
    let name = ParameterString::try_from(name.as_bytes().to_vec()).unwrap();
    let chain = "polkadot";
    let chain = ParameterString::try_from(chain.as_bytes().to_vec()).unwrap();
    let amount = "0";
    let amount = ParameterString::try_from(amount.as_bytes().to_vec()).unwrap();

    let eth_holder = AchainableAmount {
        name,
        chain,
        amount,
    };
    let assertion = Assertion::Achainable(AchainableParams::Amount(eth_holder));
    println!("[Assertion]>>> {:?}", assertion);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion DOT holder <<<<<<<<<<<<<<<<<<<<<<<< ");

    api_client.request_vc(&shard, &assertion);

    print_passed();
}

// #[test]
fn tc_achainable_contract_creator_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let name = "Created over {amount} contracts";
    let name = ParameterString::try_from(name.as_bytes().to_vec()).unwrap();
    let chain = "ethereum";
    let chain = ParameterString::try_from(chain.as_bytes().to_vec()).unwrap();
    let amount = "0";
    let amount = ParameterString::try_from(amount.as_bytes().to_vec()).unwrap();

    let class_of_year = AchainableAmount {
        name,
        chain,
        amount,
    };
    let assertion = Assertion::Achainable(AchainableParams::Amount(class_of_year));
    println!("[Assertion]>>> {:?}", assertion);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion ClassOfYear <<<<<<<<<<<<<<<<<<<<<<<< ");

    api_client.request_vc(&shard, &assertion);

    print_passed();
}

// #[test]
fn tc_achainable_nniswap_v2_v3_user_works() {
    let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
    let api_client = ApiClient::new_with_signer(alice).unwrap();

    let shard = api_client.get_shard().unwrap();
    let user_shielding_key = generate_user_shielding_key();
    api_client
        .set_user_shielding_key(&shard, &user_shielding_key)
        .unwrap();

    let name = "Account created between {dates}";
    let name = ParameterString::try_from(name.as_bytes().to_vec()).unwrap();
    let chain = "ethereum";
    let chain = ParameterString::try_from(chain.as_bytes().to_vec()).unwrap();

    let basic = AchainableBasic { name, chain };
    let assertion = Assertion::Achainable(AchainableParams::Basic(basic));
    println!("[Assertion]>>> {:?}", assertion);

    println!("\n\n\n 🚧 >>>>>>>>>>>>>>>>>>>>>>> Starting Request Assertion Uniswap V2/V3 User <<<<<<<<<<<<<<<<<<<<<<<< ");

    api_client.request_vc(&shard, &assertion);

    print_passed();
}
