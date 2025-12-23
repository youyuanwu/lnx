//! Rust helper bindings for kernel inline functions
//!
//! This crate provides Rust bindings to kernel inline functions by wrapping
//! them in C helper functions that can be linked.

#![no_std]
#![allow(clippy::missing_safety_doc)]

pub mod fs;

pub mod bindings;
