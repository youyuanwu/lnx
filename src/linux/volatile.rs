#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![no_std]

use core::ptr;

/// A wrapper type around a volatile variable, which allows for volatile reads and writes
/// to the contained value. The stored type needs to be `Copy`, as volatile reads and writes
/// take and return copies of the value.
///
/// The size of this struct is the same as the size of the contained type.
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Volatile<T: Copy>(T);

impl<T: Copy> Volatile<T> {
    /// Construct a new volatile instance wrapping the given value.
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let value = Volatile::new(0u32);
    /// ```
    ///
    /// ```rust
    /// use volatile::Volatile;
    /// let mut_ptr = 0xFEE00000 as *mut u32;
    /// let volatile_ptr = mut_ptr as *mut Volatile<u32>;
    /// ```
    ///
    /// # Panics
    ///
    /// This method never panics.
    #[cfg(feature = "const_fn")]
    pub const fn new(value: T) -> Volatile<T> {
        Volatile(value)
    }

    /// Construct a new volatile instance wrapping the given value.
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let value = Volatile::new(0u32);
    /// ```
    ///
    /// # Panics
    ///
    /// This method never panics.
    #[cfg(not(feature = "const_fn"))]
    pub fn new(value: T) -> Volatile<T> {
        Volatile(value)
    }

    /// Performs a volatile read of the contained value, returning a copy
    /// of the read value. Volatile reads are guaranteed not to be optimized
    /// away by the compiler, but by themselves do not have atomic ordering
    /// guarantees. To also get atomicity, consider looking at the `Atomic` wrapper type.
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let value = Volatile::new(42u32);
    ///
    /// assert_eq!(value.read(), 42u32);
    /// ```
    ///
    /// # Panics
    ///
    /// This method never panics.
    pub fn read(&self) -> T {
        // UNSAFE: Safe, as we know that our internal value exists.
        unsafe { ptr::read_volatile(&self.0) }
    }

    /// Performs a volatile write, setting the contained value to the given value `value`. Volatile
    /// writes are guaranteed to not be optimized away by the compiler, but by themselves do not
    /// have atomic ordering guarantees. To also get atomicity, consider looking at the `Atomic`
    /// wrapper type.
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let mut value = Volatile::new(0u32);
    ///
    /// value.write(42u32);
    ///
    /// assert_eq!(value.read(), 42u32);
    /// ```
    ///
    /// # Panics
    ///
    /// This method never panics.
    pub fn write(&mut self, value: T) {
        // UNSAFE: Safe, as we know that our internal value exists.
        unsafe { ptr::write_volatile(&mut self.0, value) };
        /*
        use kernel::bindings;
        unsafe { bindings::writel(value, &mut self.0 as *mut u32 as *mut _) };
        */
    }

    /// Performs a volatile read of the contained value, passes a mutable reference to it to the
    /// function `f`, and then performs a volatile write of the (potentially updated) value back to
    /// the contained value.
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let mut value = Volatile::new(21u32);
    ///
    /// value.update(|val_ref| *val_ref *= 2);
    ///
    /// assert_eq!(value.read(), 42u32);
    /// ```
    ///
    /// # Panics
    ///
    /// Ths method never panics.
    pub fn update<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut value = self.read();
        f(&mut value);
        self.write(value);
    }
}

impl<T: Copy> Clone for Volatile<T> {
    fn clone(&self) -> Self {
        Volatile(self.read())
    }
}

/// A volatile wrapper which allows both read and write operations;
/// functionally equivalent to the `Volatile` type, as it is a type
/// alias for it.
///
/// The size of this struct is the same as the contained type.
pub type ReadWrite<T> = Volatile<T>;
