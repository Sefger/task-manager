FROM rust:1.87.0 as builder

WORKDIR /app

# Копируем только необходимое для сборки зависимостей
COPY Cargo.toml .
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Копируем реальный исходный код
COPY src ./src

# Собираем финальную версию
RUN touch src/main.rs && \
    cargo build --release

# Финальный образ
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3
COPY --from=builder /app/target/release/task-manager /usr/local/bin/
CMD ["task-manager"]