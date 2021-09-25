FROM ubuntu:20.04

ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTFLAGS="-D warnings"
ENV RUST_BACKTRACE="1"
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt update && apt install -y build-essential curl gcc-arm-none-eabi
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
RUN rustup target add thumbv6m-none-eabi
RUN rustup component add llvm-tools-preview
RUN cargo install cargo-binutils uf2conv

RUN mkdir /build
COPY build.rs Cargo.lock Cargo.toml entrypoint.sh memory.x /build/
COPY .cargo /build/.cargo
COPY src /build/src

COPY entrypoint.sh /
ENTRYPOINT ["/entrypoint.sh"]
WORKDIR /build
# .cargo/config not working tip from: https://www.natarajmb.com/2021/02/cross-compiling-rust-using-docker/
ENV RUSTFLAGS="-D warnings -C linker=arm-none-eabi-ld -C link-arg=--nmagic -C link-arg=-Tlink.x -C link-arg=-Tdefmt.x -Z trap-unreachable=no -C inline-threshold=5 -C no-vectorize-loops"
