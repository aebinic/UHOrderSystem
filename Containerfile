FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/UHOrderSystem ./UHOrderSystem
EXPOSE 3000
CMD ["./UHOrderSystem"]
