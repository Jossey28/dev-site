# Building the binary

FROM rust:1.92.0 AS builder

WORKDIR /app

ARG GIT_COMMIT_SHA_ARG="unknown"
ENV GIT_COMMIT_SHA_DOCKER=${GIT_COMMIT_SHA_ARG}

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked

COPY . .

RUN cargo leptos build --release

# Running it within Docker

FROM debian:bookworm-slim

RUN apt-get update -y
RUN apt-get install ca-certificates -y --no-install-recommends 
RUN rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/dev-site ./dev-site
COPY --from=builder /app/target/site ./site

ENV LEPTOS_SITE_ROOT=site
ENV RUST_LOG=info

EXPOSE 8080

CMD [ "./dev-site" ]