fn main() {
    println!("cargo:rerun-if-env-changed=UNC_SANDBOX_BIN_PATH");

    let doc_build = cfg!(doc) || std::env::var("DOCS_RS").is_ok();
    let env_bin = std::env::var("UNC_SANDBOX_BIN_PATH").is_ok();
    if !doc_build && !env_bin && cfg!(feature = "install") {
        unc_sandbox_utils::install().expect("Could not install sandbox");
    }
}
