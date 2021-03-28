build:
	cargo build --target x86_64-unknown-linux-gnu --release
	rm -f kernel.elf
	mv target/x86_64-unknown-linux-gnu/release/kernel kernel.elf