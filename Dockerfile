FROM ekidd/rust-musl-builder as builder

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM scratch

WORKDIR /app

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/cuckoo /app/cuckoo

ENTRYPOINT ["/app/cuckoo"]