#![allow(
    unsafe_op_in_unsafe_fn,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    improper_ctypes,
    dead_code,
    unnecessary_transmutes,
    clippy::all
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const SLAB_RECLAIM_ACCOUNT: slab_flags_t = BINDINGS_SLAB_RECLAIM_ACCOUNT;
pub const SLAB_ACCOUNT: slab_flags_t = BINDINGS_SLAB_ACCOUNT;
pub const MAX_LFS_FILESIZE: loff_t = BINDINGS_MAX_LFS_FILESIZE;
