task :boot do
    %x(
cd boot
cargo build -Zbuild-std
    )
end

task :kernel do
    %x(
cd kernel
cargo build
    )
end

task :mk_disk => [:boot, :kernel] do
    %x(
qemu-img create -f raw disk.img 200M
mkfs.fat -n 'Mikan OS' -s 2 -f 2 -R 32 -F 32 disk.img
mkdir -p mnt
sudo mount -o loop disk.img mnt
sudo mkdir -p mnt/EFI/BOOT
sudo cp target/x86_64-unknown-uefi/debug/boot.efi mnt/EFI/BOOT/BOOTX64.EFI
sudo cp target/x86_64-unknown-linux-gnu/debug/kernel mnt/kernel.elf
sudo umount mnt
    )
end

task :run_qemu do
    %x(
qemu-system-x86_64 \
-drive if=pflash,format=raw,file=OVMF.fd \
-hda disk.img \
-monitor stdio
    )
end