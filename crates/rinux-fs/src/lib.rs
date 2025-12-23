#![no_std]

pub mod folio;

pub mod fs;

pub use kernel::macros;

pub(crate) mod ffi;

pub enum Either<L, R> {
    /// Left value.
    Left(L),

    /// Right value.
    Right(R),
}
