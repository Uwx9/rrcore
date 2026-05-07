TARGET := riscv64gc-unknown-none-elf
MODE := release
TARGET_DIR := target/$(TARGET)/$(MODE)

elf:
	cargo build --release

binary: elf
	rust-objcopy --strip-all $(TARGET_DIR)/rrcore -O binary $(TARGET_DIR)/rrcore.bin

build: binary

run:
	qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin \
	-device loader,file=$(TARGET_DIR)/rrcore.bin,addr=0x80200000 

debug:
	qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin \
	-device loader,file=$(TARGET_DIR)/rrcore.bin,addr=0x80200000 \
	-s -S
