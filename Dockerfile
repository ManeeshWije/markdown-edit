FROM node:latest AS frontend-builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/package-lock.json ./

RUN npm install

COPY frontend .

RUN npm run build

FROM rust:latest AS backend-builder

WORKDIR /app/backend

COPY backend/Cargo.toml backend/Cargo.lock ./

# Create a dummy project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY backend .

RUN cargo install sqlx-cli --no-default-features --features postgres
RUN sqlx database create
RUN sqlx migrate run

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

COPY --from=frontend-builder /app/frontend/dist ./frontend
COPY --from=backend-builder /app/backend/target/release/backend ./backend

EXPOSE 8080

CMD ["./backend"]
