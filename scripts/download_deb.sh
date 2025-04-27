#!/bin/bash

CMAKE_BINARY_DIR="./build"
sudo rm -rf ${CMAKE_BINARY_DIR}/debian-rootfs || exit 1
mkdir -p ${CMAKE_BINARY_DIR}/debian-rootfs || exit 1
sudo debootstrap --arch=amd64 stable ${CMAKE_BINARY_DIR}/debian-rootfs http://deb.debian.org/debian/ || exit 1
sudo sed -i 's/^root:[^:]*:/root::/' ${CMAKE_BINARY_DIR}/debian-rootfs/etc/shadow || exit 1