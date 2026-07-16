FROM rust:1.97.0-slim-bookworm AS builder

ARG TARGETARCH

WORKDIR /build
COPY . .

RUN case "$TARGETARCH" in \
  amd64) RUST_TARGET=x86_64-unknown-linux-musl ;; \
  arm64) RUST_TARGET=aarch64-unknown-linux-musl ;; \
  *) echo "Unsupported architecture: $TARGETARCH" && exit 1 ;; \
  esac \
  && rustup target add "$RUST_TARGET" \
  && apt-get update \
  && apt-get install -y --no-install-recommends musl-tools \
  && rm -rf /var/lib/apt/lists/* \
  && cargo build --release -p activator --target "$RUST_TARGET" \
  && cp "target/$RUST_TARGET/release/activator" /activator

FROM alpine:3.23.5

RUN apk add --no-cache ca-certificates

RUN addgroup -S igl-activator \
  && adduser -S igl-activator -G igl-activator

USER igl-activator
WORKDIR /home/igl-activator

COPY --from=builder /activator /usr/local/bin/igl-activator

ENV ACTIVATOR_CONFIG_DIR=/home/igl-activator/config

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/igl-activator"]
