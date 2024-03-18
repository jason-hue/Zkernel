# RISCV64 Kernel

This project is a kernel designed to run on the RISCV64 platform.

## Usage

To use this project, follow the instructions in the Makefile:

```makefile
# Define variables
TARGET_NAME := os
TARGET_BIN := os.bin

# Default target
all: build

# Compile the Release version
build:
    cd os && cargo build --release && cd ..

# Use rust-objcopy to generate the binary file
bin: build
    rust-objcopy --strip-all os/target/riscv64gc-unknown-none-elf/release/$(TARGET_NAME) -O binary os/target/riscv64gc-unknown-none-elf/release/$(TARGET_BIN)

# Run on qemu
run: bin
    qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios bootloader/rustsbi-qemu.bin \
        -device loader,file=os/target/riscv64gc-unknown-none-elf/release/$(TARGET_BIN),addr=0x80200000

# Start gdb server
gdbserver: bin
    qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios bootloader/rustsbi-qemu.bin \
        -device loader,file=os/target/riscv64gc-unknown-none-elf/release/$(TARGET_BIN),addr=0x80200000 \
        -s -S

# Start a GDB client connecting to Qemu
gdbclient:
    ~/path/to/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14/bin/riscv64-unknown-elf-gdb \
        -ex 'file os/target/riscv64gc-unknown-none-elf/release/os' \
        -ex 'set arch riscv:rv64' \
        -ex 'target remote localhost:1234'

# Clean generated files
clean:
    cargo clean
