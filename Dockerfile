FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y binutils grub-pc-bin nasm xorriso qemu make curl build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup override set nightly \
    && rustup component add rust-src

ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /workspace
COPY . /workspace

CMD ["make", "iso"]
