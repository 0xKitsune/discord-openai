FROM rust:1.67 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/discord-openai ./target/release/discord-openai

ENTRYPOINT ["/target/release/discord-openai"]