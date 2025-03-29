FROM rust:latest

WORKDIR /app/

COPY . . 

RUN rustup default && rustup target add wasm32-unknown-unknown

RUN cargo install diesel_cli --no-default-features --features postgres && cargo install cargo-watch

WORKDIR /app/frontend
RUN cargo build --target wasm32-unknown-unknown 
RUN cargo install trunk && trunk build

VOLUME [ "/app/shared" ]
VOLUME ["/app/target"]

CMD ["cargo", "watch", "--why", "--", "echo"]