#!/bin/sh -ex

cargo build --verbose --release
cargo objcopy --verbose --release -- -O binary badrandom.bin
uf2conv badrandom.bin --base 0x10000000 --family 0xe48bff56 --output badrandom.uf2
cp badrandom.uf2 $1
