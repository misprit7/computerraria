#!/usr/bin/sh

# Script to show assembly of rust target
cargo objdump --target riscv32i-unknown-none-elf --bin template -- -d

