# Copied from https://github.com/Rust-for-Linux/rust-out-of-tree-module
# git hash: df508ea156314fe281cdaded07bcf89d22c3373a

include(${CMAKE_SOURCE_DIR}/cmake/KMod.cmake)
add_kmod(
  NAME rust_out_of_tree
  KDIR ${KDIR_ROOT}
)