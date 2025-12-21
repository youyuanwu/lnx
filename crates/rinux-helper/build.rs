// This compiles the c helper functions into a static library for linking.
fn main() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_dir = manifest_dir.parent().unwrap().parent().unwrap();

    // Kernel source and build directories
    let linux_src = root_dir.join("linux");
    let linux_build = root_dir.join("linux_bin");

    // Include paths for kernel headers
    let inc_dir = linux_src.join("include");
    let inc_generated = linux_build.join("include");
    let inc_arch_src = linux_src.join("arch").join("x86").join("include");
    let inc_arch_generated = linux_build
        .join("arch")
        .join("x86")
        .join("include")
        .join("generated");
    let autoconf = linux_build
        .join("include")
        .join("generated")
        .join("autoconf.h");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let obj_file = out_dir.join("helpers.o");
    let archive_file = out_dir.join("librinux_helpers.a");

    // Compile C helper file directly with clang, matching kernel build flags
    let output = std::process::Command::new("clang")
        .arg("-c")
        .arg("src/helpers.c")
        .arg("-o")
        .arg(&obj_file)
        .arg("--target=x86_64-linux-gnu")
        .arg(format!("-I{}", inc_dir.display()))
        .arg(format!("-I{}", inc_generated.display()))
        .arg(format!("-I{}", inc_arch_src.display()))
        .arg(format!("-I{}", inc_arch_generated.display()))
        .arg("-include")
        .arg(&autoconf)
        .arg("-D__KERNEL__")
        .arg("-DMODULE")
        .arg("-D__BINDGEN__")
        .arg("-fno-builtin")
        .arg("-fno-PIE")
        .arg("-fno-strict-aliasing")
        .arg("-fno-common")
        .arg("-std=gnu11")
        .arg("-w") // Suppress warnings
        .output()
        .expect("Failed to execute clang");

    if !output.status.success() {
        eprintln!("clang stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("clang stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Failed to compile helpers.c");
    }

    // Create static library from object file
    let ar_output = std::process::Command::new("ar")
        .arg("crus")
        .arg(&archive_file)
        .arg(&obj_file)
        .output()
        .expect("Failed to execute ar");

    if !ar_output.status.success() {
        eprintln!("ar stderr: {}", String::from_utf8_lossy(&ar_output.stderr));
        panic!("Failed to create static library");
    }

    // Tell cargo to link the library
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=rinux_helpers");
    println!("cargo:rerun-if-changed=src/helpers.c");
}
