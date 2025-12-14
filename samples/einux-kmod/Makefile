# SPDX-License-Identifier: GPL-2.0

# Paths - using relative paths to kernel build directory
KERNEL_BUILD := MUST_OVERRIDE
KERNEL_SOURCE := MUST_OVERRIDE

# Build output directory
BUILD_DIR := $(abspath ../../build/samples/einux-kmod/build)
SRC_DIR := $(CURDIR)
WORKSPACE_DIR := $(abspath ../..)

# Module and dependency names
MODULE_NAME := einux_kmod
EINUX_VEC := einux_vec

# Output files
MODULE_OBJ := $(BUILD_DIR)/$(MODULE_NAME)_main.o
EINUX_VEC_OBJ := $(BUILD_DIR)/$(EINUX_VEC).o
KO_FILE := $(BUILD_DIR)/$(MODULE_NAME).ko

# Cargo paths
CARGO := cargo
CARGO_TARGET_DIR := $(WORKSPACE_DIR)/target

.PHONY: all clean modules

all: $(KO_FILE)

# Step 1: Build einux-vec using cargo
$(EINUX_VEC_OBJ): $(WORKSPACE_DIR)/samples/einux-vec/src/lib.rs
	@mkdir -p $(BUILD_DIR)
	@echo "Building einux-vec with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p einux-vec --release -- \
		--emit=obj=$(EINUX_VEC_OBJ) -C extra-filename=

# Step 2: Build einux-kmod using cargo (depends on einux-vec)
$(MODULE_OBJ): $(SRC_DIR)/src/lib.rs $(EINUX_VEC_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Building einux-kmod with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p einux-kmod --release -- \
		--emit=obj=$(MODULE_OBJ) -C extra-filename=

# Step 3: Use KBuild to link all objects into a kernel module
$(KO_FILE): $(MODULE_OBJ) $(EINUX_VEC_OBJ)
	@echo "Linking kernel module with KBuild: $@"
	@$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 modules

clean:
	@if [ -d "$(BUILD_DIR)" ]; then \
		$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 clean; \
	fi
	rm -rf $(BUILD_DIR)

help:
	@echo "Einux-kmod cargo-based kernel module build"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build the kernel module ($(KO_FILE))"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "This Makefile uses cargo to build Rust object files and"
	@echo "KBuild to link them into a kernel module."