# SPDX-License-Identifier: GPL-2.0

# Paths - using relative paths to kernel build directory
KERNEL_BUILD := MUST_OVERRIDE

# Build output directory
BUILD_DIR := $(abspath ../../build/samples/rinux-rofs/build)
SRC_DIR := $(CURDIR)
WORKSPACE_DIR := $(abspath ../..)

# Module and dependency names
MODULE_NAME := rinux_rofs
RINUX_FS := rinux_fs
RINUX_KERNEL := rinux_kernel
RINUX_BINDINGS := rinux_bindings
RINUX_HELPER := rinux_helper

# Output files
MODULE_OBJ := $(BUILD_DIR)/$(MODULE_NAME)_main.o
RINUX_FS_OBJ := $(BUILD_DIR)/$(RINUX_FS).o
RINUX_KERNEL_OBJ := $(BUILD_DIR)/$(RINUX_KERNEL).o
RINUX_BINDINGS_OBJ := $(BUILD_DIR)/$(RINUX_BINDINGS).o
RINUX_HELPER_OBJ := $(BUILD_DIR)/$(RINUX_HELPER).o
RINUX_HELPER_C_OBJ := $(BUILD_DIR)/helpers.o
KO_FILE := $(BUILD_DIR)/$(MODULE_NAME).ko

# Cargo paths
CARGO := cargo
CARGO_TARGET_DIR := $(WORKSPACE_DIR)/target

.PHONY: all clean modules

all: $(KO_FILE)

# Step 1: Build rinux-bindings using cargo
$(RINUX_BINDINGS_OBJ): $(WORKSPACE_DIR)/crates/rinux-bindings/src/lib.rs
	@mkdir -p $(BUILD_DIR)
	@echo "Building rinux-bindings with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p rinux-bindings --release -- \
		--emit=obj=$(RINUX_BINDINGS_OBJ) -C extra-filename=

# Step 2: Build rinux-helper using cargo and extract C helpers
$(RINUX_HELPER_OBJ) $(RINUX_HELPER_C_OBJ): $(WORKSPACE_DIR)/crates/rinux-helper/src/lib.rs $(RINUX_BINDINGS_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Building rinux-helper with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p rinux-helper --release -- \
		--emit=obj=$(RINUX_HELPER_OBJ) -C extra-filename=
	@echo "Extracting C helper objects from static library"
	@HELPER_LIB=$$(find $(CARGO_TARGET_DIR)/target/release/build -name "librinux_helpers.a" -path "*/rinux-helper-*/out/*" -type f -printf '%T@ %p\n' | sort -rn | head -n1 | cut -d' ' -f2); \
	if [ -n "$$HELPER_LIB" ] && [ -f "$$HELPER_LIB" ]; then \
		cd $(BUILD_DIR) && ar x $$HELPER_LIB helpers.o; \
	else \
		echo "Error: Could not find librinux_helpers.a in $(CARGO_TARGET_DIR)/target/release/build"; \
		exit 1; \
	fi

# Step 3: Build rinux-kernel using cargo
$(RINUX_KERNEL_OBJ): $(WORKSPACE_DIR)/crates/rinux-kernel/src/lib.rs $(RINUX_BINDINGS_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Building rinux-kernel with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p rinux-kernel --release -- \
		--emit=obj=$(RINUX_KERNEL_OBJ) -C extra-filename=

# Step 4: Build rinux-fs using cargo
$(RINUX_FS_OBJ): $(WORKSPACE_DIR)/crates/rinux-fs/src/lib.rs $(RINUX_BINDINGS_OBJ) $(RINUX_KERNEL_OBJ) $(RINUX_HELPER_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Building rinux-fs with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p rinux-fs --release -- \
		--emit=obj=$(RINUX_FS_OBJ) -C extra-filename=

# Step 5: Build rinux-rofs using cargo (depends on all above)
$(MODULE_OBJ): $(SRC_DIR)/src/lib.rs $(RINUX_FS_OBJ) $(RINUX_KERNEL_OBJ) $(RINUX_BINDINGS_OBJ) $(RINUX_HELPER_OBJ)
	@mkdir -p $(BUILD_DIR)
	@echo "Building rinux-rofs with cargo"
	cd $(WORKSPACE_DIR) && $(CARGO) rustc -p rinux-rofs --release -- \
		--emit=obj=$(MODULE_OBJ) -C extra-filename=

# Step 6: Use KBuild to link all objects into a kernel module
$(KO_FILE): $(MODULE_OBJ) $(RINUX_FS_OBJ) $(RINUX_KERNEL_OBJ) $(RINUX_BINDINGS_OBJ) $(RINUX_HELPER_OBJ) $(RINUX_HELPER_C_OBJ)
	@echo "Linking kernel module with KBuild:$@"
	@$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 modules

clean:
	@if [ -d "$(BUILD_DIR)" ]; then \
		$(MAKE) -C $(KERNEL_BUILD) M=$(SRC_DIR) MO=$(BUILD_DIR) LLVM=1 clean; \
	fi
	rm -rf $(BUILD_DIR)

help:
	@echo "Rinux-rofs cargo-based kernel module build"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build the kernel module ($(KO_FILE))"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "This Makefile uses cargo to build Rust object files and"
	@echo "KBuild to link them into a kernel module."
