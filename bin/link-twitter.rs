use creek::{
	primitives::identity::Identity, Creek, CreekExplorer, ValidationDataBuilder, WorkerOp,
};
use sp_core::{sr25519, Pair};

fn main() {
	// 1: Import your main account as signer (or primary identity).
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

	// 2: Set this alice as signer.
	let parachain_endpoint = "wss://tee-internal.litentry.io:443";
	let worker_endpoint: &str = "wss://localhost:2600";
	let creek = Creek::explorer(parachain_endpoint, worker_endpoint, alice.into()).unwrap();

	// Local test, please replace this to sidechain nonce.
	let vdata = creek.twitter_vdata("0").unwrap();
	let identity = Identity::Twitter("mock_user".to_string());
	let networks = vec![];

	let _ = creek.link_identity(identity, networks, vdata);
}
