#!/bin/sh

cd $1

cargo build --release --target thumbv6m-none-eabi
cargo objcopy --release --target thumbv6m-none-eabi -- -O binary badrandom.bin
uf2conv badrandom.bin --base 0x10000000 --family 0xe48bff56 --output badrandom.uf2
