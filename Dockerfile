FROM lukemathwalker/cargo-chef:latest-rust-1.83.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# up to this point, if our dependency tree remains the same
# all layers should be cached
COPY . .
ENV SQLX_OFFLINE true
# Build project
RUN cargo build --release --bin zero2prod


# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSl - it is dynamically linker by some of our dependencies
# Install ca-certificates - they are needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/zero2prod"]
