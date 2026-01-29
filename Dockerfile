# Stage 1: Build
FROM rust:alpine AS builder
RUN apk add --no-cache build-base

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates tzdata

COPY cert.pem /etc/ssl/certs/cert.pem
COPY key.pem /etc/ssl/certs/key.pem

COPY --from=builder /app/target/release/myapp /usr/local/bin/myapp

ENTRYPOINT ["myapp"]