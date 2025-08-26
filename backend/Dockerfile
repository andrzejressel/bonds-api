FROM rust:1.89.0-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:trixie-20250811-slim

# Required for new relic
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/myapp-cli myapp-cli
COPY --from=builder /usr/src/assets assets

ENTRYPOINT ["/usr/app/myapp-cli", "start", "-e", "production"]
