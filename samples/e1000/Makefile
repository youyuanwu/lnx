# SPDX-License-Identifier: GPL-2.0

# Paths - using relative paths to kernel build directory
KERNEL_BUILD := MUST_OVERRIDE

# Build output directory
BUILD_DIR := $(abspath ../../build/samples/e1000/build)
SRC_DIR := $(CURDIR)
WORKSPACE_DIR := $(abspath ../..)

# Module name
MODULE_NAME := e1000_for_linux

# Output files
MODULE_OBJ := $(BUILD_DIR)/$(MODULE_NAME)_main.o
KO_FILE := $(BUILD_DIR)/$(MODULE_NAME).ko

# Cargo paths
CARGO := cargo
CARGO_TARGET_DIR := $(WORKSPACE_DIR)/target

.PHONY: all clean modules

all: $(KO_FILE)

# Step 1: Build e1000 crate using cargo
$(MODULE_OBJ): $(SRC_DIR)/src/e1000_for_linux.rs
	@mkdir -p $(BUILD_DIR)
	@echo "Building e1000 with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p e1000 --release -- \
		--emit=obj=$(MODULE_OBJ) -C extra-filename=

# Step 2: Use KBuild to link the object into a kernel module
$(KO_FILE): $(MODULE_OBJ)
	@echo "Linking kernel module with KBuild: $@"
	@$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 modules

clean:
	@if [ -d "$(BUILD_DIR)" ]; then \
		$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 clean; \
	fi
	rm -rf $(BUILD_DIR)

help:
	@echo "E1000 cargo-based kernel module build"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build the kernel module ($(KO_FILE))"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "This Makefile uses cargo to build the Rust object file and"
	@echo "KBuild to link it into a kernel module."
