FROM rust:1.86-slim as builder

RUN apt update && \
    apt install -y libpq-dev build-essential && \
    apt clean

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM rust:1.86-slim as runner

RUN apt update && \
    apt install -y libpq-dev && \
    apt clean

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/inception-axum /app/
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/diesel.toml /app/

RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8000
ENTRYPOINT /app/inception-axum
