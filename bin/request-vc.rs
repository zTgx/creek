use creek::{primitives::assertion::Assertion, Creek, CreekExplorer, WorkerOp};
use sp_core::{sr25519, Pair};

fn main() {
	// 1: Import your main account as signer (or primary identity).
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

	// 2: Set this alice as signer.
	let parachain_endpoint = "wss://tee-internal.litentry.io:443";
	let worker_endpoint: &str = "wss://localhost:2600";
	let creek = Creek::explorer(parachain_endpoint, worker_endpoint, alice.into()).unwrap();

	// 8: Select which `Assertion` you want to request.
	let assertion = Assertion::A1;
	let _ = creek.request_vc(assertion);
}
