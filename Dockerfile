FROM rust:1.82-alpine as builder

RUN apk update && \
    apk install -y libpq-dev build-essential && \
    apk clean

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM rust:1.82-alpine as runner

RUN apk update && \
    apk install -y libpq-dev && \
    apk clean

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/inception-axum /app/
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/diesel.toml /app/

RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8000
ENTRYPOINT /app/inception-axum
