# Install deps
QEMU KVM
```sh
sudo apt install qemu-kvm libvirt-daemon-system libvirt-clients bridge-utils virt-manager
# check virt ok
sudo kvm-ok
# check status
sudo systemctl status libvirtd
# add user permission
sudo usermod -aG libvirt $USER

qemu-img create -f qcow2 rootfs.img 10G
qemu-system-x86_64 -kernel /path/to/bzImage -append "root=/dev/sda console=ttyS0" -hda rootfs.img -m 1024 -smp 2 -nographic

qemu-system-x86_64 \
    -kernel ../linux/arch/x86_64/boot/bzImage \
    -append "root=/dev/sda console=ttyS0" \
    -hda ./build/img/debian.img \
    -m 1024 \
    -smp 2 \
    -nographic
```

* Create an empty disk img.
```sh
# create empty img
dd if=/dev/zero of=rootfs.img bs=1M count=100
mkfs.ext4 rootfs.img
# mount image
mkdir ./rootfs
sudo mount -o loop rootfs.img ./rootfs
sudo cp ../../rust_out_of_tree.ko ./rootfs
sudo umount ./rootfs
```

insmod /etc/modules-load.d/rust_out_of_tree.ko 
lsmod
rmmod rust_out_of_tree

shutdown now

* somethings not mounted.
mount -t proc proc /proc
mount | grep proc
mount -t sysfs sysfs /sys

# MISC

* Use debian file system
```sh
sudo apt-get install debootstrap
# create in curr dir.
sudo debootstrap --arch=amd64 stable debian-rootfs http://deb.debian.org/debian/

# Customize:
sudo chroot debian-rootfs
apt-get update
apt-get install <package_name>
exit

# create image
dd if=/dev/zero of=debian.img bs=1M count=1024
mkfs.ext4 debian.img

# mount
mkdir -p ./build/debianmnt
sudo mount -o loop ./build/img/debian.img ./build/debianmnt
# cp content to img
sudo cp -r ./build/debian-rootfs/* ./build/debianmnt
# unmount 
sudo umount ./build/debianmnt

```

ctrl+A , C, quit
```sh
# Still need pwd
udisksctl loop-setup -f build/tmp/debian.img
udisksctl mount -b /dev/loop0

# use dev
sudo losetup /dev/loop0 ./build/tmp/debian.img
sudo chmod 666 /dev/loop0  # Grants read/write permissions to all users
mount /dev/loop0 ./build/tmp/debianmnt/
```

```sh
# without mount
mcopy -i disk.img file_to_copy ::/destination_path
```