# Builder stage
FROM rust:1.59.0 AS builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM rust:1.56.0 AS runtime

WORKDIR /app
# Copy the compiled binary from the builder env to our runtime
COPY --from=builder /app/target/release/auth auth
EXPOSE 8000
# We need the configuration file at runtime
ENTRYPOINT [ "cargo", "run" ]