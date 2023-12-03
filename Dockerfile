FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
ENV RUST_BACKTRACE=1
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
# Build application
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin tokenspan_api


FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY ./tokenspan-api/config ./tokenspan-api/config
COPY --from=builder /app/target/release/tokenspan_api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/tokenspan_api"]