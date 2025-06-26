FROM rust:1.87 as builder

WORKDIR /app

# Кэширование зависимостей
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Установка sqlx-cli
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Копируем исходный код
COPY . .

# Собираем приложение
RUN cargo build --release && \
    cp target/release/task-manager /usr/local/bin/ && \
    cp /usr/local/cargo/bin/sqlx /usr/local/bin/

# Финальный образ
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    libpq5 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем бинарники и ресурсы
COPY --from=builder /usr/local/bin/task-manager /usr/local/bin/
COPY --from=builder /usr/local/bin/sqlx /usr/local/bin/
COPY --from=builder /app/migrations ./migrations
COPY docker-entrypoint.sh ./
COPY config ./config

RUN chmod +x /app/docker-entrypoint.sh

ENV RUST_LOG=info
ENV CONFIG_DIR=/app/config

EXPOSE 8000

ENTRYPOINT ["/app/docker-entrypoint.sh"]