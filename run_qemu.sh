cd boot; make; cd -;
cd kernel; make; cd -;

qemu-img create -f raw disk.img 200M
mkfs.fat -n 'Mikan OS' -s 2 -f 2 -R 32 -F 32 disk.img
mkdir -p mnt
sudo mount -o loop disk.img mnt
sudo mkdir -p mnt/EFI/BOOT
sudo cp boot/boot.efi mnt/EFI/BOOT/BOOTX64.EFI
sudo cp kernel/kernel.elf mnt/kernel.elf
sudo umount mnt

# https://gihyo.jp/admin/serial/01/ubuntu-recipe/0441
qemu-system-x86_64 \
    -drive if=pflash,format=raw,file=OVMF.fd \
    -hda disk.img \
    -monitor stdio
