FROM rust:latest

WORKDIR /app

COPY . . 

COPY ../shared /app/shared

WORKDIR /app/frontend

RUN rustup default && rustup target add wasm32-unknown-unknown && cargo install trunk 

ENV TRUNK_WATCH_USE_POLLING=true
CMD ["trunk", "serve","--address", "0.0.0.0"]