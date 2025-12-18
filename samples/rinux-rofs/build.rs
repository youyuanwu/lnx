fn main() {
    // set the RUST_MODFILE env var
    println!("cargo:rustc-env=RUST_MODFILE=rinux-rofs");
    // Declare MODULE as a valid cfg to avoid warnings
    println!("cargo:rustc-check-cfg=cfg(MODULE)");
    // Inform kernel module build system that we are building a module
    println!("cargo:rustc-cfg=MODULE");
}
