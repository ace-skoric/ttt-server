FROM rust:1.65-alpine3.16 AS builder
WORKDIR /app
COPY . .
ENV CARGO_TARGET_DIR /volume/target
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=$CARGO_HOME/registry \
    cargo build --release --bin ttt-server && \
    mv /volume/target/release/ttt-server .

FROM gcr.io/distroless/static:nonroot
COPY --from=builder /app/ttt-server /app/

ENTRYPOINT ["/app/ttt-server"]