FROM rust:1.91-slim-bullseye AS builder

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config build-essential

WORKDIR /app

COPY backend/ ./backend
COPY shared/ ./shared

WORKDIR /app/backend

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends libpq5 libssl-dev ca-certificates build-essential postgresql-client && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/backend//target/release/backend/ /app/

COPY backend/.env /app/.env

COPY backend/Rocket.toml /app/Rocket.toml

COPY backend/diesel.toml /app/diesel.toml

COPY backend/migrations /app/migrations
EXPOSE 8000

CMD ["./backend"] 