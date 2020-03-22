FROM rustlang/rust:nightly
WORKDIR app
COPY . .
RUN cargo build --release
EXPOSE 8080
ENTRYPOINT ["./target/release/live_wait_server"]
