FROM rust:1.69.0-buster as base-avida
COPY . /app
WORKDIR /app/ursa-service
RUN  cargo build --release
CMD ./target/release/ursa-service

# FROM rust:1.67.1-slim-bookworm
# COPY --from=base-avida
