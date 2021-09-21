FROM rust:1.55 as builder
WORKDIR /src
COPY . .
RUN cargo build --release
COPY ./assets/ ./target/release/assets/
WORKDIR /src/target/release
RUN rm -rf ./build && rm -rf ./deps && rm -rf ./examples
WORKDIR /src

FROM debian:buster-slim
WORKDIR /root
RUN apt-get update && apt-get install -y apt-transport-https wget curl gnupg && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /src/target/release/ .

ENTRYPOINT [ "taiga-bot-rs" ]