# rust-pico-badrandom

What it says on the tin.
Set Raspberry Pi Pico's gpio 15 from a [16-bit LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Fibonacci_LFSRs).
For testing a randomness testing setup.

## Build

Prep:

```console
apt install gcc-arm-none-eabi gdb-multiarch
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install uf2conv cargo-binutils
```

UF2 for flashing:
```console
cargo build --release
cargo objcopy --release -- -O binary badrandom.bin
uf2conv badrandom.bin --base 0x10000000 --family 0xe48bff56 --output badrandom.uf2
```

## Debug

Bridge to picoprobe with
[openocd](https://github.com/raspberrypi/openocd):
```console
openocd -f interface/picoprobe.cfg -f target/rp2040.cfg  # add e.g. "-c 'bindto 0.0.0.0'" for remote access
```

Build an ELF file:
```console
cargo build
cargo objcopy -- -O elf32-littlearm badrandom.elf
```

In `gdb-multiarch badrandom.elf`:
```
target extended-remote localhost:3333
load
monitor reset init  # still halted here
continue            # run the program
```
