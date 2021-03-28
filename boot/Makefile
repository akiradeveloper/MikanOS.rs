build:
	rm -f boot.efi
	cargo build -Z build-std --target x86_64-unknown-uefi
	cp target/x86_64-unknown-uefi/debug/boot.efi ./boot.efi