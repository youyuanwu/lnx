#!/bin/bash
qemu-system-x86_64 \
    -kernel ../linux/arch/x86_64/boot/bzImage \
    -append "root=/dev/sda rw console=ttyS0" \
    -drive file=./build/img/debian.img,format=raw,media=disk \
    -m 1024 \
    -smp 2 \
    -nographic