FROM rust:1.97.0-slim-bookworm AS builder

WORKDIR /build
COPY . .

RUN rustup target add x86_64-unknown-linux-musl \
  && apt-get update \
  && apt-get install -y --no-install-recommends musl-tools \
  && rm -rf /var/lib/apt/lists/*

RUN cargo build --release -p activator --target x86_64-unknown-linux-musl

FROM alpine:3.23.5

RUN apk add --no-cache ca-certificates

RUN addgroup -S igl-activator && adduser -S igl-activator -G igl-activator
USER igl-activator
WORKDIR /home/igl-activator

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/activator /usr/local/bin/igl-activator

ENV ACTIVATOR_CONFIG_DIR=/home/igl-activator/config

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/igl-activator"]
