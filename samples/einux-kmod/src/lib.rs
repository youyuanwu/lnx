#![no_std]
// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

use einux_vec::create_vec2;

module! {
    type: RustOutOfTree,
    name: "einux-kmod",
    authors: ["Rust for Linux Contributors"],
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustOutOfTree {
    numbers: KVec<i32>,
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let numbers = create_vec2(5)?;

        Ok(RustOutOfTree { numbers })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
