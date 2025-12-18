# Use the rofs kmod
Install mod
```sh
insmod /lib/modules/6.18.0/rinux_rofs.ko 
lsmod
rmmod rinux_rofs
```
Use fs
```sh
# mount the fs
mkdir -p /mnt/rofs
mount -t rust-fs none /mnt/rofs

# inspect content
cd /mnt/rofs
ls -la
cat test.txt    # Should display "hello\n"
cat link.txt    # Should display "./test.txt"

# cleanup
umount /mnt/rofs
```