# Set up build container
FROM rust:1.43.0 as builder
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup component add rustfmt
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app

# Do the fake build
COPY server/Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() { println!(\"if you see this, the build broke\"); }" > src/main.rs
COPY server/build.rs .
COPY proto proto
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/server*

# Do the real build
COPY server/src src
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# Set up run container
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server .
CMD ["./server"]
