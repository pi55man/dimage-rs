FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl
FROM alpine:latest
RUN apk add --no-cache ca-certificates
EXPOSE 3200
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/dimage-rs usr/local/bin/app
CMD ["app"]
