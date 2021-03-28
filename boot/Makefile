build:
	cargo build -Zbuild-std --target x86_64-unknown-uefi
	rm -f boot.efi
	mv target/x86_64-unknown-uefi/debug/boot.efi boot.efi