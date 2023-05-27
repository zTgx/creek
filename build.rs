#[cfg(target_arch = "x86_64")]
extern crate cc;

#[cfg(target_arch = "x86_64")]
fn main() {
    let c_file = "src/ra/lib_sgx.c";
    let header_dir = "/opt/intel/sgxsdk/include";
    let lib_dir = "/opt/intel/sgxsdk/lib64";
    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo:rustc-link-lib=sgx_epid_sim");
    println!("cargo:rustc-link-lib=sgx_utls"); // sgx_verify_report

    cc::Build::new()
        .file(c_file)
        .include(header_dir)
        .static_flag(true)
        .compile("lib_sgx");
}

#[cfg(target_os = "macos")]
fn main() {}
