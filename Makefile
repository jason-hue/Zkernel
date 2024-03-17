# 定义变量
TARGET_NAME := os
TARGET_BIN := os.bin

# 默认目标
all: build

# 编译Release版本
build:
	cd os && cargo build --release && cd ..

# 使用rust-objcopy生成二进制文件
bin: build
	rust-objcopy --strip-all os/target/riscv64gc-unknown-none-elf/release/$(TARGET_NAME) -O binary os/target/riscv64gc-unknown-none-elf/release/$(TARGET_BIN)

# 在qemu运行
run: bin
	qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios bootloader/rustsbi-qemu.bin \
        -device loader,file=os/target/riscv64gc-unknown-none-elf/release/$(TARGET_BIN),addr=0x80200000

# 启动gdb服务
gdbserver: bin
	qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios bootloader/rustsbi-qemu.bin \
        -device loader,file=os/target/riscv64gc-unknown-none-elf/release/$(TARGET_BIN),addr=0x80200000 \
        -s -S

#启动一个 GDB 客户端连接到 Qemu
gdbclient:
	~/下载/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14/bin/riscv64-unknown-elf-gdb \
        -ex 'file os/target/riscv64gc-unknown-none-elf/release/os' \
        -ex 'set arch riscv:rv64' \
        -ex 'target remote localhost:1234'

# 清理生成的文件
clean:
	cargo clean
