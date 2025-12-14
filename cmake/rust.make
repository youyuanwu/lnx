# SPDX-License-Identifier: GPL-2.0
# Common Rust kernel module compilation settings

# Check required variables
ifndef KERNEL_BUILD
$(error KERNEL_BUILD is not set. Please set it to the kernel build directory.)
endif

# Kernel paths derived from KERNEL_BUILD
KERNEL_RUST := $(KERNEL_BUILD)/rust
KERNEL_RUSTC_CFG := $(KERNEL_BUILD)/include/generated/rustc_cfg

# Use rustc - MUST match kernel build version
# Rust .rmeta files are version-specific and incompatible across versions
RUSTC := rustc

# Rustc flags for kernel module compilation (stable features only)
RUSTC_FLAGS := --edition=2021 \
	-Dunsafe_op_in_unsafe_fn

# Code generation flags
RUST_CODEGEN_FLAGS := -Cpanic=abort \
	-Cembed-bitcode=n \
	-Clto=n \
	-Cforce-unwind-tables=n \
	-Ccodegen-units=1 \
	-Csymbol-mangling-version=v0 \
	-Crelocation-model=static

# Target and optimization flags
RUST_TARGET_FLAGS := --target=$(KERNEL_BUILD)/scripts/target.json \
	-Ctarget-feature=-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2 \
	-Zcf-protection=branch \
	-Zno-jump-tables \
	-Ctarget-cpu=x86-64 \
	-Cno-redzone=y \
	-Ccode-model=kernel \
	-Zfunction-return=thunk-extern \
	-Zpatchable-function-entry=16,16 \
	-Copt-level=2 \
	-Cdebug-assertions=n \
	-Coverflow-checks=y

# Feature flags (only unstable features that need explicit enabling)
# Note: asm_goto is stable since Rust 1.87.0 and removed for Rust 1.91+
RUST_FEATURE_FLAGS := --cfg MODULE \
	@$(KERNEL_RUSTC_CFG) \
	-Zallow-features=arbitrary_self_types,used_with_arg \
	-Zcrate-attr=no_std \
	-Zcrate-attr='feature(arbitrary_self_types,used_with_arg)' \
	-Zunstable-options

# Extern libraries and crate options
RUST_EXTERN_FLAGS := --extern pin_init \
	--extern kernel \
	--crate-type rlib

# Library search path
RUST_KERNEL_LIB_PATH := -L $(KERNEL_RUST)
