FROM rust:latest

<<<<<<< Updated upstream
WORKDIR /app/

COPY . .

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch 

CMD ["cargo", "watch", "--why", "--", "echo"]
=======
WORKDIR /app

COPY . . 

COPY ../shared /app/shared

WORKDIR /app/backend

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres && cargo install cargo-watch

CMD [ "cargo", "watch", "--why", "--", "echo", "Change detected"]
>>>>>>> Stashed changes
