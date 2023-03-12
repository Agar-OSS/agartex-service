FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

# Force crates.io init for better docker caching
RUN cargo search --limit 0

COPY . .
RUN cargo build --release



FROM debian:10.13-slim as environment
WORKDIR /app

RUN useradd user
USER user

COPY --from=builder /app/src/target/release/agartex-service .

ENTRYPOINT [ "./agartex-service" ]
