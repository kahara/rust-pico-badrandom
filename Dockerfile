FROM ubuntu:20.04

ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTFLAGS="-D warnings"
ENV RUST_BACKTRACE="1"
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt update && apt install -y build-essential curl gcc-arm-none-eabi
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN rustup target add thumbv6m-none-eabi
RUN rustup component add llvm-tools-preview
RUN cargo install uf2conv cargo-binutils

COPY entrypoint.sh /
ENTRYPOINT ["/entrypoint.sh"]
