FROM rust AS builder_amd64

RUN apt-get update && apt-get install -y musl-tools musl-dev protobuf-compiler
RUN rustup target add x86_64-unknown-linux-musl && rustup component add clippy

WORKDIR /usr/src/mc_missile_guidance

# cache dependencies
COPY ./Cargo.toml ./Cargo.toml
RUN echo 'fn main() {}' > dummy.rs && \
    sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo clippy -- -D warnings && \
    cargo test && \
    cargo build --release --target x86_64-unknown-linux-musl

# actually build
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./src ./src
COPY ./proto ./proto
RUN cargo clippy -- -D warnings && \
    cargo test && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    cp /usr/src/mc_missile_guidance/target/x86_64-unknown-linux-musl/release/mc_missile_guidance /var/run/mc_missile_guidance

FROM builder_amd64 AS builder

FROM alpine

COPY --from=builder /var/run/mc_missile_guidance /var/run/mc_missile_guidance
WORKDIR /var/run
ENTRYPOINT ["/var/run/mc_missile_guidance"]

LABEL org.opencontainers.image.source=https://github.com/christopher-besch/mc_missile_guidance
