use std::env;
use std::process::Command;

fn main() {
    let sdk_dir = env::var("SGX_SDK").unwrap_or_else(|_| "/opt/intel/sgxsdk".to_string());

    #[cfg(target_arch = "x86")]
    let edger8r = format!("{}/bin/x86/sgx_edger8r", sdk_dir);
    #[cfg(not(target_arch = "x86"))]
    let edger8r = format!("{}/bin/x64/sgx_edger8r", sdk_dir);

    Command::new(edger8r)
        .args(&[
            "--untrusted",
            "../enclave/Enclave.edl",
            "--search-path",
            &format!("{}/include", sdk_dir),
            "--search-path",
            "../../rust-sgx-sdk/edl",
            "--untrusted-dir",
            ".",
        ])
        .status()
        .unwrap();

    cc::Build::new()
        .file("Enclave_u.c")
        .include(&format!("{}/include", sdk_dir))
        .include("../../rust-sgx-sdk/edl")
        .compile("enclave.a");

    #[cfg(target_arch = "x86")]
    println!("cargo:rustc-link-search=native={}/lib", sdk_dir);
    #[cfg(not(target_arch = "x86"))]
    println!("cargo:rustc-link-search=native={}/lib64", sdk_dir);

    println!("cargo:rustc-link-lib=dylib=sgx_urts");
    println!("cargo:rustc-link-lib=dylib=sgx_uae_service");

    println!("cargo:rerun-if-changed=../enclave/Enclave.edl");
}
