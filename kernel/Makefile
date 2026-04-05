build:
	cargo build --release && \
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/rrcore -O binary target/riscv64gc-unknown-none-elf/release/rrcore.bin

run:
	qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin \
	-device loader,file=target/riscv64gc-unknown-none-elf/release/rrcore.bin,addr=0x80200000 

debug:
	qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin \
	-device loader,file=target/riscv64gc-unknown-none-elf/release/rrcore.bin,addr=0x80200000 \
	-s -S
