FROM rust:1-slim-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/rust-api-template /usr/local/bin/rust-api-template
ENV RUST_LOG=rust_api_template=info,tower_http=info
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rust-api-template"]
