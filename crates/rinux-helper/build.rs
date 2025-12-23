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
    let inc_uapi = linux_src.join("include").join("uapi");
    let inc_uapi_generated = linux_build.join("include").join("generated").join("uapi");
    let inc_arch_uapi = linux_src
        .join("arch")
        .join("x86")
        .join("include")
        .join("uapi");
    let inc_arch_uapi_generated = linux_build
        .join("arch")
        .join("x86")
        .join("include")
        .join("generated")
        .join("uapi");
    let autoconf = linux_build
        .join("include")
        .join("generated")
        .join("autoconf.h");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let obj_file = out_dir.join("helpers.o");
    let archive_file = out_dir.join("librinux_helpers.a");

    // Get clang's resource directory for compiler built-ins
    let clang_resource_output = std::process::Command::new("clang")
        .arg("-print-resource-dir")
        .output()
        .expect("Failed to get clang resource dir");
    let clang_resource_dir = String::from_utf8_lossy(&clang_resource_output.stdout)
        .trim()
        .to_string();
    let clang_include = format!("{}/include", clang_resource_dir);

    // Compile C helper file directly with clang, matching kernel build flags
    let output = std::process::Command::new("clang")
        .arg("-c")
        .arg("src/helpers.c")
        .arg("-o")
        .arg(&obj_file)
        .arg("--target=x86_64-linux-gnu")
        .arg("-nostdinc") // Don't use standard system includes
        .arg("-isystem")
        .arg(&clang_include) // Use compiler built-ins only
        .arg(format!("-I{}", inc_dir.display()))
        .arg(format!("-I{}", inc_generated.display()))
        .arg(format!("-I{}", inc_arch_src.display()))
        .arg(format!("-I{}", inc_arch_generated.display()))
        .arg(format!("-I{}", inc_uapi.display()))
        .arg(format!("-I{}", inc_uapi_generated.display()))
        .arg(format!("-I{}", inc_arch_uapi.display()))
        .arg(format!("-I{}", inc_arch_uapi_generated.display()))
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

    overwrite_bindgen();
}

fn overwrite_bindgen() {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root_dir = manifest_dir.parent().unwrap().parent().unwrap();

    // Kernel source and build directories
    let linux_src = root_dir.join("linux");
    let linux_build = root_dir.join("linux_bin");

    // Include paths matching kernel build system
    let inc_dir = linux_src.join("include");
    let inc_generated = linux_build.join("include");
    let inc_arch_src = linux_src.join("arch").join("x86").join("include");
    let inc_arch = linux_build.join("arch").join("x86").join("include");
    let inc_arch_generated = linux_build
        .join("arch")
        .join("x86")
        .join("include")
        .join("generated");

    // UAPI include paths (needed for asm/types.h, etc.)
    let inc_uapi = linux_src.join("include").join("uapi");
    let inc_generated_uapi = linux_build.join("include").join("generated").join("uapi");
    let inc_arch_uapi = linux_src
        .join("arch")
        .join("x86")
        .join("include")
        .join("uapi");
    let inc_arch_generated_uapi = linux_build
        .join("arch")
        .join("x86")
        .join("include")
        .join("generated")
        .join("uapi");

    // Match kernel's bindgen configuration from linux/rust/Makefile
    // BINDGEN_TARGET_x86 := x86_64-linux-gnu (note: NOT x86_64-unknown-linux-gnu)
    // The kernel uses the GNU target triple without "unknown"
    let target = "x86_64-linux-gnu";

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(manifest_dir.join("include/wrapper.h").to_str().unwrap())
        // Match kernel: --target=$(BINDGEN_TARGET)
        .clang_arg(format!("--target={}", target))
        // Prevent use of system headers
        .clang_arg("-nostdinc")
        // Include paths in the order the kernel uses them
        .clang_arg(format!("-I{}", inc_dir.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_generated.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_arch_src.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_arch.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_arch_generated.to_string_lossy()))
        // UAPI include paths for asm/types.h, etc.
        .clang_arg(format!("-I{}", inc_uapi.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_generated_uapi.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_arch_uapi.to_string_lossy()))
        .clang_arg(format!("-I{}", inc_arch_generated_uapi.to_string_lossy()))
        // Match kernel: --use-core --with-derive-default --ctypes-prefix ffi
        .use_core()
        .derive_default(true)
        .ctypes_prefix("ffi")
        // Match kernel: --no-layout-tests --no-debug '.*'
        .layout_tests(false)
        .generate_comments(false)
        // Match kernel: --enable-function-attribute-detection
        .clang_arg("-fno-builtin")
        .clang_arg("-D__BINDGEN__")
        .clang_arg("-D__KERNEL__")
        .clang_arg("-DMODULE")
        // Include generated kernel config (defines CONFIG_* macros)
        .clang_arg("-include")
        .clang_arg(format!(
            "{}/include/generated/autoconf.h",
            linux_build.to_string_lossy()
        ))
        .allowlist_recursively(true)
        // fs related items
        .allowlist_item("folio_.*")
        .allowlist_item("BINDINGS_.*")
        .allowlist_item("fs_context_.*")
        .allowlist_item("get_tree_.*")
        .allowlist_item("generic_.*")
        .allowlist_item("page_get_link|init_special_inode|inode_nohighmem|set_nlink")
        .allowlist_item(".*_inode")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write bindings to OUT_DIR directory
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let binding_file = "bindings.rs";
    bindings
        .write_to_file(out_path.join(binding_file))
        .expect("Couldn't write bindings!");
}
