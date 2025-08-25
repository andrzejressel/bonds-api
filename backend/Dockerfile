FROM rust:1.87.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/myapp-cli myapp-cli

ENTRYPOINT ["/usr/app/myapp-cli", "start"]
