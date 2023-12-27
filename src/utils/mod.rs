pub mod address;
pub mod crypto;
pub mod di;
pub mod enclave;
pub mod hex;
pub mod identity;
pub mod vc;

pub fn print_passed() {
	println!(" 🎉 All testcases passed!");
}

pub fn print_failed(reason: String) {
	println!(" ❌ Testcase failed, reason: {}", reason);
}
