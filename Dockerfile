FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y binutils grub-pc-bin nasm xorriso qemu make

WORKDIR /workspace

COPY . /workspace

CMD ["make", "iso"]
