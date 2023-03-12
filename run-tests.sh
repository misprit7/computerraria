#!/bin/sh

#################################################
# Runs all riscof tests
#################################################

cd test
# Why all this junk isn't specified in the config is weird, hate having to make little wrapper scripts like this
riscof run --config=config.ini --suite=riscv-arch-test/riscv-test-suite/ --env=riscv-arch-test/riscv-test-suite/env
