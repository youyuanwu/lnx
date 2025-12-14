#![no_std]

// Re-export the pre-built kernel crate
// The actual kernel crate is provided via --extern kernel in .cargo/config.toml
pub use kernel::*;
