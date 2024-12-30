FROM rust:1.82-slim as builder

RUN apt update && \
#    apt install -y pkg-config build-essential && \
    apt clean

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM rust:1.82-slim as runner

RUN  cargo install diesel_cli --no-default-features --features postgres

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/inception-axum /app/inception-axum

EXPOSE 8000
ENTRYPOINT /app/inception-axum