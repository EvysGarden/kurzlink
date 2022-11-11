FROM rust:1.61.0-slim as builder

WORKDIR /app
# Create blank project
# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock src/
## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release
################
##### Runtime
FROM busybox AS runtime
# Copy application binary from builder image
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/kurzlink /usr/local/bin



