FROM rust:1.82-slim as builder

RUN apt update && \
    apt install -y build-essential && \
    apt clean

WORKDIR /app
COPY . /app

RUN cargo build --release --all-features

FROM rust:1.82-slim as runner

RUN  apt update && \
     apt install -y libpq-dev && \
     cargo install diesel_cli --no-default-features --features postgres && \
     apt clean

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/inception-axum /app/inception-axum

EXPOSE 8000
ENTRYPOINT /app/inception-axum