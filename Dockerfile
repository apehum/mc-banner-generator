FROM node:20-slim AS web

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

ENV VITE_API_BASE_URL="/api"

WORKDIR /app/web

COPY ./web/package.json ./web/pnpm-lock.yaml* ./
RUN pnpm install

COPY . /app
RUN pnpm run build

FROM rust:1.92-bullseye AS server

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm -f ./target/release/deps/banner_generator*
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=server /app/target/release/banner-generator .
RUN mkdir web
COPY --from=web /app/web/dist ./web/dist
COPY ./patterns ./patterns

CMD ["./banner-generator"]
