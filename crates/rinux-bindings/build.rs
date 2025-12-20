fn main() {
    // get manifest dir
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let bindings_src_dir = format!("{}/../../build/linux_bin/rust/bindings", manifest_dir);
    // create a symlink to bindings_generated.rs file if not exists
    let src = format!("{}/bindings_generated.rs", bindings_src_dir);
    let helper_src = format!("{}/bindings_helpers_generated.rs", bindings_src_dir);
    let dst_helper = format!("{}/bindings_helpers_generated.rs", out_dir);
    let dst = format!("{}/bindings_generated.rs", out_dir);
    if !std::path::Path::new(&dst).exists() {
        std::os::unix::fs::symlink(src, dst).unwrap();
    }
    if !std::path::Path::new(&dst_helper).exists() {
        std::os::unix::fs::symlink(helper_src, dst_helper).unwrap();
    }
    // config CONFIG_RUSTC_HAS_UNNECESSARY_TRANSMUTES
    println!("cargo::rustc-check-cfg=cfg(CONFIG_RUSTC_HAS_UNNECESSARY_TRANSMUTES)");
    println!("cargo:rustc-cfg=CONFIG_RUSTC_HAS_UNNECESSARY_TRANSMUTES");
}
