use creek::{Creek, ParachainOp};
use sp_core::{sr25519, Pair};

fn main() {
	// 1: Import your main account as signer (or primary identity).
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

	// 2: Set this alice as signer.
	let parachain_endpoint = "wss://tee-internal.litentry.io:443";
	let worker_endpoint: &str = "wss://localhost:2600";
	let creek = Creek::new(parachain_endpoint, worker_endpoint, alice.into()).unwrap();

	let contexts = creek.vc_registry().unwrap();
	println!("VC contexts: {:#?}", contexts);
}
