FROM rust:1.91-slim-bullseye AS base

RUN apt-get update && apt-get install -y libpq-dev libssl-dev pkg-config build-essential

WORKDIR /app

COPY backend/ ./backend
COPY shared/ ./shared

FROM base as development

WORKDIR /app/backend

RUN cargo build 

CMD ["cargo", "run"]

FROM base AS builder

WORKDIR /app/backend

RUN cargo build --release

FROM debian:bullseye-slim as production

WORKDIR /app

COPY --from=builder /app/backend/target/release/backend /app/backend


EXPOSE 8000

CMD ["./backend"] 