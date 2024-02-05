FROM docker.io/library/rust:alpine as builder

WORKDIR /app
RUN apk upgrade --update-cache --available && \
    apk add openssl g++
COPY . .
RUN cargo build --release

################
##### Runtime
FROM alpine
WORKDIR /app
# Copy application binary from builder image
COPY --from=builder /app/target/release/kurzlink /bin/kurzlink

ENTRYPOINT ["/bin/kurzlink"]
