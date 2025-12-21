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
add_subdirectory(samples/einux-kmod)
add_subdirectory(samples/rinux-rofs)

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
  COMMAND $(MAKE) LLVM=1 defconfig O=${KBIN_ROOT}
  COMMAND scripts/config --set-val CONFIG_RUST y --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_NETDEVICES n --file ${KBIN_ROOT}/.config
  COMMAND scripts/config --set-val CONFIG_LTO_NONE y --file ${KBIN_ROOT}/.config
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