# Build stage
FROM rust:1.79.0-slim-buster as builder

RUN apt update && apt install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    make \
    gcc

RUN mkdir -p /app/backend

COPY alns/Cargo.toml /app/backend/
COPY alns/Cargo.lock /app/backend/
WORKDIR /app/backend

RUN cargo fetch

COPY alns /app/backend/
RUN cargo build --release --package alns

FROM debian:buster-slim

COPY --from=builder /app/backend/target/release/backend /app/backend/alns


WORKDIR /app/backend/
EXPOSE 3000
ENV RUST_LOG="info"
ENV PORT="3000"

CMD ["./alns"]

# docker build --platform linux/amd64 .