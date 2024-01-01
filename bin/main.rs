use creek::{
	primitives::{assertion::Assertion, identity::Identity, network::Web3Network},
	Creek, ValidationDataBuilder, WorkerSTF,
};
use sp_core::{sr25519, Pair};

fn main() {
	// 1: Import your main account as signer (or primary identity).
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

	// 2: Set this alice as signer.
	let parachain_endpoint = "wss://tee-internal.litentry.io:443";
	let worker_endpoint: &str = "wss://localhost:2600";
	let creek = Creek::new(parachain_endpoint, worker_endpoint, alice.into());

	// 3: Import your another account(bob) you want to LINK.
	let bob = sr25519::Pair::from_string("//Bob", None).unwrap();

	// 4: Using bob to sign a message to prove that you own this to-being-linked account.
	let vdata = creek.web3_vdata(&bob.clone().into()).unwrap();

	// 5: Build identity from this bob account.
	// If bob is from ethereum ecos then Using Identity::Evm(..) or
	// If bob is from Substrate ecos then Using Identity::Substrate.
	// We don't care about WEB2 identity in this demo.
	// But they're follow the same rules.
	let bob_identity = Identity::Substrate(bob.public().into());

	// 6: Specify the network your to-being-linke account(bob) comes from.
	let networks = vec![Web3Network::Litentry];

	// 7: Call `link-identity` to link your bob account.
	let _ = creek.link_identity(bob_identity, networks, vdata);

	// 8: Select which `Assertion` you want to request.
	let assertion = Assertion::A1;
	let _ = creek.request_vc(assertion);
}
