FROM rust:1.46 as builder
WORKDIR build
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY client/ client/
COPY --from=builder /build/target/release/live_wait_server ./
COPY --from=builder /build/Rocket.toml ./
EXPOSE 8080
ENTRYPOINT ["./live_wait_server"]
