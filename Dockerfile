# https://github.com/zupzup/rust-docker-web/blob/main/debian/Dockerfile
FROM rust:1.59.0 AS builder
ENV SQLX_OFFLINE true
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# -------------------------------------

# FROM base AS builder

RUN USER=root cargo new --bin play_auth
WORKDIR /play_auth
COPY . ./

RUN cargo build --release -p auth \
    && rm src/*.rs target/release/deps/auth*

# RUN cargo build --release -p auth

FROM debian:buster-slim AS debian
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=app_user

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# FROM builder AS prod

FROM debian AS prod
# COPY --from=builder /play_auth/target/release/auth ${APP}/auth
COPY --from=builder /play_auth/target/release/auth auth
COPY configuration configuration


USER $APP_USER
# WORKDIR ${APP}

CMD [ "./auth" ]

# # -------------------------------------

# FROM base as dev
# RUN cargo install cargo-watch
# ENV SQLX_OFFLINE true
# WORKDIR /usr/src/app

# VOLUME [ "/usr/src/app/cargo" ]
# CMD ["cargo", "watch", "-x", "run", "-p", "auth"]
