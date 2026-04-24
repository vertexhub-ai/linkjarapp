FROM rust:1.78-slim AS builder

WORKDIR /app

# native-tls requires OpenSSL headers at build time
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependency layer — only rebuilt when Cargo.toml/Cargo.lock change
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN SQLX_OFFLINE=true cargo build --release
RUN rm -f target/release/deps/linkjar_api*

# Build real binary
COPY src ./src
RUN SQLX_OFFLINE=true cargo build --release


FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/linkjar-api ./

# DATABASE_URL must be injected at runtime (e.g. -e DATABASE_URL=postgres://...)
ENV DATABASE_URL=""

EXPOSE 8080

CMD ["./linkjar-api"]
