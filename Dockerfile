FROM rust:1.68-slim-buster as builder
WORKDIR /app/src

# Force crates.io init for better docker caching
RUN cargo search --limit 0

COPY . .
RUN cargo build --release



FROM agaross.azurecr.io/agar-oss/latex-base:latest as environment

WORKDIR /app
RUN chmod 777 .

RUN useradd user
USER user

COPY --from=builder /app/src/target/release/agartex-service .

# Test
RUN mkdir tex
COPY example.tex tex/example.tex
RUN cd tex && \
    latexmk -pdf example.tex && \
    cd ..

EXPOSE 3000
ENTRYPOINT [ "./agartex-service" ]
