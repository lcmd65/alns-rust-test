FROM rust:1.79.0-slim-buster AS builder

RUN apt update && apt install -y \
    build-essential \
    libssl-dev \
    libclang-dev \
    libc6-dev \
    clang \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src/ ./src/
RUN cargo build --release

FROM debian:buster-slim

RUN apt update && apt install -y \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/alns /usr/local/bin/alns

EXPOSE 8080

ENV RUST_LOG="info"
ENV PORT="8080"

CMD ["alns"]


# docker build --platform linux/amd64 --progress=plain -t rust-backend .

# docker build --platform linux/amd64 -t rust-backend
# docker run -p 8080:8080 rust-backend