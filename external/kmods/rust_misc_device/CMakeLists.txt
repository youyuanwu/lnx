# Copied from https://github.com/Rust-for-Linux/linux/blob/rust-next/samples/rust/rust_misc_device.rs
# git hash: 9d7a0577c9db35c4cc52db90bc415ea248446472

include(${CMAKE_SOURCE_DIR}/cmake/KMod.cmake)
add_kmod(
  NAME rust_misc_device
  KDIR ${KDIR_ROOT}
)