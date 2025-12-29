#!/bin/bash
qemu-system-x86_64 \
    -kernel ./linux_bin/arch/x86_64/boot/bzImage \
    -drive file=./build/img/debian.img,format=raw,media=disk \
    -append "root=/dev/sda rw console=ttyS0 ip=10.0.2.15::10.0.2.1:255.255.255.0" \
    -device e1000,netdev=eth0 -netdev user,id=eth0 \
    -m 1024 \
    -smp 2 \
    -nographic

# -object "filter-dump,id=myeth0,netdev=myeth0,file=build/dump.dat" \