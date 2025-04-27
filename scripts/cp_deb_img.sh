#!/bin/bash
mkdir -p ./build/debianmnt
sudo mount -o loop ./build/img/debian.img ./build/debianmnt
# init the mod dir
sudo mkdir -p ./build/debianmnt/lib/modules
# cp content to img
sudo cp -r ./build/debian-rootfs/* ./build/debianmnt
# unmount 
sudo umount ./build/debianmnt