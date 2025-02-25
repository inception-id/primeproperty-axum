# FROM rust:1.82-slim as builder

# RUN apt update && \
#     apt install -y libpq-dev build-essential && \
#     apt clean

# WORKDIR /app
# COPY . /app

# RUN cargo build --release --all-features

# FROM rust:1.82-slim as runner

# RUN apt update && \
#     apt install -y libpq-dev && \
#     apt clean

# # Copy the build artifact from the builder stage
# COPY --from=builder /app/target/release/inception-axum /app/
# COPY --from=builder /app/migrations /app/migrations
# COPY --from=builder /app/diesel.toml /app/

# RUN cargo install diesel_cli --no-default-features --features postgres

# EXPOSE 8000
# ENTRYPOINT /app/inception-axum
# Build Stage
FROM rust:1.82-slim AS builder

RUN apt update && \
    apt install -y libpq-dev build-essential && \
    apt clean

WORKDIR /app
COPY . .

RUN cargo build --release --all-features

# Runtime Stage
FROM debian:bookworm-slim AS runner

RUN apt update && \
    apt install -y libpq-dev curl && \
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh && \
    apt remove -y curl && \
    apt autoremove -y && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/inception-axum /app/
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/diesel.toml /app/

# Install only the diesel CLI binary instead of using Cargo

EXPOSE 8000
ENTRYPOINT ["/app/inception-axum"]
