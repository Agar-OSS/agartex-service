FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

# Force crates.io init for better docker caching
RUN cargo search --limit 0

COPY . .
RUN cargo build --release



FROM debian:10.13-slim as environment
WORKDIR /app

# Setup latex
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y texlive-full && \
    rm -rf /var/lib/apt/lists/*
RUN chmod 777 .

RUN useradd user
USER user

COPY --from=builder /app/src/target/release/agartex-service .

# Test
RUN mkdir tex
COPY example.tex tex/example.tex
RUN cd tex && \
    pdflatex example.tex && \
    cd ..

ENTRYPOINT [ "./agartex-service" ]
