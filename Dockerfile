FROM docker.io/library/rust:1.61.0-slim as builder

WORKDIR /app
# Create blank project
# We want dependencies cached, so copy those first.
RUN apt update
RUN apt install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
COPY  Cargo.toml Cargo.lock redirect.template kurzlink.yml ./
COPY  src ./src/
RUN ls
## Install target platform (Cross-Compilation) --> Needed for Alpine
# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release
################
##### Runtime
FROM busybox AS runtime
# Copy application binary from builder image
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/kurzlink /bin/kurzlink
COPY --from=builder /app/redirect.template /
COPY --from=builder /app/kurzlink.yml /




