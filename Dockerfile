FROM rust:1.82.0-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install musl-tools -y && rustup target add x86_64-unknown-linux-musl
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# COMPILED IMAGE
FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/wilayah-service /wilayah-service

EXPOSE 8080
CMD ["/wilayah-service"]
