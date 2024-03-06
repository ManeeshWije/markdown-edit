# FROM rust:latest
#
# ARG DATABASE_URL
# ARG GOOGLE_CLIENT_ID
# ARG GOOGLE_CLIENT_SECRET
# ARG BASE_URL
# ARG CLIENT_URL
#
# ENV DATABASE_URL=$DATABASE_URL
# ENV GOOGLE_CLIENT_ID=$GOOGLE_CLIENT_ID
# ENV GOOGLE_CLIENT_SECRET=$GOOGLE_CLIENT_SECRET
# ENV BASE_URL=$BASE_URL
# ENV CLIENT_URL=$CLIENT_URL
#
# WORKDIR /usr/src/myapp
#
# COPY . .
#
# RUN cargo install --path .
# RUN cargo install cargo-watch
#
# EXPOSE 3001
#
# CMD ["backend"]
FROM lukemathwalker/cargo-chef:latest as chef

ARG DATABASE_URL
ARG GOOGLE_CLIENT_ID
ARG GOOGLE_CLIENT_SECRET
ARG BASE_URL
ARG CLIENT_URL

ENV DATABASE_URL=$DATABASE_URL
ENV GOOGLE_CLIENT_ID=$GOOGLE_CLIENT_ID
ENV GOOGLE_CLIENT_SECRET=$GOOGLE_CLIENT_SECRET
ENV BASE_URL=$BASE_URL
ENV CLIENT_URL=$CLIENT_URL

WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/backend ./app

FROM debian:latest AS runtime
WORKDIR /app
# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /app/app /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/app"]
