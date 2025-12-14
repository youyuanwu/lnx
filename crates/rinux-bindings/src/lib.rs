//! Stub bindings crate for IDE support
//!
//! This crate provides a minimal stub that re-exports the pre-built kernel bindings.
//! For actual kernel module compilation, use the Makefile which links against
//! the pre-built bindings at build/linux_bin/rust/libbindings.rmeta

#![no_std]

// Re-export the pre-built bindings crate
// The actual bindings are provided via --extern bindings in .cargo/config.toml
pub use bindings::*;
