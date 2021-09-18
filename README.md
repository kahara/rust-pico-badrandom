# rust-pico-badrandom

What it says on the tin. Set Raspberry Pi Pico's gpio 15 from a [16-bit LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Fibonacci_LFSRs).
For testing a randomness testing setup.

## Build

```console
rustup target add thumbv7m-none-eabi
cargo build --target thumbv7m-none-eabi
cargo objcopy -- -O elf32-littlearm study-rust-rp2040.elf
```
