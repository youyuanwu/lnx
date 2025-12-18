#![no_std]

pub mod ffi;

pub mod folio;

pub mod fs;

pub use kernel::macros;

pub enum Either<L, R> {
    /// Left value.
    Left(L),

    /// Right value.
    Right(R),
}
