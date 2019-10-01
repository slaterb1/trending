############################
# Build container
############################
FROM rust:1.37

RUN apt-get update && \
apt-get install musl-tools libssl-dev -y && \
rustup target add x86_64-unknown-linux-musl

COPY ./ /ops 

WORKDIR /ops

RUN RUSTFLAG=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl && \
strip /ops/target/x86_64-unknown-linux-musl/release/rustydata
