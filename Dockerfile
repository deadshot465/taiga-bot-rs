FROM rust:1.88 as builder
WORKDIR /src
COPY . .
RUN cargo build --release
COPY ./assets/ ./target/release/assets/
WORKDIR /src/target/release
RUN rm -rf ./build && rm -rf ./deps && rm -rf ./examples && rm -rf ./incremental
WORKDIR /src

FROM debian:bookworm-slim
WORKDIR /root
RUN apt-get update && apt-get install -y apt-transport-https wget curl gnupg openssl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /src/target/release/ .

ENTRYPOINT [ "/app/taiga-bot-rs" ]
