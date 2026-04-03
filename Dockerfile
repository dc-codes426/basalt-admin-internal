FROM rust:1.94-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY generated ./generated
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

RUN adduser --disabled-password --no-create-home appuser

COPY --from=builder /app/target/release/basalt-admin-internal /usr/local/bin/basalt-admin-internal

USER appuser

EXPOSE 3000

CMD ["basalt-admin-internal"]
