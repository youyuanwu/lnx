cmake_minimum_required(VERSION 3.20)
project(lnx LANGUAGES)

set(KDIR_ROOT ${CMAKE_SOURCE_DIR}/linux)
set(KBIN_ROOT ${CMAKE_SOURCE_DIR}/linux_bin)

# Check if SCCACHE_GHA_ENABLED envariable is set
if(DEFINED ENV{SCCACHE_GHA_ENABLED})
  message(STATUS "SCCACHE_GHA_ENABLED is set to: $ENV{SCCACHE_GHA_ENABLED}")
  set(K_CC 'sccache clang')
else()
  set(K_CC 'clang')
endif()



# Check that the kernel source directory exists
# Manually create a symlink: 
# ln -s ../linux linux
# ln -s ../linux_bin linux_bin
# Or manually unlink: 
# unlink ./linux
if(NOT EXISTS ${KDIR_ROOT}/Makefile)
  message(FATAL_ERROR "Kernel source directory not found: ${KDIR_ROOT}")
endif()

add_custom_target(rs_analyzer_all)
add_custom_target(clean_all)
add_custom_target(fmt_all)
add_subdirectory(external/kmods)
add_subdirectory(samples/e1000)

# copy stuff in image
# mount
# mkdir -p ./build/debianmnt
# sudo mount -o loop ./build/img/debian.img ./build/debianmnt
# # cp content to img
# sudo cp -r ./build/debian-rootfs/* ./build/debianmnt
# # unmount 
# sudo umount ./build/debianmnt

add_custom_target(print_download_deb_commands
  COMMAND echo "Download deb files: ./scripts/download_deb.sh"
  COMMAND echo "Copy deb into image: ./scripts/cp_deb_img.sh"
)

add_custom_target(build_empty_image
  COMMAND mkdir -p img
  COMMAND dd if=/dev/zero of=img/debian.img bs=1M count=1024
  COMMAND mkfs.ext4 img/debian.img
  COMMAND chmod a+w img/debian.img 
  WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
)

add_custom_target(debfs_ls
  COMMAND debugfs -w -f ../scripts/ls.txt img/debian.img
  WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
)

add_custom_target(debfs_cp_ko
  COMMAND debugfs -w -f ../scripts/cp_ko.txt img/debian.img
  WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
)

add_custom_target(configure_linux
  # Start from tinyconfig (minimal kernel) and enable only what QEMU tests need
  COMMAND $(MAKE) LLVM=1 tinyconfig O=${KBIN_ROOT}
  # 64-bit, ELF, modules, Rust
  COMMAND scripts/config --set-val CONFIG_64BIT y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_MODULES y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_MODULE_UNLOAD y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_RUST y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_LTO_NONE y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_BINFMT_ELF y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_BINFMT_SCRIPT y --file ${KBIN_ROOT}/.config
  # Serial console for QEMU -nographic
  COMMAND scripts/config --set-val CONFIG_TTY y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_SERIAL_8250 y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_SERIAL_8250_CONSOLE y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_PRINTK y --file ${KBIN_ROOT}/.config
  # Filesystems needed by init / test scripts
  COMMAND scripts/config --set-val CONFIG_PROC_FS y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_SYSFS y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_DEVTMPFS y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_DEVTMPFS_MOUNT y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_TMPFS y --file ${KBIN_ROOT}/.config
  # Block layer (needed for initramfs / general kernel infra)
  COMMAND scripts/config --set-val CONFIG_BLOCK y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_BLK_DEV_INITRD y --file ${KBIN_ROOT}/.config
  # Loop device (needed for mounting filesystem images in QEMU tests)
  COMMAND scripts/config --set-val CONFIG_BLK_DEV_LOOP y --file ${KBIN_ROOT}/.config
  # Minimal hardware for QEMU x86_64
  COMMAND scripts/config --set-val CONFIG_PCI y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_ATA y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_ATA_PIIX y --file ${KBIN_ROOT}/.config
  # ACPI needed for QEMU power off
  COMMAND scripts/config --set-val CONFIG_PM y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_ACPI y --file ${KBIN_ROOT}/.config
  # Busybox needs these
  COMMAND scripts/config --set-val CONFIG_FUTEX y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_EPOLL y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_SIGNALFD y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_TIMERFD y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_EVENTFD y --file ${KBIN_ROOT}/.config
  # SMP required for standard spinlock / mutex types used by bindings
  COMMAND scripts/config --set-val CONFIG_SMP y --file ${KBIN_ROOT}/.config
  # Full SLUB allocator (tinyconfig enables SLUB_TINY which stubs out SLAB_RECLAIM_ACCOUNT)
  COMMAND scripts/config --set-val CONFIG_SLUB_TINY n --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_SLUB y --file ${KBIN_ROOT}/.config
  # Rust emits retpoline thunks and stack-protector calls; kernel must export them
  COMMAND scripts/config --set-val CONFIG_CPU_MITIGATIONS y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_STACKPROTECTOR y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_STACKPROTECTOR_STRONG y --file ${KBIN_ROOT}/.config
  # Networking support (always enabled for rko networking modules)
  COMMAND scripts/config --set-val CONFIG_NET y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_INET y --file ${KBIN_ROOT}/.config
  # KUnit test framework (for rko kunit_tests module)
  COMMAND scripts/config --set-val CONFIG_KUNIT y --file ${KBIN_ROOT}/.config
  # io_uring (for IORING_OP_URING_CMD custom command support)
  COMMAND scripts/config --set-val CONFIG_IO_URING y --file ${KBIN_ROOT}/.config
  # Symbolized stack traces for debugging
  COMMAND scripts/config --set-val CONFIG_KALLSYMS y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_KALLSYMS_ALL y --file ${KBIN_ROOT}/.config
  # Resolve dependencies and verify Rust
  COMMAND $(MAKE) LLVM=1 olddefconfig O=${KBIN_ROOT}
  COMMAND $(MAKE) LLVM=1 rustavailable
  WORKING_DIRECTORY ${KDIR_ROOT}
)

# disable net device to avoid driver conflicts.
add_custom_target(make_linux
  COMMAND $(MAKE) LLVM=1 O=${KBIN_ROOT} CC=${K_CC}
  WORKING_DIRECTORY ${KDIR_ROOT}
)

add_custom_target(clean_linux
  COMMAND $(MAKE) O=${KBIN_ROOT} clean
  WORKING_DIRECTORY ${KDIR_ROOT}
)

# note the "depmod" needs to be run before the auto mod loading works are reboot.