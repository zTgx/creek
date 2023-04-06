extern crate cc;

fn main() {
    // gcc lib_sgx.c -I/opt/intel/sgxsdk/include -L/opt/intel/sgxsdk/lib64 -lsgx_epid_sim -o lib_sgx
    let lib_dir = "/opt/intel/sgxsdk/lib64";
    println!("cargo:rustc-link-search={}", lib_dir);
    println!("cargo:rustc-link-lib=sgx_epid_sim");

    cc::Build::new()
        .file("src/ra/lib_sgx.c")
        .include("/opt/intel/sgxsdk/include")
        .static_flag(true)
        .compile("lib_sgx");
}
