FROM rust:latest

WORKDIR /app/frontend 

COPY frontend/ . 

RUN rustup default && rustup target add wasm32-unknown-unknown && cargo install trunk 

RUN cargo build --target wasm32-unknown-unknown

EXPOSE 8001 

CMD [ "trunk", "serve", "--address", "0.0.0.0", "--port", "8001" ]