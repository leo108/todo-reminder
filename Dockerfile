# syntax=docker/dockerfile:1.3

FROM rust:alpine as builder

ARG TARGETARCH
# Set RUST_TARGET based on architecture
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      echo "aarch64-unknown-linux-musl" > /tmp/rust_target; \
    else \
      echo "x86_64-unknown-linux-musl" > /tmp/rust_target; \
    fi

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/todo-reminder

COPY . .

RUN RUST_TARGET=$(cat /tmp/rust_target) && \
    rustup target add $RUST_TARGET && \
    cargo build --release --target $RUST_TARGET && \
    cp target/$RUST_TARGET/release/todo-reminder /tmp/

FROM alpine:latest

COPY --from=builder /tmp/todo-reminder /usr/local/bin/todo-reminder

ENTRYPOINT ["/usr/local/bin/todo-reminder"]
