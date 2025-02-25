FROM rust:1.82-slim as builder

RUN apt update && \
    apt install -y libpq-dev build-essential && \
    apt clean

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM debian:bookworm-slim as runner

RUN apt update && \
    apt install -y libpq-dev curl && \
    curl --proto -L '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh \
    | tar -xz -C /usr/local/bin && \
    chmod +x /usr/local/bin/diesel && \
    apt remove -y curl && \
    apt autoremove -y && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/inception-axum /app/
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/diesel.toml /app/

RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8000
ENTRYPOINT /app/inception-axum
