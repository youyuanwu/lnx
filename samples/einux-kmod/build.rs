fn main() {
    // set the RUST_MODFILE env var
    println!("cargo:rustc-env=RUST_MODFILE={}", "einux-kmod");
    // Declare MODULE as a valid cfg to avoid warnings
    println!("cargo:rustc-check-cfg=cfg(MODULE)");
    // Inform kernel module build system that we are building a module
    println!("cargo:rustc-cfg=MODULE");
}
// Object file can be generated here (without deps):
// target/target/release/deps
// cargo rustc -p einux-kmod --release -- --emit=obj
// then need to manually link with kernel build system.
// Transitive dependencies also need to be manually built into object files.
// Cargo cannot do -C relocation-model=static because some of the deps are not rlib.
//
// See Makefile and Kbuild for more details.
