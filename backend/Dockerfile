FROM rust:1.89.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-20250811-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/myapp-cli myapp-cli
COPY --from=builder /usr/src/assets assets

ENTRYPOINT ["/usr/app/myapp-cli", "start", "-e", "production"]
