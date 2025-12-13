# SPDX-License-Identifier: GPL-2.0

# Use stable rustc (1.86.0) - MUST match kernel build version
# Rust .rmeta files are version-specific and incompatible across versions
RUSTC := rustc +stable

# Paths - using relative paths to kernel build directory
KERNEL_BUILD := MUST_OVERRIDE
KERNEL_SOURCE := MUST_OVERRIDE
KERNEL_RUST := $(KERNEL_BUILD)/rust
KERNEL_SCRIPTS := $(KERNEL_BUILD)/scripts
KERNEL_OBJTOOL := $(KERNEL_BUILD)/tools/objtool/objtool
KERNEL_RUSTC_CFG := $(KERNEL_BUILD)/include/generated/rustc_cfg
KERNEL_MODULE_LDS := $(KERNEL_SCRIPTS)/module.lds

# Build output directory
BUILD_DIR := $(abspath ../../../build/external/kmods/rust_out_of_tree3)
SRC_DIR := $(CURDIR)

# Source and output files
RUST_SOURCE := rust_out_of_tree3.rs
LIB_VEC_SOURCE := lib_vec.rs
MODULE_NAME := rust_out_of_tree3
LIB_VEC_NAME := lib_vec
OBJ_FILE := $(BUILD_DIR)/$(MODULE_NAME)_prebuilt.o
LIB_VEC_OBJ := $(BUILD_DIR)/$(LIB_VEC_NAME).o
MOD_O_FILE := $(BUILD_DIR)/$(MODULE_NAME).mod.o
MOD_C_FILE := $(BUILD_DIR)/$(MODULE_NAME).mod.c
KO_FILE := $(BUILD_DIR)/$(MODULE_NAME).ko

# Rustc flags for kernel module compilation (stable features only)
RUSTC_FLAGS := --edition=2021 \
	-Dunsafe_op_in_unsafe_fn

# Code generation flags
CODEGEN_FLAGS := -Cpanic=abort \
	-Cembed-bitcode=n \
	-Clto=n \
	-Cforce-unwind-tables=n \
	-Ccodegen-units=1 \
	-Csymbol-mangling-version=v0 \
	-Crelocation-model=static

# Target and optimization flags
TARGET_FLAGS := --target=$(KERNEL_BUILD)/scripts/target.json \
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
FEATURE_FLAGS := --cfg MODULE \
	@$(KERNEL_RUSTC_CFG) \
	-Zallow-features=asm_goto,arbitrary_self_types,used_with_arg \
	-Zcrate-attr=no_std \
	-Zcrate-attr='feature(asm_goto,arbitrary_self_types,used_with_arg)' \
	-Zunstable-options

# Extern libraries and crate options
EXTERN_FLAGS := --extern pin_init \
	--extern kernel \
	--crate-type rlib

# Library search path
LIB_PATH := -L $(KERNEL_RUST)

# Objtool flags for module
OBJTOOL_FLAGS := --hacks=jump_label \
	--hacks=noinstr \
	--hacks=skylake \
	--ibt \
	--orc \
	--retpoline \
	--rethunk \
	--static-call \
	--uaccess \
	--prefix=16 \
	--link \
	--module

.PHONY: all clean modules

all: $(KO_FILE)

# Step 0: Compile lib_vec as a separate library
$(LIB_VEC_OBJ): $(LIB_VEC_SOURCE)
	@mkdir -p $(BUILD_DIR)
	@echo "Compiling lib_vec library: $<"
	RUSTC_BOOTSTRAP=1 $(RUSTC) $(RUSTC_FLAGS) $(CODEGEN_FLAGS) $(TARGET_FLAGS) \
		$(FEATURE_FLAGS) $(EXTERN_FLAGS) $(LIB_PATH) \
		--crate-name $(LIB_VEC_NAME) \
		--sysroot=/dev/null \
		--out-dir $(BUILD_DIR) \
		--emit=obj=$@,metadata=$(BUILD_DIR)/lib$(LIB_VEC_NAME).rmeta \
		$<

# Step 1: Compile Rust source to object file using rustc (with lib_vec dependency)
$(OBJ_FILE): $(RUST_SOURCE) $(LIB_VEC_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Compiling Rust module: $<"
	RUST_MODFILE=$(BUILD_DIR)/$(MODULE_NAME) RUSTC_BOOTSTRAP=1 $(RUSTC) $(RUSTC_FLAGS) $(CODEGEN_FLAGS) $(TARGET_FLAGS) \
		$(FEATURE_FLAGS) $(EXTERN_FLAGS) $(LIB_PATH) \
		--extern $(LIB_VEC_NAME)=$(BUILD_DIR)/lib$(LIB_VEC_NAME).rmeta \
		-L $(BUILD_DIR) \
		--crate-name $(MODULE_NAME) \
		--sysroot=/dev/null \
		--out-dir $(BUILD_DIR) \
		--emit=obj=$@ \
		$<

# Step 2: Use KBuild to link the Rust object into a kernel module
$(KO_FILE): $(OBJ_FILE)
	@echo "Building kernel module with KBuild: $@"
	@$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 modules

clean:
	@if [ -d "$(BUILD_DIR)" ]; then \
		$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 clean; \
	fi
	rm -rf $(BUILD_DIR)

help:
	@echo "Rust out-of-tree kernel module build"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build the kernel module ($(KO_FILE))"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "This Makefile directly invokes rustc and kernel scripts to build"
	@echo "a Rust kernel module without using the full kernel build system."
