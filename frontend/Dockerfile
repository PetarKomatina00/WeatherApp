FROM rust:1.90-slim-bookworm AS builder

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown && cargo install trunk --version 0.21.14 --locked

COPY . . 

COPY ../shared /app/shared

WORKDIR /app/frontend

RUN trunk build --release

FROM nginx:alpine

COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 8001

CMD ["nginx", "-g", "daemon off;"]
