use creek::{
	primitives::{identity::Identity, network::Web3Network},
	Creek, ValidationDataBuilder, WorkerSTF,
};
use sp_core::{sr25519, Pair};

fn main() {
	// First: Import your main account as signer (or primary identity).
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

	// Second: Set this alice as signer.
	let creek = Creek::new_with_signer(alice.into());

	// Third: Import your another account(bob) you want to LINK.
	let bob = sr25519::Pair::from_string("//Bob", None).unwrap();

	// Fourth: Using bob to sign a message to prove that you own this to-being-linked account.
	let vdata = creek.web3_vdata(&bob.clone().into()).unwrap();

	// Fifth: Build identity from this bob account.
	// If bob is from ethereum ecos then Using Identity::Evm(..) or
	// If bob is from Substrate ecos then Using Identity::Substrate.
	// We don't care about WEB2 identity in this demo.
	// But they're follow the same rules.
	let bob_identity = Identity::Substrate(bob.public().into());

	// Before Finally: Specify the network your to-being-linke account(bob) comes from.
	let networks = vec![Web3Network::Litentry];

	// Finally: Call `link-identity` to link your bob account.
	let _ = creek.link_identity(bob_identity, networks, vdata);
}
