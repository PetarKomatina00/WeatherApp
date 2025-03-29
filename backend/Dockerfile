FROM rust:latest

WORKDIR /app/backend

COPY backend/ . 

RUN rustup default

RUN cargo install diesel_cli --no-default-features --feature postgres

EXPOSE 8000

CMD ["cargo", "run"]