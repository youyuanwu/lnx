# SPDX-License-Identifier: GPL-2.0

# Paths - using relative paths to kernel build directory
KERNEL_BUILD := MUST_OVERRIDE
KERNEL_SOURCE := MUST_OVERRIDE

# Include common Rust compilation settings
include ../../../cmake/rust.make

# Build output directory
BUILD_DIR := $(abspath ../../../build/external/kmods/rust_out_of_tree3)
SRC_DIR := $(CURDIR)

# Source and output files
RUST_SOURCE := src/rust_out_of_tree3_mod.rs
LIB_VEC_SOURCE := src/lib_vec.rs
MODULE_NAME := rust_out_of_tree3
LIB_VEC_NAME := lib_vec
OBJ_FILE := $(BUILD_DIR)/$(MODULE_NAME)_mod.o
LIB_VEC_OBJ := $(BUILD_DIR)/$(LIB_VEC_NAME).o
KO_FILE := $(BUILD_DIR)/$(MODULE_NAME).ko

.PHONY: all clean modules

all: $(KO_FILE)

# Step 0: Compile lib_vec as a separate library
$(LIB_VEC_OBJ): $(LIB_VEC_SOURCE)
	@mkdir -p $(BUILD_DIR)
	@echo "Compiling lib_vec library: $<"
	RUSTC_BOOTSTRAP=1 $(RUSTC) $(RUSTC_FLAGS) $(RUST_CODEGEN_FLAGS) $(RUST_TARGET_FLAGS) \
		$(RUST_FEATURE_FLAGS) $(RUST_EXTERN_FLAGS) $(RUST_KERNEL_LIB_PATH) \
		--crate-name $(LIB_VEC_NAME) \
		--sysroot=/dev/null \
		--out-dir $(BUILD_DIR) \
		--emit=obj=$@,metadata=$(BUILD_DIR)/lib$(LIB_VEC_NAME).rmeta \
		$<

# Step 1: Compile Rust source to object file using rustc (with lib_vec dependency)
$(OBJ_FILE): $(RUST_SOURCE) $(LIB_VEC_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Compiling Rust module: $<"
	RUST_MODFILE=$(BUILD_DIR)/$(MODULE_NAME) RUSTC_BOOTSTRAP=1 $(RUSTC) $(RUSTC_FLAGS) $(RUST_CODEGEN_FLAGS) $(RUST_TARGET_FLAGS) \
		$(RUST_FEATURE_FLAGS) $(RUST_EXTERN_FLAGS) $(RUST_KERNEL_LIB_PATH) \
		--extern $(LIB_VEC_NAME)=$(BUILD_DIR)/lib$(LIB_VEC_NAME).rmeta \
		-L $(BUILD_DIR) \
		--crate-name $(MODULE_NAME) \
		--sysroot=/dev/null \
		--out-dir $(BUILD_DIR) \
		--emit=obj=$@ \
		$<

# Step 2: Use KBuild to link the Rust object into a kernel module
$(KO_FILE): $(OBJ_FILE)
	@echo "Building kernel module with KBuild:$@"
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
