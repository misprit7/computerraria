#!/usr/bin/sh

# Script to call binutils without having to remember a million cli options
# e.g. ./binutils.sh objdump -d or ./binutils.sh size
cargo $1 --release --target riscv32i-unknown-none-elf --bin game-of-life -- "${@:2}"

