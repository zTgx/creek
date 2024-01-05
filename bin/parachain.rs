use creek::{Creek, CreekExplorer, ParachainOp};
use sp_core::{sr25519, Pair};

pub const LOCAL: (&str, &str) = ("wss://localhost:9944", "wss://localhost:2600");
pub const INTERNAL: (&str, &str) =
	("wss://tee-internal.litentry.io:443", "wss://tee-internal.litentry.io:2000");
pub const STAGING: (&str, &str) =
	("wss://tee-staging.litentry.io:443", "wss://tee-staging.litentry.io:2000");

pub trait GetUrl {
	fn get_url(&self) -> (&'static str, &'static str);
}

pub enum Env {
	Local,
	Internal,
	Staging,
}

impl GetUrl for Env {
	fn get_url(&self) -> (&'static str, &'static str) {
		match self {
			Env::Local => LOCAL,
			Env::Internal => INTERNAL,
			Env::Staging => STAGING,
		}
	}
}

fn main() {
	// 1: Import your main account as signer (or primary identity).
	let alice = sr25519::Pair::from_string("//Alice", None).unwrap();

	// Select Env
	let env = Env::Staging;

	// 2: Set this alice as signer.
	let creek = Creek::explorer(env.get_url().0, env.get_url().1, alice.into()).unwrap();

	let contexts = creek.vc_registry().unwrap();
	println!("VC contexts: {:#?}", contexts);
}
