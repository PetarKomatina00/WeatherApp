FROM rust:1.90-slim-bookworm AS base

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown && cargo install trunk --version 0.21.14 --locked

COPY frontend/ ./frontend
COPY shared ./shared

FROM base as development

WORKDIR /app/frontend

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8001"]

FROM base as builder

WORKDIR /app/frontend

RUN trunk build --release

FROM nginx:alpine AS production

COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
