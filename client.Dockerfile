FROM rustlang/rust:nightly as builder
WORKDIR build
COPY . .
RUN cargo build --release --bin live_wait_client

FROM gcr.io/distroless/cc
COPY --from=builder /build/target/release/live_wait_client ./
ENTRYPOINT ["./live_wait_client"]
