FROM docker.io/library/rust:1.91-alpine
WORKDIR /app

RUN apk add musl-dev build-base

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo fetch
RUN rm -rf /app
