############################
# Build container
############################
FROM rust:1.38 AS dep

WORKDIR /ops

COPY . .

RUN cargo build --release

############################
# Final container
############################
FROM debian:10-slim

WORKDIR /ops

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=dep /ops/target/release/trending /bin
