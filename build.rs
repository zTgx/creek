extern crate cc;

fn main() {
    let c_file = "src/ra/lib_sgx.c";
    let header_dir = "/opt/intel/sgxsdk/include";
    let lib_dir = "/opt/intel/sgxsdk/lib64";
    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo:rustc-link-lib=sgx_epid_sim");

    cc::Build::new()
        .file(c_file)
        .include(header_dir)
        .static_flag(true)
        .compile("lib_sgx");
}
