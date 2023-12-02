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


FROM debian:bullseye-slim AS final
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=builder /app/target/release/tokenspan_api /usr/local/bin

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
ENTRYPOINT ["/usr/local/bin/tokenspan_api"]
