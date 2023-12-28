use self::hex::{JsonResponse, remove_whitespace};

pub mod address;
pub mod crypto;
pub mod di;
pub mod enclave;
pub mod hex;
pub mod identity;
pub mod vc;

pub fn decode_rpc_methods(jsonreponse: &JsonResponse) -> Vec<String> {
    let mut sresult = remove_whitespace(&jsonreponse.result);
    sresult.remove_matches("methods:[");
    sresult.remove_matches("]");

    let mut rpc_methods = vec![];
    let methods: Vec<&str> = sresult.split(',').collect();
    methods.iter().for_each(|m| {
        rpc_methods.push(m.to_string());
    });

    rpc_methods
}