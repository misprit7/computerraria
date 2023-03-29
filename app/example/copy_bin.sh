#!/usr/bin/sh

# Rebuild project
cargo build --release

# Convert from elf to bin
rust-objcopy -O binary ./target/riscv32i-unknown-none-elf/release/example /tmp/rust.bin 

# Convert from bin to txt
hexdump -ve '1/1 "%.2x "' /tmp/rust.bin | head -c -1 > $1

