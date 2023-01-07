FROM docker.io/library/rust:1.61.0-alpine as builder

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
COPY redirect.template /etc/kurzlink/redirect.template

ENTRYPOINT ["/bin/kurzlink"]
