FROM ubuntu:bionic
MAINTAINER MLSEDevOps

RUN apt-get update && apt-get install --no-install-recommends -y

# Prerequiste
RUN apt install -y musl-tools
ENV DEPLOY=1
ENV TARGET=x86_64-unknown-linux-musl
ENV OPENSSL_DIR=$HOME/openssl-musl

# Install sccache
RUN apt-get install -y curl libssl-dev pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ./rustup-init.sh
RUN sh ./rustup-init.sh -y --default-toolchain 1.21.0
RUN . ${HOME}/.cargo/env
RUN /root/.cargo/bin/rustc --version
ENV PATH=/root/.cargo/bin:$PATH
COPY .  /root/sccache

WORKDIR /root/sccache
RUN bash ./scripts/travis-musl-openssl.sh
RUN /root/.cargo/bin/cargo build --features=redis --target $TARGET --release

