FROM rust:latest

WORKDIR /usr/src/myapp

COPY . .

RUN cargo clean && cargo install --path .

EXPOSE 3001

ENTRYPOINT ["backend"]
