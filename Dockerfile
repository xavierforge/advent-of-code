FROM rust:1.89-slim-bookworm as chef
WORKDIR /app

COPY . .

RUN cargo build --release -p common

FROM debian:bookworm-slim
WORKDIR /app
CMD ["echo", "Please specify a binary to run"]