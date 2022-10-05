FROM clux/muslrust:stable AS builder
COPY . .
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release --bin ttt-server && \
    mv /volume/target/x86_64-unknown-linux-musl/release/ttt-server .

FROM gcr.io/distroless/static:nonroot
COPY --from=builder /volume/ttt-server /app/

ENTRYPOINT ["/app/ttt-server"]