FROM rust:latest

WORKDIR /app

COPY . .


WORKDIR /app/frontend
RUN rustup default && rustup target add wasm32-unknown-unknown && cargo install trunk 

RUN cargo build --target wasm32-unknown-unknown

CMD [ "trunk", "serve", "--address", "0.0.0.0"]