FROM rust:latest

WORKDIR /app/

COPY . .

WORKDIR /app/backend

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres && cargo install cargo-watch

CMD ["cargo", "watch", "--why", "--", "echo"]