FROM debian:10.13-slim as environment

# Setup latex
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y texlive-full && \
    rm -rf /var/lib/apt/lists/*
