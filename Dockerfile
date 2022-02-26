# # Builder stage
# FROM rust:1.59.0 AS builder

# WORKDIR /app
# COPY . .
# ENV SQLX_OFFLINE true
# RUN cargo build -p auth --release

# # Runtime stage
# FROM rust:1.56.0 AS runtime
# RUN cargo install cargo-watch
# WORKDIR /app
# # Copy the compiled binary from the builder env to our runtime
# COPY --from=builder /app/target/release/auth auth
# # COPY --from=builder /app/target/release/auth auth
# EXPOSE 8000
# # We need the configuration file at runtime
# # ENTRYPOINT [ "./auth" ]
# # ENTRYPOINT [ "cargo watch -x "run -p auth"" ]


FROM rust:1.59.0
# RUN apk update && apk add bash && apk add yarn
# RUN apk add git
RUN rustup override set nightly
RUN mkdir /app
WORKDIR /app
COPY . /app
# ENV DATABASE_URL DATABASE_URL
# ENV SECRET_KEY SECRET_KEY
# ENV ROCKET_DATABASES ROCKET_DATABASES
EXPOSE 8000
CMD [ "cargo", "run" ]