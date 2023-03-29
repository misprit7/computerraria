#!/usr/bin/zsh

# Helper script to copy file to txt file format WireHead expects
# Example: ./copy_bin.sh /tmp/in.txt

# Rebuild project
cargo build --release

# Convert from elf to bin
# Fancy glob matching! https://zsh.sourceforge.io/Doc/Release/Expansion.html#Glob-Qualifiers
# If you're not zsh then sucks to suck, just hardcode your project name
rust-objcopy -O binary ./target/riscv32i-unknown-none-elf/release/*(x.) /tmp/rust.bin 

# Convert from bin to txt
hexdump -ve '1/1 "%.2x "' /tmp/rust.bin | head -c -1 > $1

