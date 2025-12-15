#![no_std]
// Vector creation utility for kernel modules

use kernel::prelude::*;

/// Creates a vector with integers from 0 to `num - 1`.
pub fn create_vec2(num: usize) -> Result<KVec<i32>> {
    let mut numbers = KVec::new();
    for i in 0..num {
        numbers.push(i as i32, GFP_KERNEL)?;
    }
    Ok(numbers)
}
